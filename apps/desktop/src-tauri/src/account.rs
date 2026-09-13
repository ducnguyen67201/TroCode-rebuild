//! Host-private proof credentials. Never serialized to React or forwarded as a provider key.
use crate::worker::WorkerError;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Credentials {
    origin: String,
    token: String,
}
fn credentials() -> Result<Credentials, WorkerError> {
    let home = std::env::var_os(if cfg!(windows) {
        "LOCALAPPDATA"
    } else {
        "HOME"
    })
    .ok_or(WorkerError::new(
        "NOT_READY",
        "Private account configuration unavailable.",
    ))?;
    let path = std::path::PathBuf::from(home).join(".tro-rebuild/proof-account.json");
    let metadata = std::fs::symlink_metadata(&path).map_err(|_| {
        WorkerError::new(
            "NOT_READY",
            "Configure a proof account on this device first.",
        )
    })?;
    if !metadata.is_file() || metadata.len() > 4096 {
        return Err(WorkerError::new(
            "FORBIDDEN",
            "Invalid private account file.",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(WorkerError::new(
                "FORBIDDEN",
                "Private account file must be owner-only.",
            ));
        }
    }
    let credentials: Credentials = serde_json::from_slice(
        &std::fs::read(path)
            .map_err(|_| WorkerError::new("NOT_READY", "Private account file unavailable."))?,
    )
    .map_err(|_| WorkerError::new("NOT_READY", "Private account file is invalid."))?;
    let url = reqwest::Url::parse(&credentials.origin)
        .map_err(|_| WorkerError::new("FORBIDDEN", "Invalid proof origin."))?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.path() != "/"
        || url.query().is_some()
        || url.fragment().is_some()
        || credentials.token.len() != 64
        || !credentials.token.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err(WorkerError::new(
            "FORBIDDEN",
            "Proof configuration requires HTTPS and a scoped account token.",
        ));
    }
    Ok(credentials)
}
async fn request(path: &str, session: Option<&str>) -> Result<(Value, String), WorkerError> {
    let credentials = credentials()?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| WorkerError::new("INTERNAL", "Account client unavailable."))?;
    let url = format!("{}{path}", credentials.origin.trim_end_matches('/'));
    let request = if let Some(session) = session {
        client.post(url).json(&json!({"teachingSessionId":session}))
    } else {
        client.get(url)
    };
    let mut response = request
        .bearer_auth(credentials.token)
        .send()
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Proof account server unavailable."))?;
    if !response.status().is_success() {
        return Err(WorkerError::new(
            "UNAUTHORIZED",
            "Proof account request refused.",
        ));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Account response interrupted."))?
    {
        if body.len() + chunk.len() > 4096 {
            return Err(WorkerError::new(
                "INVALID_MESSAGE",
                "Account response exceeds limit.",
            ));
        }
        body.extend_from_slice(&chunk);
    }
    let value = serde_json::from_slice(&body)
        .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid account response."))?;
    Ok((value, credentials.origin))
}
pub async fn identity() -> Result<String, WorkerError> {
    let (value, _) = request("/v1/me", None).await?;
    value["accountId"]
        .as_str()
        .and_then(|v| uuid::Uuid::parse_str(v).ok())
        .map(|v| v.to_string())
        .ok_or(WorkerError::new(
            "UNAUTHORIZED",
            "Invalid account identity.",
        ))
}
pub async fn model_context(account: &str, session: &str) -> Result<Value, WorkerError> {
    let (value, origin) = request("/v1/runtime-grants", Some(session)).await?;
    if value["accountId"] != account {
        return Err(WorkerError::new(
            "UNAUTHORIZED",
            "Account changed. Reconnect explicitly.",
        ));
    }
    Ok(json!({"origin":origin,"grant":value["grant"],"model":value["model"]}))
}
pub fn storage_root() -> Result<String, WorkerError> {
    let base = std::env::var_os(if cfg!(windows) {
        "LOCALAPPDATA"
    } else {
        "HOME"
    })
    .ok_or(WorkerError::new(
        "NOT_READY",
        "Private storage unavailable.",
    ))?;
    Ok(std::path::PathBuf::from(base)
        .join(".tro-rebuild/evidence")
        .to_string_lossy()
        .into_owned())
}
