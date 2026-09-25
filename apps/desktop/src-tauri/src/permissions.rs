//! Native permission projection. Prompts are initiated only by explicit setup commands.
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

pub fn voice_permissions(request: bool) -> crate::voice::VoicePermissions {
    #[cfg(target_os = "macos")]
    let keyboard = unsafe { AXIsProcessTrusted() };
    #[cfg(not(target_os = "macos"))]
    let keyboard = true;
    let microphone = if request {
        crate::voice::audio::MicrophoneCapture::start().is_ok()
    } else {
        false
    };
    crate::voice::VoicePermissions {
        microphone: if microphone { "granted" } else if request { "denied" } else { "prompt" }.into(),
        keyboard_monitoring: if keyboard { "granted" } else { "denied" }.into(),
        ready: microphone && keyboard,
        recovery: if cfg!(target_os = "macos") {
            "Enable Tro in Privacy & Security → Microphone and Accessibility/Input Monitoring, then relaunch."
        } else {
            "Enable microphone privacy access. Protected or elevated windows are unavailable."
        }.into(),
    }
}
