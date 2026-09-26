use crate::{
    auth::{AuthManager, AuthStatus, WorkspaceMember, WorkspaceMemberList},
    lifecycle::Status,
    manager::RuntimeManager,
    voice::{VoiceManager, VoiceStatus},
    worker::WorkerError,
};
use std::{sync::Arc, time::Duration};
use tauri::State;
use uuid::Uuid;
type Manager<'a> = State<'a, Arc<RuntimeManager>>;
type Authentication<'a> = State<'a, Arc<AuthManager>>;
type Voice<'a> = State<'a, Arc<VoiceManager>>;
type ModifierListener<'a> = State<'a, Arc<crate::modifier_chord::ModifierListener>>;

fn require_main(window: &tauri::WebviewWindow) -> Result<(), WorkerError> {
    if window.label() == "main" {
        Ok(())
    } else {
        Err(WorkerError::new("FORBIDDEN", "Main window required."))
    }
}

#[tauri::command]
pub fn app_relaunch(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
) -> Result<(), WorkerError> {
    require_main(&window)?;
    app.request_restart();
    Ok(())
}

async fn require_workspace(auth: &AuthManager) -> Result<(), WorkerError> {
    auth.require_workspace_access().await
}

#[tauri::command]
pub fn auth_status(
    window: tauri::WebviewWindow,
    auth: Authentication<'_>,
) -> Result<AuthStatus, WorkerError> {
    require_main(&window)?;
    Ok(auth.status())
}

#[tauri::command]
pub async fn auth_sign_in_google(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    auth: Authentication<'_>,
) -> Result<AuthStatus, WorkerError> {
    require_main(&window)?;
    Ok(auth.sign_in(&app).await)
}

#[tauri::command]
pub async fn auth_retry(
    window: tauri::WebviewWindow,
    auth: Authentication<'_>,
) -> Result<AuthStatus, WorkerError> {
    require_main(&window)?;
    Ok(auth.retry().await)
}

#[tauri::command]
pub async fn auth_sign_out(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    auth: Authentication<'_>,
) -> Result<AuthStatus, WorkerError> {
    require_main(&window)?;
    crate::overlay::hide(&app);
    Ok(auth.sign_out().await)
}

#[tauri::command]
pub async fn workspace_members(
    window: tauri::WebviewWindow,
    workspace_id: String,
    auth: Authentication<'_>,
) -> Result<WorkspaceMemberList, WorkerError> {
    require_main(&window)?;
    auth.workspace_members(&workspace_id).await
}

#[tauri::command]
pub async fn workspace_add_member(
    window: tauri::WebviewWindow,
    workspace_id: String,
    email: String,
    role: String,
    auth: Authentication<'_>,
) -> Result<WorkspaceMember, WorkerError> {
    require_main(&window)?;
    if email.len() > 254 || role.len() > 16 {
        return Err(WorkerError::new(
            "INVALID_MESSAGE",
            "Invalid workspace membership request.",
        ));
    }
    auth.add_workspace_member(&workspace_id, &email, &role)
        .await
}

#[tauri::command]
pub async fn workspace_remove_member(
    window: tauri::WebviewWindow,
    workspace_id: String,
    membership_id: String,
    auth: Authentication<'_>,
) -> Result<(), WorkerError> {
    require_main(&window)?;
    if Uuid::parse_str(&membership_id).is_err() {
        return Err(WorkerError::new(
            "INVALID_MESSAGE",
            "Invalid workspace membership request.",
        ));
    }
    auth.remove_workspace_member(&workspace_id, &membership_id)
        .await
}

#[tauri::command]
pub async fn runtime_start(
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<Status, WorkerError> {
    require_workspace(&auth).await?;
    manager.start().await
}
#[tauri::command]
pub async fn runtime_health(
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<Status, WorkerError> {
    require_workspace(&auth).await?;
    manager.health().await
}
#[tauri::command]
pub async fn runtime_stop(
    app: tauri::AppHandle,
    manager: Manager<'_>,
) -> Result<Status, WorkerError> {
    crate::overlay::hide(&app);
    Ok(manager.stop().await)
}
#[tauri::command]
pub async fn runtime_status(
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<Status, WorkerError> {
    require_workspace(&auth).await?;
    Ok(manager.status())
}
#[tauri::command]
pub async fn runtime_restart(
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<Status, WorkerError> {
    require_workspace(&auth).await?;
    manager.stop().await;
    manager.start().await
}
#[tauri::command]
pub async fn account_select(
    profile: String,
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<Status, WorkerError> {
    require_workspace(&auth).await?;
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

#[tauri::command]
pub async fn teaching_request(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    kind: String,
    payload: serde_json::Value,
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<serde_json::Value, WorkerError> {
    require_main(&window)?;
    require_workspace(&auth).await?;
    crate::overlay::hide(&app);
    let epoch = crate::overlay::epoch(&app);
    let state = manager.teaching(&kind, payload).await?;
    crate::overlay::present(&app, &state, epoch).await?;
    if crate::overlay::should_track(&state) {
        crate::overlay::track(app.clone(), manager.inner().clone(), epoch);
    }
    Ok(state)
}

#[tauri::command]
pub async fn proof_connect(
    window: tauri::WebviewWindow,
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<Status, WorkerError> {
    require_main(&window)?;
    require_workspace(&auth).await?;
    manager.connect_proof().await
}

#[tauri::command]
pub fn voice_status(
    window: tauri::WebviewWindow,
    voice: Voice<'_>,
) -> Result<VoiceStatus, WorkerError> {
    require_main(&window)?;
    Ok(voice.status())
}

#[tauri::command]
pub async fn voice_enable(
    window: tauri::WebviewWindow,
    voice: Voice<'_>,
    listener: ModifierListener<'_>,
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<VoiceStatus, WorkerError> {
    require_main(&window)?;
    require_workspace(&auth).await?;
    arm_voice_control(
        voice.inner().clone(),
        listener.inner().clone(),
        manager.inner().clone(),
    )
    .await
}

pub(crate) async fn arm_voice_control(
    voice: Arc<VoiceManager>,
    listener: Arc<crate::modifier_chord::ModifierListener>,
    manager: Arc<RuntimeManager>,
) -> Result<VoiceStatus, WorkerError> {
    manager.start().await?;
    let status = voice.enable(crate::permissions::voice_permissions(true));
    listener
        .set_enabled(status.permissions.ready)
        .map_err(|_| WorkerError::new("NOT_READY", "Global push-to-talk listener unavailable."))?;
    Ok(status)
}

#[tauri::command]
pub async fn voice_disable(
    window: tauri::WebviewWindow,
    voice: Voice<'_>,
    listener: ModifierListener<'_>,
    manager: Manager<'_>,
) -> Result<VoiceStatus, WorkerError> {
    require_main(&window)?;
    let _ = listener.set_enabled(false);
    voice.cancel(Some(&manager)).await;
    Ok(voice.disable())
}

#[tauri::command]
pub async fn voice_execute_text(
    window: tauri::WebviewWindow,
    instruction: String,
    voice: Voice<'_>,
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<VoiceStatus, WorkerError> {
    require_main(&window)?;
    require_workspace(&auth).await?;
    voice
        .execute_text(instruction, manager.inner().clone(), auth.inner().clone())
        .await
}

#[tauri::command]
pub async fn voice_cancel(
    window: tauri::WebviewWindow,
    voice: Voice<'_>,
    manager: Manager<'_>,
) -> Result<VoiceStatus, WorkerError> {
    require_main(&window)?;
    Ok(voice.cancel(Some(&manager)).await)
}

#[tauri::command]
pub async fn voice_decide(
    window: tauri::WebviewWindow,
    run_id: String,
    confirmation_id: String,
    approve: bool,
    voice: Voice<'_>,
    manager: Manager<'_>,
    auth: Authentication<'_>,
) -> Result<VoiceStatus, WorkerError> {
    require_main(&window)?;
    require_workspace(&auth).await?;
    let run_id = Uuid::parse_str(&run_id)
        .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid action decision."))?;
    let confirmation_id = Uuid::parse_str(&confirmation_id)
        .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid action decision."))?;
    if !manager
        .decide_action(run_id, confirmation_id, approve)
        .await?
    {
        return Err(WorkerError::new(
            "NOT_READY",
            "This confirmation is no longer active.",
        ));
    }
    Ok(voice.status())
}
