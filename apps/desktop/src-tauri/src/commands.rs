use crate::{lifecycle::Status, manager::RuntimeManager, worker::WorkerError};
use std::{sync::Arc, time::Duration};
use tauri::State;
use uuid::Uuid;
type Manager<'a> = State<'a, Arc<RuntimeManager>>;
#[tauri::command]
pub async fn runtime_start(manager: Manager<'_>) -> Result<Status, WorkerError> {
    manager.start().await
}
#[tauri::command]
pub async fn runtime_health(manager: Manager<'_>) -> Result<Status, WorkerError> {
    manager.health().await
}
#[tauri::command]
pub async fn runtime_stop(manager: Manager<'_>) -> Result<Status, WorkerError> {
    Ok(manager.stop().await)
}
#[tauri::command]
pub fn runtime_status(manager: Manager<'_>) -> Status {
    manager.status()
}
#[tauri::command]
pub async fn runtime_restart(manager: Manager<'_>) -> Result<Status, WorkerError> {
    manager.stop().await;
    manager.start().await
}
#[tauri::command]
pub async fn account_select(profile: String, manager: Manager<'_>) -> Result<Status, WorkerError> {
    let ticket = manager.begin_account_change().await;
    if !cfg!(debug_assertions) || !matches!(profile.as_str(), "teacher" | "student-a" | "student-b")
    {
        return Err(WorkerError::new(
            "FORBIDDEN",
            "Select a configured development profile.",
        ));
    }
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.local/profiles.json");
    let profiles: serde_json::Value = serde_json::from_slice(
        &std::fs::read(path)
            .map_err(|_| WorkerError::new("NOT_READY", "Run npm run db:seed first."))?,
    )
    .map_err(|_| WorkerError::new("NOT_READY", "Development profiles are invalid."))?;
    let token = profiles[&profile]
        .as_str()
        .ok_or(WorkerError::new("UNAUTHORIZED", "Profile unavailable."))?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| WorkerError::new("INTERNAL", "Unable to initialize account client."))?;
    // Fixed loopback fixture origin; never send local fixture credentials to a supplied URL.
    let response = client
        .get("http://127.0.0.1:4318/v1/me")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Start the local fixture API."))?;
    if !response.status().is_success() {
        return Err(WorkerError::new(
            "UNAUTHORIZED",
            "Development credential was rejected.",
        ));
    }
    let identity: serde_json::Value = response
        .json()
        .await
        .map_err(|_| WorkerError::new("UNAUTHORIZED", "Invalid account response."))?;
    let account = identity["accountId"]
        .as_str()
        .and_then(|value| Uuid::parse_str(value).ok())
        .ok_or(WorkerError::new(
            "UNAUTHORIZED",
            "Invalid account identity.",
        ))?;
    manager
        .finish_account_change(ticket, Some(account.to_string()))
        .await?;
    Ok(manager.status())
}
