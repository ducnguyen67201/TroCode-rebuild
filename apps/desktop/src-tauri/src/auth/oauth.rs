use super::OAuthExchange;
use crate::worker::WorkerError;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, time::Duration};
use tauri_plugin_opener::OpenerExt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::Instant,
};
use url::Url;

const CALLBACK_PATH: &str = "/oauth2/callback";
const MAX_CALLBACK_BYTES: usize = 8192;

pub async fn authorize(
    app: &tauri::AppHandle,
    client_id: &str,
) -> Result<OAuthExchange, WorkerError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Unable to start secure sign-in."))?;
    let port = listener
        .local_addr()
        .map_err(|_| WorkerError::new("NOT_READY", "Unable to start secure sign-in."))?
        .port();
    let redirect_uri = format!("http://127.0.0.1:{port}{CALLBACK_PATH}");
    let state = random_url_token(32);
    let nonce = random_url_token(32);
    let code_verifier = random_url_token(64);
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier.as_bytes()));
    let mut authorization = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
        .expect("fixed Google authorization URL");
    authorization
        .query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "openid email profile")
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", &state)
        .append_pair("nonce", &nonce);
    app.opener()
        .open_url(authorization.as_str(), None::<&str>)
        .map_err(|_| WorkerError::new("NOT_READY", "Unable to open the system browser."))?;

    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(WorkerError::new(
                "AUTH_TIMEOUT",
                "Google sign-in timed out. Try again.",
            ));
        }
        let accepted = tokio::time::timeout(remaining, listener.accept())
            .await
            .map_err(|_| WorkerError::new("AUTH_TIMEOUT", "Google sign-in timed out. Try again."))?
            .map_err(|_| WorkerError::new("NOT_READY", "Unable to receive Google sign-in."))?;
        if !accepted.1.ip().is_loopback() {
            continue;
        }
        let mut stream = accepted.0;
        let callback_remaining = deadline.saturating_duration_since(Instant::now());
        let callback = tokio::time::timeout(
            callback_remaining,
            receive_callback(&mut stream, port, &state),
        )
        .await;
        match callback {
            Err(_) => {
                write_completion(&mut stream, false).await;
                return Err(WorkerError::new(
                    "AUTH_TIMEOUT",
                    "Google sign-in timed out. Try again.",
                ));
            }
            Ok(Ok(Some(code))) => {
                write_completion(&mut stream, true).await;
                return Ok(OAuthExchange {
                    code,
                    code_verifier,
                    redirect_uri,
                    nonce,
                });
            }
            Ok(Ok(None)) => continue,
            Ok(Err(error)) => {
                write_completion(&mut stream, false).await;
                return Err(error);
            }
        }
    }
}

async fn receive_callback(
    stream: &mut TcpStream,
    port: u16,
    expected_state: &str,
) -> Result<Option<String>, WorkerError> {
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 1024];
    loop {
        let count = stream
            .read(&mut buffer)
            .await
            .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid sign-in callback."))?;
        if count == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if bytes.len() > MAX_CALLBACK_BYTES {
            return Err(WorkerError::new(
                "INVALID_MESSAGE",
                "Sign-in callback exceeds the allowed size.",
            ));
        }
        if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }
    let request = std::str::from_utf8(&bytes)
        .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid sign-in callback."))?;
    parse_callback(request, port, expected_state)
}

fn parse_callback(
    request: &str,
    port: u16,
    expected_state: &str,
) -> Result<Option<String>, WorkerError> {
    let mut lines = request.split("\r\n");
    let request_line = lines
        .next()
        .ok_or_else(|| WorkerError::new("INVALID_MESSAGE", "Invalid sign-in callback."))?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or_default();
    let target = request_parts.next().unwrap_or_default();
    let version = request_parts.next().unwrap_or_default();
    if method != "GET" || version != "HTTP/1.1" || request_parts.next().is_some() {
        return Err(WorkerError::new(
            "INVALID_MESSAGE",
            "Invalid sign-in callback.",
        ));
    }
    if target == "/favicon.ico" {
        return Ok(None);
    }
    let expected_host = format!("127.0.0.1:{port}");
    let host = lines
        .find_map(|line| {
            line.split_once(':')
                .filter(|(name, _)| name.eq_ignore_ascii_case("host"))
                .map(|(_, value)| value.trim())
        })
        .unwrap_or_default();
    if host != expected_host {
        return Err(WorkerError::new(
            "INVALID_MESSAGE",
            "Invalid sign-in callback.",
        ));
    }
    let callback = Url::parse(&format!("http://{expected_host}{target}"))
        .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid sign-in callback."))?;
    if callback.path() != CALLBACK_PATH || callback.fragment().is_some() {
        return Err(WorkerError::new(
            "INVALID_MESSAGE",
            "Invalid sign-in callback.",
        ));
    }
    let mut parameters: HashMap<String, Vec<String>> = HashMap::new();
    for (name, value) in callback.query_pairs() {
        if !matches!(
            name.as_ref(),
            "code"
                | "state"
                | "error"
                | "error_description"
                | "scope"
                | "authuser"
                | "prompt"
                | "hd"
        ) {
            return Err(WorkerError::new(
                "INVALID_MESSAGE",
                "Invalid sign-in callback.",
            ));
        }
        parameters
            .entry(name.into_owned())
            .or_default()
            .push(value.into_owned());
    }
    if parameters.values().any(|values| values.len() != 1) {
        return Err(WorkerError::new(
            "INVALID_MESSAGE",
            "Invalid sign-in callback.",
        ));
    }
    if parameters.contains_key("error") {
        return Err(WorkerError::new(
            "AUTH_CANCELLED",
            "Google sign-in was cancelled.",
        ));
    }
    if parameters
        .get("state")
        .and_then(|values| values.first())
        .map(String::as_str)
        != Some(expected_state)
    {
        return Err(WorkerError::new(
            "UNAUTHORIZED",
            "Google sign-in could not be verified.",
        ));
    }
    let code = parameters
        .get("code")
        .and_then(|values| values.first())
        .filter(|value| !value.is_empty() && value.len() <= 4096)
        .cloned()
        .ok_or_else(|| WorkerError::new("UNAUTHORIZED", "Google sign-in returned no code."))?;
    Ok(Some(code))
}

async fn write_completion(stream: &mut TcpStream, success: bool) {
    let (title, message) = if success {
        ("Sign-in complete", "Return to Tro to continue.")
    } else {
        ("Sign-in stopped", "Return to Tro and try again.")
    };
    let body = format!(
        "<!doctype html><meta charset=utf-8><meta http-equiv=Content-Security-Policy content=\"default-src 'none'; style-src 'unsafe-inline'\"><title>{title}</title><style>body{{font:18px system-ui;background:#f4f3ed;color:#183d34;display:grid;min-height:90vh;place-items:center}}main{{max-width:30rem;padding:3rem}}h1{{font-size:2rem}}</style><main><h1>{title}</h1><p>{message}</p></main>"
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.shutdown().await;
}

fn random_url_token(bytes: usize) -> String {
    let mut value = vec![0_u8; bytes];
    rand::rng().fill_bytes(&mut value);
    URL_SAFE_NO_PAD.encode(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_requires_exact_host_path_and_state() {
        let request =
            "GET /oauth2/callback?code=ok&state=expected HTTP/1.1\r\nHost: 127.0.0.1:51234\r\n\r\n";
        assert_eq!(
            parse_callback(request, 51234, "expected").unwrap(),
            Some("ok".to_owned())
        );
        assert!(parse_callback(request, 51235, "expected").is_err());
        assert!(parse_callback(request, 51234, "wrong").is_err());
    }

    #[test]
    fn pkce_material_is_url_safe_and_unique() {
        let first = random_url_token(64);
        let second = random_url_token(64);
        assert_ne!(first, second);
        assert!((43..=128).contains(&first.len()));
        assert!(
            first
                .bytes()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, b'-' | b'_'))
        );
    }
}
