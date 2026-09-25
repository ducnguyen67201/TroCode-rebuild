use crate::worker::WorkerError;

#[async_trait::async_trait]
pub trait CredentialStore: Send + Sync {
    async fn read(&self) -> Result<Option<String>, WorkerError>;
    async fn write(&self, refresh_token: &str) -> Result<(), WorkerError>;
    async fn clear(&self) -> Result<(), WorkerError>;
}

#[cfg(feature = "desktop")]
pub struct OsCredentialStore;

#[cfg(feature = "desktop")]
impl OsCredentialStore {
    const SERVICE: &'static str = "com.tro.desktop.auth";
    const USER: &'static str = "refresh-session";

    fn entry() -> Result<keyring::Entry, WorkerError> {
        keyring::Entry::new(Self::SERVICE, Self::USER).map_err(|_| {
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
        tokio::task::spawn_blocking(|| match Self::entry()?.get_password() {
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
        let refresh_token = refresh_token.to_owned();
        tokio::task::spawn_blocking(move || {
            Self::entry()?.set_password(&refresh_token).map_err(|_| {
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
        tokio::task::spawn_blocking(|| match Self::entry()?.delete_credential() {
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
