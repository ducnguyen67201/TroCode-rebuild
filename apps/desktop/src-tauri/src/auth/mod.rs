pub mod api_client;
pub mod credential_store;
#[cfg(feature = "desktop")]
pub mod oauth;

use crate::manager::RuntimeManager;
use api_client::{ApiFailure, AuthApiClient};
use credential_store::CredentialStore;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use tokio::sync::{Mutex, watch};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthUser {
    pub account_id: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceSummary {
    pub workspace_id: String,
    pub name: String,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SessionEnvelope {
    access_token: String,
    access_token_expires_at: String,
    refresh_token: String,
    #[allow(dead_code)]
    refresh_token_expires_at: String,
    account: AuthUser,
    workspaces: Vec<WorkspaceSummary>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub state: String,
    pub revision: u64,
    pub message: String,
    pub configured: bool,
    pub retryable: bool,
    pub user: Option<AuthUser>,
    pub workspaces: Vec<WorkspaceSummary>,
    pub access_token_expires_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthExchange {
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: String,
    pub nonce: String,
}

#[derive(Clone)]
struct ActiveSession {
    access_token: String,
    access_token_expires_at: String,
    account: AuthUser,
    workspaces: Vec<WorkspaceSummary>,
}

pub struct AuthManager {
    runtime: Arc<RuntimeManager>,
    api: Option<AuthApiClient>,
    #[cfg(feature = "desktop")]
    google_client_id: Option<String>,
    credentials: Arc<dyn CredentialStore>,
    session: Mutex<Option<ActiveSession>>,
    status: watch::Sender<AuthStatus>,
    operation: Mutex<()>,
    epoch: std::sync::atomic::AtomicU64,
}

impl AuthManager {
    #[cfg(feature = "desktop")]
    pub fn from_env(runtime: Arc<RuntimeManager>) -> Self {
        let credentials: Arc<dyn CredentialStore> = Arc::new(credential_store::OsCredentialStore);
        let configuration = (|| {
            let origin = std::env::var("TRO_AUTH_API_ORIGIN").map_err(|_| ())?;
            let client_id = std::env::var("TRO_GOOGLE_CLIENT_ID").map_err(|_| ())?;
            if client_id.is_empty()
                || client_id.len() > 255
                || !client_id.ends_with(".apps.googleusercontent.com")
            {
                return Err(());
            }
            let api = AuthApiClient::new(&origin, cfg!(debug_assertions)).map_err(|_| ())?;
            Ok((api, client_id))
        })();
        match configuration {
            Ok((api, client_id)) => Self::new(runtime, Some(api), Some(client_id), credentials),
            Err(()) => Self::new(runtime, None, None, credentials),
        }
    }

    pub fn new(
        runtime: Arc<RuntimeManager>,
        api: Option<AuthApiClient>,
        google_client_id: Option<String>,
        credentials: Arc<dyn CredentialStore>,
    ) -> Self {
        let configured = api.is_some() && google_client_id.is_some();
        let initial = if configured {
            status(
                "checking",
                "Checking this device for a secure session…",
                true,
                false,
            )
        } else {
            status(
                "error",
                "Google sign-in is not configured for this build.",
                false,
                false,
            )
        };
        let (status, _) = watch::channel(initial);
        #[cfg(not(feature = "desktop"))]
        let _ = google_client_id;
        Self {
            runtime,
            api,
            #[cfg(feature = "desktop")]
            google_client_id,
            credentials,
            session: Mutex::new(None),
            status,
            operation: Mutex::new(()),
            epoch: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn subscribe(&self) -> watch::Receiver<AuthStatus> {
        self.status.subscribe()
    }

    pub fn status(&self) -> AuthStatus {
        self.status.borrow().clone()
    }

    pub fn has_workspace_access(&self) -> bool {
        let current = self.status();
        current.state == "authenticated"
            && !current.workspaces.is_empty()
            && current
                .access_token_expires_at
                .as_deref()
                .and_then(|value| OffsetDateTime::parse(value, &Rfc3339).ok())
                .is_some_and(|expires_at| expires_at > OffsetDateTime::now_utc())
    }

    pub async fn restore(&self) -> AuthStatus {
        if self.api.is_none() {
            return self.status();
        }
        let ticket = self.next_epoch();
        let _operation = self.operation.lock().await;
        match self.credentials.read().await {
            Ok(Some(refresh_token)) => self.refresh_locked(&refresh_token, ticket).await,
            Ok(None) if self.is_current(ticket) => {
                self.clear_runtime_identity().await;
                self.publish(status(
                    "signedOut",
                    "Sign in to continue to your Tro workspace.",
                    true,
                    false,
                ))
            }
            Err(_) if self.is_current(ticket) => self.publish(status(
                "error",
                "Secure credential storage is unavailable on this device.",
                true,
                true,
            )),
            _ => self.status(),
        }
    }

    #[cfg(feature = "desktop")]
    pub async fn sign_in(&self, app: &tauri::AppHandle) -> AuthStatus {
        let (Some(api), Some(client_id)) = (&self.api, &self.google_client_id) else {
            return self.status();
        };
        let ticket = self.next_epoch();
        self.publish(status(
            "signingIn",
            "Continue in your browser to finish Google sign-in…",
            true,
            false,
        ));
        let _operation = self.operation.lock().await;
        let exchange = match oauth::authorize(app, client_id).await {
            Ok(exchange) => exchange,
            Err(error) => {
                if !self.is_current(ticket) {
                    return self.status();
                }
                let cancelled = error.code == "AUTH_CANCELLED";
                return self.publish(status(
                    if cancelled { "signedOut" } else { "error" },
                    error.message,
                    true,
                    !cancelled,
                ));
            }
        };
        if !self.is_current(ticket) {
            return self.status();
        }
        match api.exchange(&exchange).await {
            Ok(envelope) => self.apply_envelope(envelope, ticket).await,
            Err(error) => self.apply_api_failure(error, ticket).await,
        }
    }

    pub async fn retry(&self) -> AuthStatus {
        if self.api.is_none() {
            return self.status();
        }
        let ticket = self.next_epoch();
        let _operation = self.operation.lock().await;
        match self.credentials.read().await {
            Ok(Some(refresh_token)) => self.refresh_locked(&refresh_token, ticket).await,
            Ok(None) if self.is_current(ticket) => self.publish(status(
                "signedOut",
                "Sign in to continue to your Tro workspace.",
                true,
                false,
            )),
            Err(_) if self.is_current(ticket) => self.publish(status(
                "error",
                "Secure credential storage is unavailable on this device.",
                true,
                true,
            )),
            _ => self.status(),
        }
    }

    pub async fn sign_out(&self) -> AuthStatus {
        self.next_epoch();
        let _operation = self.operation.lock().await;
        let active = self.session.lock().await.clone();
        let access_token = active
            .filter(|session| access_is_valid(&session.access_token_expires_at))
            .map(|session| session.access_token);
        let access_token = if access_token.is_some() {
            access_token
        } else if let Some(api) = &self.api {
            match self.credentials.read().await {
                Ok(Some(refresh_token)) => api
                    .refresh(&refresh_token)
                    .await
                    .ok()
                    .map(|envelope| envelope.access_token),
                _ => None,
            }
        } else {
            None
        };
        if let (Some(api), Some(access_token)) = (&self.api, access_token) {
            let _ = api.logout(&access_token).await;
        }
        let cleared = self.credentials.clear().await.is_ok();
        self.clear_runtime_identity().await;
        self.publish(status(
            if cleared { "signedOut" } else { "error" },
            if cleared {
                "You’re signed out on this device."
            } else {
                "Tro signed out, but secure storage needs attention before another sign-in."
            },
            self.api.is_some(),
            !cleared,
        ))
    }

    pub async fn refresh_loop(self: Arc<Self>) {
        let mut status = self.subscribe();
        loop {
            let current = status.borrow().clone();
            let expires_at = current.access_token_expires_at.clone();
            let Some(expires_at) =
                expires_at.and_then(|value| OffsetDateTime::parse(&value, &Rfc3339).ok())
            else {
                if status.changed().await.is_err() {
                    return;
                }
                continue;
            };
            let until_refresh =
                expires_at - OffsetDateTime::now_utc() - time::Duration::seconds(60);
            let delay = if current.state == "offline" {
                Duration::from_secs(30)
            } else if until_refresh.is_positive() {
                Duration::from_secs(until_refresh.whole_seconds() as u64)
            } else {
                Duration::ZERO
            };
            let sleep = tokio::time::sleep(delay);
            tokio::pin!(sleep);
            tokio::select! {
                () = &mut sleep => { self.retry().await; }
                changed = status.changed() => { if changed.is_err() { return; } }
            }
        }
    }

    async fn refresh_locked(&self, refresh_token: &str, ticket: u64) -> AuthStatus {
        let Some(api) = &self.api else {
            return self.status();
        };
        let result = api.refresh(refresh_token).await;
        if !self.is_current(ticket) {
            if let Ok(envelope) = result {
                let _ = api.logout(&envelope.access_token).await;
            }
            return self.status();
        }
        match result {
            Ok(envelope) => self.apply_envelope(envelope, ticket).await,
            Err(error) if error.terminal() => {
                let _ = self.credentials.clear().await;
                self.clear_runtime_identity().await;
                self.publish(status(
                    "signedOut",
                    "Your session ended. Sign in again to continue.",
                    true,
                    false,
                ))
            }
            Err(error) => self.apply_api_failure(error, ticket).await,
        }
    }

    async fn apply_envelope(&self, envelope: SessionEnvelope, ticket: u64) -> AuthStatus {
        if !self.is_current(ticket) {
            if let Some(api) = &self.api {
                let _ = api.logout(&envelope.access_token).await;
            }
            return self.status();
        }
        if self
            .credentials
            .write(&envelope.refresh_token)
            .await
            .is_err()
        {
            if let Some(api) = &self.api {
                let _ = api.logout(&envelope.access_token).await;
            }
            self.clear_runtime_identity().await;
            return self.publish(status(
                "error",
                "Tro could not protect this session in secure storage.",
                true,
                false,
            ));
        }
        if !self.is_current(ticket) {
            if let Some(api) = &self.api {
                let _ = api.logout(&envelope.access_token).await;
            }
            let _ = self.credentials.clear().await;
            return self.status();
        }
        let has_workspace = !envelope.workspaces.is_empty();
        let active = ActiveSession {
            access_token: envelope.access_token,
            access_token_expires_at: envelope.access_token_expires_at.clone(),
            account: envelope.account.clone(),
            workspaces: envelope.workspaces.clone(),
        };
        let runtime_ticket = self.runtime.begin_account_change().await;
        let account = has_workspace.then(|| active.account.account_id.clone());
        if self
            .runtime
            .finish_account_change(runtime_ticket, account)
            .await
            .is_err()
        {
            if let Some(api) = &self.api {
                let _ = api.logout(&active.access_token).await;
            }
            let _ = self.credentials.clear().await;
            return self.publish(status(
                "error",
                "Tro could not switch to the authenticated account.",
                true,
                true,
            ));
        }
        if !self.is_current(ticket) {
            if let Some(api) = &self.api {
                let _ = api.logout(&active.access_token).await;
            }
            let _ = self.credentials.clear().await;
            self.clear_runtime_identity().await;
            return self.status();
        }
        *self.session.lock().await = Some(active.clone());
        self.publish(AuthStatus {
            state: if has_workspace {
                "authenticated".to_owned()
            } else {
                "membershipRequired".to_owned()
            },
            revision: 0,
            message: if has_workspace {
                "Signed in securely.".to_owned()
            } else {
                "Ask a workspace owner to add this exact Google email, then retry.".to_owned()
            },
            configured: true,
            retryable: !has_workspace,
            user: Some(active.account),
            workspaces: active.workspaces,
            access_token_expires_at: Some(active.access_token_expires_at),
        })
    }

    async fn apply_api_failure(&self, error: ApiFailure, ticket: u64) -> AuthStatus {
        if !self.is_current(ticket) {
            return self.status();
        }
        let session = self.session.lock().await.clone();
        self.publish(AuthStatus {
            state: if error.retryable { "offline" } else { "error" }.to_owned(),
            revision: 0,
            message: bounded_message(&error.message),
            configured: self.api.is_some(),
            retryable: error.retryable,
            user: session.as_ref().map(|value| value.account.clone()),
            workspaces: session
                .as_ref()
                .map(|value| value.workspaces.clone())
                .unwrap_or_default(),
            access_token_expires_at: session.map(|value| value.access_token_expires_at),
        })
    }

    async fn clear_runtime_identity(&self) {
        *self.session.lock().await = None;
        self.runtime.select_account(None).await;
    }

    fn next_epoch(&self) -> u64 {
        self.epoch.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
    }

    fn is_current(&self, ticket: u64) -> bool {
        self.epoch.load(std::sync::atomic::Ordering::SeqCst) == ticket
    }

    fn publish(&self, mut next: AuthStatus) -> AuthStatus {
        self.status.send_modify(|current| {
            next.revision = current.revision.saturating_add(1);
            *current = next.clone();
        });
        next
    }
}

fn status(state: &str, message: &str, configured: bool, retryable: bool) -> AuthStatus {
    AuthStatus {
        state: state.to_owned(),
        revision: 0,
        message: bounded_message(message),
        configured,
        retryable,
        user: None,
        workspaces: Vec::new(),
        access_token_expires_at: None,
    }
}

fn bounded_message(message: &str) -> String {
    message.chars().take(256).collect()
}

fn access_is_valid(expires_at: &str) -> bool {
    OffsetDateTime::parse(expires_at, &Rfc3339)
        .is_ok_and(|expires_at| expires_at > OffsetDateTime::now_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_status_never_contains_credentials() {
        let value = serde_json::to_value(status("signedOut", "Sign in.", true, false)).unwrap();
        assert!(value.get("accessToken").is_none());
        assert!(value.get("refreshToken").is_none());
        assert_eq!(value["state"], "signedOut");
    }

    #[test]
    fn messages_are_bounded() {
        assert_eq!(bounded_message(&"x".repeat(400)).len(), 256);
    }

    #[test]
    fn access_validity_is_fail_closed() {
        let future = (OffsetDateTime::now_utc() + time::Duration::minutes(5))
            .format(&Rfc3339)
            .unwrap();
        let expired = (OffsetDateTime::now_utc() - time::Duration::minutes(5))
            .format(&Rfc3339)
            .unwrap();
        assert!(access_is_valid(&future));
        assert!(!access_is_valid(&expired));
        assert!(!access_is_valid("not-a-timestamp"));
    }
}
