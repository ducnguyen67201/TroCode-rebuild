//! Native observation consent only; no event posting or input-injection permission.
use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    screen_capture: Option<bool>,
    accessibility: Option<bool>,
    message: &'static str,
}
#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
    fn AXIsProcessTrusted() -> bool;
}
#[tauri::command]
pub async fn observation_permissions(
    window: tauri::WebviewWindow,
    request: bool,
) -> Result<Permissions, crate::worker::WorkerError> {
    if window.label() != "main" {
        return Err(crate::worker::WorkerError::new(
            "FORBIDDEN",
            "Main window required.",
        ));
    }
    let (sender, receiver) = tokio::sync::oneshot::channel();
    window.run_on_main_thread(move || {
        #[cfg(target_os="macos")]
        let result=unsafe { Permissions {
            screen_capture: Some(if request {CGRequestScreenCaptureAccess()} else {CGPreflightScreenCaptureAccess()}),
            accessibility: Some(AXIsProcessTrusted()),
            message:"Enable Tro in macOS Privacy & Security → Accessibility and Screen Recording. Relaunch after changes.",
        }};
        #[cfg(not(target_os="macos"))]
        let result={let _=request; Permissions {screen_capture:None,accessibility:None,message:"Selected-window observation checks access when you press Observe. Protected or elevated windows may be unavailable."}};
        let _=sender.send(result);
    }).map_err(|_| crate::worker::WorkerError::new("NOT_READY", "Permission check unavailable."))?;
    receiver
        .await
        .map_err(|_| crate::worker::WorkerError::new("NOT_READY", "Permission check interrupted."))
}
