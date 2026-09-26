//! Native device readiness only; this module never posts input or retains media.
use crate::worker::WorkerError;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_opener::OpenerExt;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionRequestKind {
    Microphone,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionSettingsTarget {
    ScreenCapture,
    Accessibility,
    Microphone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SettingsPlatform {
    Macos,
    Windows,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // The closed vocabulary has target-specific variants.
enum CapabilityStatus {
    Granted,
    Available,
    NotDetermined,
    Denied,
    Unavailable,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // The closed vocabulary has platform-specific variants.
enum RecoveryAction {
    None,
    Request,
    ManualSettings,
    Recheck,
    Relaunch,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapability {
    status: CapabilityStatus,
    required: bool,
    can_request: bool,
    recovery: RecoveryAction,
    message: &'static str,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceReadiness {
    platform: &'static str,
    requires_relaunch: bool,
    message: &'static str,
    screen_capture: DeviceCapability,
    accessibility: DeviceCapability,
    microphone: DeviceCapability,
}

fn capability(
    status: CapabilityStatus,
    required: bool,
    can_request: bool,
    recovery: RecoveryAction,
    message: &'static str,
) -> DeviceCapability {
    DeviceCapability {
        status,
        required,
        can_request,
        recovery,
        message,
    }
}

fn require_main(window: &tauri::WebviewWindow) -> Result<(), WorkerError> {
    if window.label() == "main" {
        Ok(())
    } else {
        Err(WorkerError::new("FORBIDDEN", "Main window required."))
    }
}

#[tauri::command]
pub async fn device_readiness(
    window: tauri::WebviewWindow,
) -> Result<DeviceReadiness, WorkerError> {
    require_main(&window)?;
    let settings_target = crate::permission_guide::take_target(window.app_handle());
    platform_check(&window, settings_target).await
}

#[tauri::command]
pub async fn device_permission_request(
    window: tauri::WebviewWindow,
    kind: PermissionRequestKind,
) -> Result<DeviceReadiness, WorkerError> {
    require_main(&window)?;
    platform_request(&window, kind).await
}

#[tauri::command]
pub fn device_permission_settings(
    window: tauri::WebviewWindow,
    target: PermissionSettingsTarget,
) -> Result<(), WorkerError> {
    require_main(&window)?;
    let platform = settings_platform();
    let url = settings_url(platform, target).ok_or(WorkerError::new(
        "UNSUPPORTED",
        "This permission does not have a device settings page.",
    ))?;
    window
        .opener()
        .open_url(url, None::<&str>)
        .map_err(|_| WorkerError::new("NOT_READY", "Device settings could not be opened."))?;
    let platform_name = match platform {
        SettingsPlatform::Macos => "macos",
        SettingsPlatform::Windows => "windows",
        SettingsPlatform::Unsupported => "unsupported",
    };
    crate::permission_guide::present(
        window.app_handle(),
        crate::permission_guide::PermissionSettingsGuide::new(platform_name, target),
    )
}

fn settings_url(
    platform: SettingsPlatform,
    target: PermissionSettingsTarget,
) -> Option<&'static str> {
    match (platform, target) {
        (SettingsPlatform::Macos, PermissionSettingsTarget::ScreenCapture) => {
            Some("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
        }
        (SettingsPlatform::Macos, PermissionSettingsTarget::Accessibility) => {
            Some("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        }
        (SettingsPlatform::Macos, PermissionSettingsTarget::Microphone) => {
            Some("x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone")
        }
        (SettingsPlatform::Windows, PermissionSettingsTarget::Microphone) => {
            Some("ms-settings:privacy-microphone")
        }
        _ => None,
    }
}

fn settings_platform() -> SettingsPlatform {
    if cfg!(target_os = "macos") {
        SettingsPlatform::Macos
    } else if cfg!(target_os = "windows") {
        SettingsPlatform::Windows
    } else {
        SettingsPlatform::Unsupported
    }
}

#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn AXIsProcessTrusted() -> bool;
}

#[cfg(target_os = "macos")]
fn mac_projection(
    screen: bool,
    accessibility: bool,
    microphone: objc2_av_foundation::AVAuthorizationStatus,
    requires_relaunch: bool,
) -> DeviceReadiness {
    use objc2_av_foundation::AVAuthorizationStatus;
    let screen_capture = if screen {
        capability(
            CapabilityStatus::Granted,
            true,
            false,
            if requires_relaunch {
                RecoveryAction::Relaunch
            } else {
                RecoveryAction::None
            },
            if requires_relaunch {
                "Screen Recording changed. Relaunch Tro before observing a window."
            } else {
                "Screen Recording is ready."
            },
        )
    } else {
        capability(
            CapabilityStatus::Unknown,
            true,
            false,
            RecoveryAction::ManualSettings,
            "Open Screen Recording settings, add or enable Tro, then recheck.",
        )
    };
    let accessibility = if accessibility {
        capability(
            CapabilityStatus::Granted,
            true,
            false,
            RecoveryAction::None,
            "Accessibility is ready for visual teaching guidance.",
        )
    } else {
        capability(
            CapabilityStatus::Unknown,
            true,
            false,
            RecoveryAction::ManualSettings,
            "Open Accessibility settings, add or enable Tro, then recheck.",
        )
    };
    let microphone = match microphone {
        AVAuthorizationStatus::Authorized => capability(
            CapabilityStatus::Granted,
            false,
            false,
            RecoveryAction::None,
            "Microphone is ready for optional push-to-talk. No audio is retained by this check.",
        ),
        AVAuthorizationStatus::NotDetermined => capability(
            CapabilityStatus::NotDetermined,
            false,
            true,
            RecoveryAction::Request,
            "Microphone setup is optional. Text always remains available.",
        ),
        AVAuthorizationStatus::Denied => capability(
            CapabilityStatus::Denied,
            false,
            false,
            RecoveryAction::ManualSettings,
            "Microphone is off. Use text, or allow Tro in System Settings → Privacy & Security → Microphone.",
        ),
        _ => capability(
            CapabilityStatus::Unavailable,
            false,
            false,
            RecoveryAction::ManualSettings,
            "Microphone access is restricted on this Mac. Text remains available.",
        ),
    };
    DeviceReadiness {
        platform: "macos",
        requires_relaunch,
        message: "Tro checked this Mac without recording screen or audio content.",
        screen_capture,
        accessibility,
        microphone,
    }
}

#[cfg(target_os = "macos")]
unsafe fn mac_status() -> (bool, bool, objc2_av_foundation::AVAuthorizationStatus) {
    use objc2_av_foundation::{AVCaptureDevice, AVMediaTypeAudio};
    unsafe {
        let media_type = AVMediaTypeAudio.expect("AVMediaTypeAudio must exist on macOS");
        (
            CGPreflightScreenCaptureAccess(),
            AXIsProcessTrusted(),
            AVCaptureDevice::authorizationStatusForMediaType(media_type),
        )
    }
}

#[cfg(target_os = "macos")]
async fn platform_check(
    window: &tauri::WebviewWindow,
    settings_target: Option<PermissionSettingsTarget>,
) -> Result<DeviceReadiness, WorkerError> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    window
        .run_on_main_thread(move || {
            let (screen, accessibility, microphone) = unsafe { mac_status() };
            let requires_relaunch =
                settings_target == Some(PermissionSettingsTarget::ScreenCapture) && screen;
            let _ = sender.send(mac_projection(
                screen,
                accessibility,
                microphone,
                requires_relaunch,
            ));
        })
        .map_err(|_| WorkerError::new("NOT_READY", "Device check is unavailable."))?;
    receiver
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Device check was interrupted."))
}

#[cfg(target_os = "macos")]
async fn platform_request(
    window: &tauri::WebviewWindow,
    _kind: PermissionRequestKind,
) -> Result<DeviceReadiness, WorkerError> {
    use block2::RcBlock;
    use objc2_av_foundation::{AVCaptureDevice, AVMediaTypeAudio};
    use std::sync::{Arc, Mutex};
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let sender = Arc::new(Mutex::new(Some(sender)));
    window
        .run_on_main_thread(move || {
            let completion_sender = sender.clone();
            let completion = RcBlock::new(move |_granted| {
                if let Ok(mut guard) = completion_sender.lock()
                    && let Some(sender) = guard.take()
                {
                    let _ = sender.send(());
                }
            });
            unsafe {
                let media_type = AVMediaTypeAudio.expect("AVMediaTypeAudio must exist on macOS");
                AVCaptureDevice::requestAccessForMediaType_completionHandler(
                    media_type,
                    &completion,
                );
            }
        })
        .map_err(|_| WorkerError::new("NOT_READY", "Microphone check is unavailable."))?;
    tokio::time::timeout(std::time::Duration::from_secs(90), receiver)
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Microphone request timed out."))?
        .map_err(|_| WorkerError::new("NOT_READY", "Microphone request was interrupted."))?;
    platform_check(window, None).await
}

#[cfg(target_os = "windows")]
fn windows_projection(microphone: Option<bool>) -> DeviceReadiness {
    let capture_supported =
        windows::Graphics::Capture::GraphicsCaptureSession::IsSupported().unwrap_or(false);
    let screen_capture = if capture_supported {
        capability(
            CapabilityStatus::Available,
            true,
            false,
            RecoveryAction::None,
            "Screen capture is available. The first selected-window Observe will verify access.",
        )
    } else {
        capability(
            CapabilityStatus::Unavailable,
            true,
            false,
            RecoveryAction::Recheck,
            "Windows screen capture is unavailable on this device. Protected or elevated windows may also be unavailable.",
        )
    };
    let microphone = match microphone {
        Some(true) => capability(
            CapabilityStatus::Available,
            false,
            true,
            RecoveryAction::None,
            "The microphone probe succeeded and discarded every sample. Text remains available.",
        ),
        Some(false) => capability(
            CapabilityStatus::Unknown,
            false,
            false,
            RecoveryAction::ManualSettings,
            "The microphone could not be opened. Check Settings → Privacy & security → Microphone and desktop app access, or use text.",
        ),
        None => capability(
            CapabilityStatus::Unknown,
            false,
            true,
            RecoveryAction::Request,
            "Microphone readiness is optional and is checked only when you ask.",
        ),
    };
    DeviceReadiness {
        platform: "windows",
        requires_relaunch: false,
        message: "Windows capture support is a preflight; the first Observe is authoritative.",
        screen_capture,
        accessibility: capability(
            CapabilityStatus::Available,
            false,
            false,
            RecoveryAction::None,
            "Windows does not use a separate macOS-style Accessibility grant.",
        ),
        microphone,
    }
}

#[cfg(target_os = "windows")]
async fn platform_check(
    _window: &tauri::WebviewWindow,
    settings_target: Option<PermissionSettingsTarget>,
) -> Result<DeviceReadiness, WorkerError> {
    let microphone = if settings_target == Some(PermissionSettingsTarget::Microphone) {
        Some(windows_microphone_probe().await)
    } else {
        None
    };
    Ok(windows_projection(microphone))
}

#[cfg(target_os = "windows")]
async fn platform_request(
    _window: &tauri::WebviewWindow,
    _kind: PermissionRequestKind,
) -> Result<DeviceReadiness, WorkerError> {
    Ok(windows_projection(Some(windows_microphone_probe().await)))
}

#[cfg(target_os = "windows")]
async fn windows_microphone_probe() -> bool {
    tokio::task::spawn_blocking(|| {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        let host = cpal::default_host();
        let Some(device) = host.default_input_device() else {
            return false;
        };
        let Ok(config) = device.default_input_config() else {
            return false;
        };
        let sample_format = config.sample_format();
        let Ok(stream) = device.build_input_stream_raw(
            config.into(),
            sample_format,
            |_data, _info| {},
            |_error| {},
            Some(std::time::Duration::from_secs(2)),
        ) else {
            return false;
        };
        if stream.play().is_err() {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(350));
        drop(stream);
        true
    })
    .await
    .unwrap_or(false)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
async fn platform_check(
    _window: &tauri::WebviewWindow,
    _settings_target: Option<PermissionSettingsTarget>,
) -> Result<DeviceReadiness, WorkerError> {
    Ok(unsupported_projection())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
async fn platform_request(
    _window: &tauri::WebviewWindow,
    _kind: PermissionRequestKind,
) -> Result<DeviceReadiness, WorkerError> {
    Ok(unsupported_projection())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn unsupported_projection() -> DeviceReadiness {
    DeviceReadiness {
        platform: "unsupported",
        requires_relaunch: false,
        message: "Device onboarding is supported on macOS and Windows.",
        screen_capture: capability(
            CapabilityStatus::Unavailable,
            true,
            false,
            RecoveryAction::Recheck,
            "Screen observation is unavailable on this platform.",
        ),
        accessibility: capability(
            CapabilityStatus::Unavailable,
            true,
            false,
            RecoveryAction::Recheck,
            "Accessibility readiness is unavailable on this platform.",
        ),
        microphone: capability(
            CapabilityStatus::Unknown,
            false,
            false,
            RecoveryAction::None,
            "Microphone readiness is not checked on this platform.",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_kind_rejects_unknown_values() {
        assert!(serde_json::from_str::<PermissionRequestKind>("\"microphone\"").is_ok());
        assert!(serde_json::from_str::<PermissionRequestKind>("\"screenObservation\"").is_err());
        assert!(serde_json::from_str::<PermissionRequestKind>("\"camera\"").is_err());
    }

    #[test]
    fn settings_target_rejects_unknown_values() {
        assert!(serde_json::from_str::<PermissionSettingsTarget>("\"screenCapture\"").is_ok());
        assert!(serde_json::from_str::<PermissionSettingsTarget>("\"camera\"").is_err());
    }

    #[test]
    fn settings_destinations_are_exact_and_closed() {
        assert_eq!(
            settings_url(
                SettingsPlatform::Macos,
                PermissionSettingsTarget::ScreenCapture
            ),
            Some("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
        );
        assert_eq!(
            settings_url(
                SettingsPlatform::Macos,
                PermissionSettingsTarget::Accessibility
            ),
            Some("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        );
        assert_eq!(
            settings_url(
                SettingsPlatform::Windows,
                PermissionSettingsTarget::Microphone
            ),
            Some("ms-settings:privacy-microphone")
        );
        assert_eq!(
            settings_url(
                SettingsPlatform::Windows,
                PermissionSettingsTarget::ScreenCapture
            ),
            None
        );
    }

    #[test]
    fn public_projection_has_no_native_error_or_media_fields() {
        let projection = DeviceReadiness {
            platform: "unsupported",
            requires_relaunch: false,
            message: "Checked.",
            screen_capture: capability(
                CapabilityStatus::Unavailable,
                true,
                false,
                RecoveryAction::Recheck,
                "Unavailable.",
            ),
            accessibility: capability(
                CapabilityStatus::Unavailable,
                true,
                false,
                RecoveryAction::Recheck,
                "Unavailable.",
            ),
            microphone: capability(
                CapabilityStatus::Unknown,
                false,
                false,
                RecoveryAction::None,
                "Optional.",
            ),
        };
        let value = serde_json::to_value(projection).expect("projection serializes");
        assert_eq!(value.as_object().map(|value| value.len()), Some(6));
        assert!(value.get("rawError").is_none());
        assert!(value.get("audio").is_none());
    }
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
