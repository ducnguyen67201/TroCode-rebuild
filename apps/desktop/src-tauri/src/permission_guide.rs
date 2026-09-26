//! Click-through help shown above native settings. It never receives or posts input.
use crate::permissions::PermissionSettingsTarget;
use crate::worker::WorkerError;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::{Emitter, Manager};

const GUIDE_LABEL: &str = "permission-settings-guide";
const GUIDE_WIDTH: f64 = 394.0;
const GUIDE_HEIGHT: f64 = 286.0;
const GUIDE_POINTER_Y: f64 = 152.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionSettingsGuide {
    platform: &'static str,
    target: PermissionSettingsTarget,
}

impl PermissionSettingsGuide {
    pub fn new(platform: &'static str, target: PermissionSettingsTarget) -> Self {
        Self { platform, target }
    }
}

#[derive(Default)]
pub struct PermissionGuideState {
    current: std::sync::Mutex<Option<PermissionSettingsGuide>>,
    main_lost_focus: AtomicBool,
}

pub fn prepare(app: &tauri::AppHandle) -> Result<(), WorkerError> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        GUIDE_LABEL,
        tauri::WebviewUrl::App("index.html?permissionGuide=1".into()),
    )
    .title("Tro permission guide")
    .inner_size(GUIDE_WIDTH, GUIDE_HEIGHT)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .closable(false)
    .decorations(false)
    .transparent(true)
    .shadow(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .focusable(false)
    .focused(false)
    .visible(false)
    .content_protected(true)
    .build()
    .map_err(|_| WorkerError::new("NOT_READY", "Permission guide unavailable."))?;
    window
        .set_ignore_cursor_events(true)
        .map_err(|_| WorkerError::new("NOT_READY", "Click-through guide unavailable."))?;
    Ok(())
}

pub fn present(app: &tauri::AppHandle, guide: PermissionSettingsGuide) -> Result<(), WorkerError> {
    let state = app.state::<PermissionGuideState>();
    *state
        .current
        .lock()
        .map_err(|_| WorkerError::new("INTERNAL", "Permission guide state unavailable."))? =
        Some(guide);
    state.main_lost_focus.store(false, Ordering::SeqCst);
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        // Give the OS settings window time to open before reading its public bounds.
        tokio::time::sleep(Duration::from_millis(350)).await;
        let still_current = handle
            .state::<PermissionGuideState>()
            .current
            .lock()
            .map(|current| *current == Some(guide))
            .unwrap_or(false);
        if !still_current {
            return;
        }
        let positioned = handle.clone();
        let _ = handle.run_on_main_thread(move || {
            let Some(window) = positioned.get_webview_window(GUIDE_LABEL) else {
                return;
            };
            let _ = position_guide(&positioned, &window);
            let _ = window.set_ignore_cursor_events(true);
            let _ = window.emit("permission-settings-guide", guide);
            let _ = window.show();
        });
    });
    Ok(())
}

pub fn hide(app: &tauri::AppHandle) {
    let state = app.state::<PermissionGuideState>();
    if let Ok(mut current) = state.current.lock() {
        *current = None;
    }
    state.main_lost_focus.store(false, Ordering::SeqCst);
    suspend(app);
}

pub fn suspend(app: &tauri::AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        if let Some(window) = handle.get_webview_window(GUIDE_LABEL) {
            let _ = window.hide();
        }
    });
}

pub fn take_target(app: &tauri::AppHandle) -> Option<PermissionSettingsTarget> {
    let state = app.state::<PermissionGuideState>();
    let target = state.current.lock().ok()?.take().map(|guide| guide.target);
    state.main_lost_focus.store(false, Ordering::SeqCst);
    suspend(app);
    target
}

pub fn main_focus_changed(app: &tauri::AppHandle, focused: bool) {
    let state = app.state::<PermissionGuideState>();
    let has_guide = state
        .current
        .lock()
        .map(|current| current.is_some())
        .unwrap_or(false);
    if !has_guide {
        state.main_lost_focus.store(false, Ordering::SeqCst);
        return;
    }
    if !focused {
        state.main_lost_focus.store(true, Ordering::SeqCst);
    } else if state.main_lost_focus.swap(false, Ordering::SeqCst) {
        // Returning to Tro ends the cross-app hint. Rechecking consumes the target.
        suspend(app);
    }
}

fn position_guide(
    app: &tauri::AppHandle,
    window: &tauri::WebviewWindow,
) -> Result<(), WorkerError> {
    let main = app
        .get_webview_window("main")
        .ok_or(WorkerError::new("NOT_READY", "Main window unavailable."))?;
    let monitor = main
        .current_monitor()
        .map_err(|_| WorkerError::new("NOT_READY", "Display geometry unavailable."))?
        .or_else(|| main.primary_monitor().ok().flatten())
        .ok_or(WorkerError::new(
            "NOT_READY",
            "Display geometry unavailable.",
        ))?;
    let origin = monitor.position();
    let size = monitor.size();
    let scale = monitor.scale_factor();
    let logical_origin = origin.to_logical::<f64>(scale);
    let logical_size = size.to_logical::<f64>(scale);
    let margin = 16.0;
    let fallback_x = logical_origin.x + logical_size.width - GUIDE_WIDTH - margin;
    let fallback_y = logical_origin.y + margin;
    #[cfg(target_os = "macos")]
    let (x, y) = system_settings_bounds()
        .map(|bounds| {
            let target_y = bounds.y + bounds.height * 0.56;
            (bounds.x + bounds.width + 8.0, target_y - GUIDE_POINTER_Y)
        })
        .unwrap_or((fallback_x, fallback_y));
    #[cfg(not(target_os = "macos"))]
    let (x, y) = (fallback_x, fallback_y);
    let x = x.clamp(
        logical_origin.x + margin,
        logical_origin.x + logical_size.width - GUIDE_WIDTH - margin,
    );
    let y = y.clamp(
        logical_origin.y + margin,
        logical_origin.y + logical_size.height - GUIDE_HEIGHT - margin,
    );
    window
        .set_position(tauri::LogicalPosition::new(x, y))
        .map_err(|_| WorkerError::new("NOT_READY", "Permission guide position unavailable."))
}

#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug)]
struct WindowBounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[cfg(target_os = "macos")]
fn system_settings_bounds() -> Option<WindowBounds> {
    use core_foundation::base::{CFType, TCFType};
    use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
    use core_foundation::string::{CFString, CFStringRef};
    use core_graphics::geometry::CGRect;
    use core_graphics::window::{
        copy_window_info, kCGNullWindowID, kCGWindowBounds, kCGWindowListExcludeDesktopElements,
        kCGWindowListOptionOnScreenOnly, kCGWindowOwnerName,
    };

    let windows = copy_window_info(
        kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements,
        kCGNullWindowID,
    )?;
    let owner_key = unsafe { CFString::wrap_under_get_rule(kCGWindowOwnerName as CFStringRef) };
    let bounds_key = unsafe { CFString::wrap_under_get_rule(kCGWindowBounds as CFStringRef) };
    let mut best: Option<WindowBounds> = None;
    for raw in windows.iter() {
        let value = unsafe { CFType::wrap_under_get_rule(*raw) };
        let dictionary = value.downcast::<CFDictionary>()?;
        let typed = unsafe {
            CFDictionary::<CFString, CFType>::wrap_under_get_rule(
                dictionary.as_concrete_TypeRef() as CFDictionaryRef
            )
        };
        let Some(owner) = typed
            .find(&owner_key)
            .and_then(|value| value.downcast::<CFString>())
        else {
            continue;
        };
        if owner != "System Settings" {
            continue;
        }
        let Some(bounds_dictionary) = typed
            .find(&bounds_key)
            .and_then(|value| value.downcast::<CFDictionary>())
        else {
            continue;
        };
        let Some(rect) = CGRect::from_dict_representation(&bounds_dictionary) else {
            continue;
        };
        if rect.size.width < 300.0 || rect.size.height < 300.0 {
            continue;
        }
        let candidate = WindowBounds {
            x: rect.origin.x,
            y: rect.origin.y,
            width: rect.size.width,
            height: rect.size.height,
        };
        if best
            .map(|current| candidate.width * candidate.height > current.width * current.height)
            .unwrap_or(true)
        {
            best = Some(candidate);
        }
    }
    best
}

#[tauri::command]
pub fn permission_settings_guide_current(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, PermissionGuideState>,
) -> Option<PermissionSettingsGuide> {
    if window.label() != GUIDE_LABEL {
        return None;
    }
    *state.current.lock().ok()?
}
