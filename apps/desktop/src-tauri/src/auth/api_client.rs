use super::{
    AuthUser, OAuthExchange, SessionEnvelope, WorkspaceMember, WorkspaceMemberList,
    WorkspaceSummary,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::time::Duration;
use url::Url;

const MAX_AUTH_RESPONSE_BYTES: usize = 32 * 1024;

#[derive(Clone)]
pub struct AuthApiClient {
    origin: Url,
    client: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct ApiFailure {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PublicError {
    code: String,
    message: String,
    retryable: bool,
    #[allow(dead_code)]
    correlation_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RefreshRequest<'a> {
    refresh_token: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddWorkspaceMemberRequest<'a> {
    email: &'a str,
    role: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MeResponse {
    pub account: AuthUser,
    pub workspaces: Vec<WorkspaceSummary>,
}

impl AuthApiClient {
    pub fn new(origin: &str, allow_loopback_http: bool) -> Result<Self, &'static str> {
        let origin = Url::parse(origin).map_err(|_| "Invalid authentication API origin.")?;
        let allowed_scheme = origin.scheme() == "https"
            || (allow_loopback_http
                && origin.scheme() == "http"
                && origin.host_str() == Some("127.0.0.1"));
        if !allowed_scheme
            || origin.host_str().is_none()
            || !origin.username().is_empty()
            || origin.password().is_some()
            || origin.path() != "/"
            || origin.query().is_some()
            || origin.fragment().is_some()
        {
            return Err("Authentication API must be a fixed HTTPS origin.");
        }
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(12))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "Authentication client unavailable.")?;
        Ok(Self { origin, client })
    }

    #[cfg(feature = "desktop")]
    pub(super) fn origin(&self) -> &str {
        self.origin.as_str()
    }

    pub async fn exchange(&self, exchange: &OAuthExchange) -> Result<SessionEnvelope, ApiFailure> {
        let response = self
            .client
            .post(self.url("v1/auth/google/exchange"))
            .json(exchange)
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        response_json(response).await
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<SessionEnvelope, ApiFailure> {
        let response = self
            .client
            .post(self.url("v1/auth/session/refresh"))
            .json(&RefreshRequest { refresh_token })
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        response_json(response).await
    }

    pub async fn me(&self, access_token: &str) -> Result<MeResponse, ApiFailure> {
        let response = self
            .client
            .get(self.url("v1/me"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        response_json(response).await
    }

    pub async fn logout(&self, access_token: &str) -> Result<(), ApiFailure> {
        let response = self
            .client
            .delete(self.url("v1/auth/session"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        if response.status().is_success() {
            return Ok(());
        }
        Err(error_response(response).await)
    }

    pub async fn workspace_members(
        &self,
        access_token: &str,
        workspace_id: &str,
    ) -> Result<WorkspaceMemberList, ApiFailure> {
        let response = self
            .client
            .get(self.url(&format!("v1/workspaces/{workspace_id}/members")))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        response_json(response).await
    }

    pub async fn add_workspace_member(
        &self,
        access_token: &str,
        workspace_id: &str,
        email: &str,
        role: &str,
    ) -> Result<WorkspaceMember, ApiFailure> {
        let response = self
            .client
            .post(self.url(&format!("v1/workspaces/{workspace_id}/members")))
            .bearer_auth(access_token)
            .json(&AddWorkspaceMemberRequest { email, role })
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        response_json(response).await
    }

    pub async fn remove_workspace_member(
        &self,
        access_token: &str,
        workspace_id: &str,
        membership_id: &str,
    ) -> Result<(), ApiFailure> {
        let response = self
            .client
            .delete(self.url(&format!(
                "v1/workspaces/{workspace_id}/members/{membership_id}"
            )))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| ApiFailure::unavailable())?;
        if response.status().is_success() {
            return Ok(());
        }
        Err(error_response(response).await)
    }

    fn url(&self, path: &str) -> Url {
        self.origin.join(path).expect("fixed relative auth path")
    }
}

impl ApiFailure {
    fn unavailable() -> Self {
        Self {
            code: "AUTH_UNAVAILABLE".to_owned(),
            message: "Authentication is temporarily unavailable. Try again.".to_owned(),
            retryable: true,
        }
    }

    pub fn terminal(&self) -> bool {
        matches!(
            self.code.as_str(),
            "SESSION_EXPIRED" | "SESSION_REVOKED" | "SESSION_REPLAYED"
        )
    }
}

async fn response_json<T: DeserializeOwned>(response: reqwest::Response) -> Result<T, ApiFailure> {
    if !response.status().is_success() {
        return Err(error_response(response).await);
    }
    let body = bounded_body(response)
        .await
        .map_err(|_| ApiFailure::unavailable())?;
    serde_json::from_slice(&body).map_err(|_| ApiFailure {
        code: "INVALID_MESSAGE".to_owned(),
        message: "The authentication service returned an invalid response.".to_owned(),
        retryable: false,
    })
}

async fn error_response(response: reqwest::Response) -> ApiFailure {
    bounded_body(response)
        .await
        .ok()
        .and_then(|body| serde_json::from_slice::<PublicError>(&body).ok())
        .map(|error| ApiFailure {
            code: error.code,
            message: error.message,
            retryable: error.retryable,
        })
        .unwrap_or_else(ApiFailure::unavailable)
}

async fn bounded_body(mut response: reqwest::Response) -> Result<Vec<u8>, ()> {
    if response
        .content_length()
        .is_some_and(|length| length as usize > MAX_AUTH_RESPONSE_BYTES)
    {
        return Err(());
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| ())? {
        if body.len() + chunk.len() > MAX_AUTH_RESPONSE_BYTES {
            return Err(());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_is_fixed_and_fail_closed() {
        assert!(AuthApiClient::new("https://api.tro.example", false).is_ok());
        assert!(AuthApiClient::new("http://127.0.0.1:4318", true).is_ok());
        assert!(AuthApiClient::new("http://127.0.0.1:4318", false).is_err());
        assert!(AuthApiClient::new("https://api.tro.example/path", false).is_err());
        assert!(AuthApiClient::new("https://user@api.tro.example", false).is_err());
    }
}
