use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use url::Url;

const GOOGLE_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_JWKS_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/certs";
const MAX_PROVIDER_BODY_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug)]
pub struct VerifiedGoogleIdentity {
    pub issuer: String,
    pub subject: String,
    pub email: String,
    pub email_normalized: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoogleError {
    Invalid,
    Unavailable,
}

pub struct GoogleVerifier {
    client_id: String,
    token_endpoint: Url,
    jwks_endpoint: Url,
    client: reqwest::Client,
    keys: RwLock<CachedKeys>,
}

#[derive(Default)]
struct CachedKeys {
    expires_at: Option<std::time::Instant>,
    values: HashMap<String, Arc<DecodingKey>>,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    code: &'a str,
    client_id: &'a str,
    code_verifier: &'a str,
    grant_type: &'static str,
    redirect_uri: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    id_token: String,
}

#[derive(Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    alg: String,
    n: String,
    e: String,
}

#[derive(Clone, Debug, Deserialize)]
struct GoogleClaims {
    iss: String,
    #[allow(dead_code)]
    aud: serde_json::Value,
    sub: String,
    email: String,
    email_verified: bool,
    name: Option<String>,
    nonce: String,
    azp: Option<String>,
    iat: u64,
    exp: u64,
}

impl GoogleVerifier {
    pub fn production(client_id: String) -> Result<Self, &'static str> {
        Self::new(
            client_id,
            Url::parse(GOOGLE_TOKEN_ENDPOINT).expect("fixed Google token endpoint"),
            Url::parse(GOOGLE_JWKS_ENDPOINT).expect("fixed Google JWKS endpoint"),
        )
    }

    pub fn new(
        client_id: String,
        token_endpoint: Url,
        jwks_endpoint: Url,
    ) -> Result<Self, &'static str> {
        if client_id.is_empty()
            || !fixed_https_or_loopback(&token_endpoint)
            || !fixed_https_or_loopback(&jwks_endpoint)
        {
            return Err("Invalid Google verifier configuration.");
        }
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "Google client unavailable.")?;
        Ok(Self {
            client_id,
            token_endpoint,
            jwks_endpoint,
            client,
            keys: RwLock::new(CachedKeys::default()),
        })
    }

    pub async fn exchange(
        &self,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
        nonce: &str,
    ) -> Result<VerifiedGoogleIdentity, GoogleError> {
        validate_exchange_input(code, code_verifier, redirect_uri, nonce)?;
        let response = self
            .client
            .post(self.token_endpoint.clone())
            .form(&TokenRequest {
                code,
                client_id: &self.client_id,
                code_verifier,
                grant_type: "authorization_code",
                redirect_uri,
            })
            .send()
            .await
            .map_err(|_| GoogleError::Unavailable)?;
        if !response.status().is_success() {
            return Err(if response.status().is_server_error() {
                GoogleError::Unavailable
            } else {
                GoogleError::Invalid
            });
        }
        let body = bounded_body(response).await?;
        let token: TokenResponse =
            serde_json::from_slice(&body).map_err(|_| GoogleError::Invalid)?;
        if token.id_token.len() > 16 * 1024 {
            return Err(GoogleError::Invalid);
        }
        self.verify_id_token(&token.id_token, nonce).await
    }

    async fn verify_id_token(
        &self,
        token: &str,
        nonce: &str,
    ) -> Result<VerifiedGoogleIdentity, GoogleError> {
        let header = decode_header(token).map_err(|_| GoogleError::Invalid)?;
        if header.alg != Algorithm::RS256 {
            return Err(GoogleError::Invalid);
        }
        let kid = header.kid.ok_or(GoogleError::Invalid)?;
        let key = self.decoding_key(&kid).await?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.leeway = 30;
        validation.set_audience(&[&self.client_id]);
        validation.set_issuer(&["accounts.google.com", "https://accounts.google.com"]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        let claims = decode::<GoogleClaims>(token, &key, &validation)
            .map_err(|_| GoogleError::Invalid)?
            .claims;
        validate_claims(claims, nonce, &self.client_id)
    }

    async fn decoding_key(&self, kid: &str) -> Result<Arc<DecodingKey>, GoogleError> {
        {
            let cache = self.keys.read().await;
            if cache
                .expires_at
                .is_some_and(|expires| expires > std::time::Instant::now())
                && let Some(key) = cache.values.get(kid)
            {
                return Ok(key.clone());
            }
        }
        self.refresh_keys().await?;
        self.keys
            .read()
            .await
            .values
            .get(kid)
            .cloned()
            .ok_or(GoogleError::Invalid)
    }

    async fn refresh_keys(&self) -> Result<(), GoogleError> {
        let response = self
            .client
            .get(self.jwks_endpoint.clone())
            .send()
            .await
            .map_err(|_| GoogleError::Unavailable)?;
        if !response.status().is_success() {
            return Err(GoogleError::Unavailable);
        }
        let body = bounded_body(response).await?;
        let jwks: Jwks = serde_json::from_slice(&body).map_err(|_| GoogleError::Unavailable)?;
        let mut values = HashMap::new();
        for key in jwks.keys.into_iter().take(16) {
            if key.kty != "RSA" || key.alg != "RS256" || key.kid.is_empty() {
                continue;
            }
            let decoding = DecodingKey::from_rsa_components(&key.n, &key.e)
                .map_err(|_| GoogleError::Unavailable)?;
            values.insert(key.kid, Arc::new(decoding));
        }
        if values.is_empty() {
            return Err(GoogleError::Unavailable);
        }
        *self.keys.write().await = CachedKeys {
            expires_at: Some(std::time::Instant::now() + Duration::from_secs(15 * 60)),
            values,
        };
        Ok(())
    }
}

fn validate_exchange_input(
    code: &str,
    verifier: &str,
    redirect_uri: &str,
    nonce: &str,
) -> Result<(), GoogleError> {
    let verifier_ok = (43..=128).contains(&verifier.len())
        && verifier.bytes().all(|value| {
            value.is_ascii_alphanumeric() || matches!(value, b'-' | b'.' | b'_' | b'~')
        });
    let nonce_ok =
        (32..=256).contains(&nonce.len()) && nonce.bytes().all(|value| value.is_ascii_graphic());
    let redirect = Url::parse(redirect_uri).map_err(|_| GoogleError::Invalid)?;
    let redirect_ok = redirect.scheme() == "http"
        && redirect.host_str() == Some("127.0.0.1")
        && redirect.port().is_some()
        && redirect.path() == "/oauth2/callback"
        && redirect.query().is_none()
        && redirect.fragment().is_none()
        && redirect.username().is_empty()
        && redirect.password().is_none();
    if code.is_empty() || code.len() > 4096 || !verifier_ok || !nonce_ok || !redirect_ok {
        return Err(GoogleError::Invalid);
    }
    Ok(())
}

fn validate_claims(
    claims: GoogleClaims,
    expected_nonce: &str,
    client_id: &str,
) -> Result<VerifiedGoogleIdentity, GoogleError> {
    let now = jsonwebtoken::get_current_timestamp();
    if !claims.email_verified
        || claims.nonce != expected_nonce
        || claims
            .azp
            .as_deref()
            .is_some_and(|value| value != client_id)
        || claims.sub.is_empty()
        || claims.sub.len() > 255
        || claims.email.len() > 254
        || claims.iat > now + 30
        || claims.exp <= claims.iat
    {
        return Err(GoogleError::Invalid);
    }
    let email_normalized = normalize_email(&claims.email).ok_or(GoogleError::Invalid)?;
    let display_name = claims
        .name
        .map(|name| name.trim().chars().take(120).collect::<String>())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| {
            email_normalized
                .split('@')
                .next()
                .unwrap_or("Learner")
                .to_owned()
        });
    Ok(VerifiedGoogleIdentity {
        issuer: claims.iss,
        subject: claims.sub,
        email: claims.email,
        email_normalized,
        display_name,
    })
}

fn normalize_email(email: &str) -> Option<String> {
    let normalized = email.trim().to_lowercase();
    let (local, domain) = normalized.split_once('@')?;
    if local.is_empty()
        || domain.is_empty()
        || domain.starts_with('.')
        || domain.ends_with('.')
        || normalized.bytes().any(|value| value.is_ascii_whitespace())
    {
        return None;
    }
    Some(normalized)
}

fn fixed_https_or_loopback(url: &Url) -> bool {
    (url.scheme() == "https" || (url.scheme() == "http" && url.host_str() == Some("127.0.0.1")))
        && url.host_str().is_some()
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
}

async fn bounded_body(mut response: reqwest::Response) -> Result<Vec<u8>, GoogleError> {
    if response
        .content_length()
        .is_some_and(|length| length as usize > MAX_PROVIDER_BODY_BYTES)
    {
        return Err(GoogleError::Unavailable);
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| GoogleError::Unavailable)?
    {
        if body.len() + chunk.len() > MAX_PROVIDER_BODY_BYTES {
            return Err(GoogleError::Unavailable);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_exact_loopback_callback_and_pkce_shape() {
        let verifier = "a".repeat(64);
        let nonce = "n".repeat(32);
        assert!(
            validate_exchange_input(
                "code",
                &verifier,
                "http://127.0.0.1:51921/oauth2/callback",
                &nonce,
            )
            .is_ok()
        );
        assert!(
            validate_exchange_input(
                "code",
                &verifier,
                "http://localhost:51921/oauth2/callback",
                &nonce,
            )
            .is_err()
        );
        assert!(
            validate_exchange_input(
                "code",
                "short",
                "http://127.0.0.1:51921/oauth2/callback",
                &nonce,
            )
            .is_err()
        );
    }

    #[test]
    fn normalizes_without_google_specific_rewriting() {
        assert_eq!(
            normalize_email(" Student.Name+lab@GMAIL.com "),
            Some("student.name+lab@gmail.com".to_owned())
        );
        assert_eq!(normalize_email("missing-domain"), None);
    }
}
