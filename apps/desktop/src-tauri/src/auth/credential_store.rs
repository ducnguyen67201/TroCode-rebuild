use crate::worker::WorkerError;

#[cfg(feature = "desktop")]
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
#[cfg(feature = "desktop")]
use sha2::{Digest, Sha256};

#[async_trait::async_trait]
pub trait CredentialStore: Send + Sync {
    async fn read(&self) -> Result<Option<String>, WorkerError>;
    async fn write(&self, refresh_token: &str) -> Result<(), WorkerError>;
    async fn clear(&self) -> Result<(), WorkerError>;
}

#[cfg(feature = "desktop")]
pub(super) struct OsCredentialStore {
    user: String,
}

#[cfg(feature = "desktop")]
impl OsCredentialStore {
    const SERVICE: &'static str = "com.tro.desktop.auth";

    pub(super) fn for_origin(origin: &str) -> Self {
        let digest = Sha256::digest(origin.as_bytes());
        let scope = URL_SAFE_NO_PAD.encode(&digest[..18]);
        Self {
            user: format!("refresh-session-{scope}"),
        }
    }

    pub(super) fn unconfigured() -> Self {
        Self {
            user: "refresh-session-unconfigured".to_owned(),
        }
    }

    fn entry(user: &str) -> Result<keyring::Entry, WorkerError> {
        keyring::Entry::new(Self::SERVICE, user).map_err(|_| {
            WorkerError::new(
                "SECURE_STORAGE_UNAVAILABLE",
                "Secure credential storage is unavailable on this device.",
            )
        })
    }
}

#[cfg(feature = "desktop")]
#[async_trait::async_trait]
impl CredentialStore for OsCredentialStore {
    async fn read(&self) -> Result<Option<String>, WorkerError> {
        let user = self.user.clone();
        tokio::task::spawn_blocking(move || match Self::entry(&user)?.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(WorkerError::new(
                "SECURE_STORAGE_UNAVAILABLE",
                "Secure credential storage could not be read.",
            )),
        })
        .await
        .map_err(|_| WorkerError::new("INTERNAL", "Credential task failed."))?
    }

    async fn write(&self, refresh_token: &str) -> Result<(), WorkerError> {
        let user = self.user.clone();
        let refresh_token = refresh_token.to_owned();
        tokio::task::spawn_blocking(move || {
            Self::entry(&user)?
                .set_password(&refresh_token)
                .map_err(|_| {
                    WorkerError::new(
                        "SECURE_STORAGE_UNAVAILABLE",
                        "Secure credential storage could not save the session.",
                    )
                })
        })
        .await
        .map_err(|_| WorkerError::new("INTERNAL", "Credential task failed."))?
    }

    async fn clear(&self) -> Result<(), WorkerError> {
        let user = self.user.clone();
        tokio::task::spawn_blocking(move || match Self::entry(&user)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err(WorkerError::new(
                "SECURE_STORAGE_UNAVAILABLE",
                "Secure credential storage could not clear the session.",
            )),
        })
        .await
        .map_err(|_| WorkerError::new("INTERNAL", "Credential task failed."))?
    }
}

#[cfg(all(test, feature = "desktop"))]
mod os_tests {
    use super::OsCredentialStore;

    #[test]
    fn credential_namespace_is_stable_and_origin_scoped() {
        let production = OsCredentialStore::for_origin("https://api.tro.example/");
        let same_production = OsCredentialStore::for_origin("https://api.tro.example/");
        let local = OsCredentialStore::for_origin("http://127.0.0.1:4318/");

        assert_eq!(production.user, same_production.user);
        assert_ne!(production.user, local.user);
        assert!(!production.user.contains("api.tro.example"));
    }
}

#[cfg(test)]
pub struct MemoryCredentialStore {
    value: tokio::sync::Mutex<Option<String>>,
    fail_writes: std::sync::atomic::AtomicBool,
}

#[cfg(test)]
impl MemoryCredentialStore {
    pub fn new(value: Option<String>) -> Self {
        Self {
            value: tokio::sync::Mutex::new(value),
            fail_writes: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn fail_writes(&self) {
        self.fail_writes
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
#[async_trait::async_trait]
impl CredentialStore for MemoryCredentialStore {
    async fn read(&self) -> Result<Option<String>, WorkerError> {
        Ok(self.value.lock().await.clone())
    }

    async fn write(&self, refresh_token: &str) -> Result<(), WorkerError> {
        if self.fail_writes.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(WorkerError::new(
                "SECURE_STORAGE_UNAVAILABLE",
                "Secure credential storage could not save the session.",
            ));
        }
        *self.value.lock().await = Some(refresh_token.to_owned());
        Ok(())
    }

    async fn clear(&self) -> Result<(), WorkerError> {
        *self.value.lock().await = None;
        Ok(())
    }
}
