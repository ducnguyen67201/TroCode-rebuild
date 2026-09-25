//! Host-owned, expiring visual presentation. These windows never receive input.
use crate::worker::WorkerError;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{Emitter, Manager};

/// Create hidden WebViews in setup, before event handlers can contend with WebView2.
pub fn prepare(app: &tauri::AppHandle) -> Result<(), WorkerError> {
    let main = app
        .get_webview_window("main")
        .ok_or(WorkerError::new("NOT_READY", "Main window unavailable."))?;
    let monitors = main
        .available_monitors()
        .map_err(|_| WorkerError::new("NOT_READY", "Display geometry unavailable."))?;
    for index in 0..monitors.len() {
        let window = tauri::WebviewWindowBuilder::new(
            app,
            format!("teaching-overlay-{index}"),
            tauri::WebviewUrl::App("index.html?overlay=1".into()),
        )
        .title("Tro visual guidance")
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
        .map_err(|_| WorkerError::new("NOT_READY", "Visual overlay unavailable."))?;
        window.set_ignore_cursor_events(true).map_err(|_| {
            WorkerError::new("NOT_READY", "Click-through presentation unavailable.")
        })?;
    }
    let hud = tauri::WebviewWindowBuilder::new(
        app,
        "voice-hud",
        tauri::WebviewUrl::App("index.html?voiceHud=1".into()),
    )
    .title("Tro voice status")
    .inner_size(360.0, 84.0)
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
    .map_err(|_| WorkerError::new("NOT_READY", "Voice status display unavailable."))?;
    hud.set_ignore_cursor_events(true)
        .map_err(|_| WorkerError::new("NOT_READY", "Voice status display unavailable."))?;
    let _ = hud.center();
    Ok(())
}

pub fn hide(app: &tauri::AppHandle) {
    app.state::<OverlayState>()
        .epoch
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        if let Ok(mut values) = handle.state::<OverlayState>().values.lock() {
            values.clear();
        }
        for (label, window) in handle.webview_windows() {
            if label.starts_with("teaching-overlay-") || label == "voice-hud" {
                let _ = window.hide();
            }
        }
    });
}

pub fn show_voice(app: &tauri::AppHandle, status: &crate::voice::VoiceStatus) {
    let handle = app.clone();
    let status = status.clone();
    let _ = app.run_on_main_thread(move || {
        let Some(window) = handle.get_webview_window("voice-hud") else { return; };
        let visible = matches!(status.phase.as_str(), "listening" | "transcribing" | "dispatching" | "executing" | "confirmation" | "failed");
        if visible {
            let _ = window.emit("voice-hud-status", serde_json::json!({"phase":status.phase,"revision":status.revision,"message":status.message}));
            let _ = window.show();
        } else {
            let _ = window.hide();
        }
    });
}

pub fn epoch(app: &tauri::AppHandle) -> u64 {
    app.state::<OverlayState>()
        .epoch
        .load(std::sync::atomic::Ordering::SeqCst)
}
pub async fn present(
    app: &tauri::AppHandle,
    state: &Value,
    expected_epoch: u64,
) -> Result<(), WorkerError> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let handle = app.clone();
    let state = state.clone();
    app.run_on_main_thread(move || {
        let _ = sender.send(present_on_main(&handle, &state, expected_epoch));
    })
    .map_err(|_| WorkerError::new("NOT_READY", "Presentation unavailable."))?;
    receiver
        .await
        .map_err(|_| WorkerError::new("NOT_READY", "Presentation interrupted."))?
}
fn present_on_main(
    app: &tauri::AppHandle,
    state: &Value,
    expected_epoch: u64,
) -> Result<(), WorkerError> {
    let overlay = app.state::<OverlayState>();
    if overlay.epoch.load(std::sync::atomic::Ordering::SeqCst) != expected_epoch {
        return Ok(());
    }
    let cue = &state["cue"];
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.emit("teaching-state", state);
    }
    if cue.is_null() || !crate::geometry::grounded(state) {
        if let Ok(mut values) = overlay.values.lock() {
            values.clear();
        }
        for (label, window) in app.webview_windows() {
            if label.starts_with("teaching-overlay-") {
                let _ = window.hide();
            }
        }
        return Ok(());
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let expires = cue["expires_at"].as_f64().unwrap_or(0.0);
    if expires <= now || expires - now > 1.0 {
        return Ok(());
    }
    let main = app
        .get_webview_window("main")
        .ok_or(WorkerError::new("NOT_READY", "Main window unavailable."))?;
    let monitors = main
        .available_monitors()
        .map_err(|_| WorkerError::new("NOT_READY", "Display geometry unavailable."))?;
    for (index, monitor) in monitors.into_iter().enumerate() {
        let scale = monitor.scale_factor();
        if !scale.is_finite() || scale <= 0.0 {
            continue;
        }
        let position = monitor.position();
        let size = monitor.size();
        // macOS AX uses desktop points; Windows UIA uses physical desktop pixels.
        // Mixed-scale macOS global origins require platform calibration; fail closed.
        if cfg!(target_os = "macos") && (position.x != 0 || position.y != 0) {
            continue;
        }
        let units = if cfg!(target_os = "macos") {
            1.0
        } else {
            scale
        };
        let origin_x = f64::from(position.x)
            / if cfg!(target_os = "macos") {
                scale
            } else {
                1.0
            };
        let origin_y = f64::from(position.y)
            / if cfg!(target_os = "macos") {
                scale
            } else {
                1.0
            };
        let label = format!("teaching-overlay-{index}");
        let Some(window) = app.get_webview_window(&label) else {
            continue;
        };
        window.set_ignore_cursor_events(true).map_err(|_| {
            WorkerError::new("NOT_READY", "Click-through presentation unavailable.")
        })?;
        window
            .set_position(*position)
            .map_err(|_| WorkerError::new("NOT_READY", "Display position unavailable."))?;
        window
            .set_size(*size)
            .map_err(|_| WorkerError::new("NOT_READY", "Display size unavailable."))?;
        let payload = serde_json::json!({"state": state, "origin": {"x": origin_x, "y": origin_y}, "units": units});
        // Store the validated projection so a newly loaded WebView cannot miss its first cue.
        app.state::<OverlayState>()
            .values
            .lock()
            .map_err(|_| WorkerError::new("INTERNAL", "Overlay state unavailable."))?
            .insert(label.clone(), payload.clone());
        window
            .emit("teaching-cue", payload)
            .map_err(|_| WorkerError::new("NOT_READY", "Presentation unavailable."))?;
        window
            .show()
            .map_err(|_| WorkerError::new("NOT_READY", "Presentation unavailable."))?;
        let app = app.clone();
        let cue_id = cue["id"].clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(Duration::from_secs_f64((expires - now).clamp(0.0, 1.0))).await;
            let handle = app.clone();
            let _ = app.run_on_main_thread(move || {
                let expired = if let Ok(mut states) = handle.state::<OverlayState>().values.lock() {
                    if states.get(&label).is_some_and(|value| {
                        value["state"]["cue"]["id"] == cue_id
                            && value["state"]["cue"]["expires_at"] == expires
                    }) {
                        states.remove(&label);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                if expired && let Some(window) = handle.get_webview_window(&label) {
                    let _ = window.hide();
                }
            });
        });
    }
    Ok(())
}
#[derive(Default)]
pub struct OverlayState {
    values: std::sync::Mutex<std::collections::HashMap<String, Value>>,
    epoch: std::sync::atomic::AtomicU64,
}

#[tauri::command]
pub fn overlay_current(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, OverlayState>,
) -> Option<Value> {
    if !window.label().starts_with("teaching-overlay-") {
        return None;
    }
    state.values.lock().ok()?.get(window.label()).cloned()
}

pub fn should_track(state: &Value) -> bool {
    !state["cue"].is_null()
        || matches!(
            state["journey"]["status"].as_str(),
            Some("running" | "awaiting_confirmation")
        )
}

/// Read-only geometry/content refresh. A newer request or Stop invalidates this loop.
pub fn track(
    app: tauri::AppHandle,
    manager: std::sync::Arc<crate::manager::RuntimeManager>,
    expected_epoch: u64,
) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;
            if epoch(&app) != expected_epoch {
                break;
            }
            let result = manager.teaching("refreshCue", serde_json::json!({})).await;
            if epoch(&app) != expected_epoch {
                break;
            }
            match result {
                Ok(state) => {
                    if present(&app, &state, expected_epoch).await.is_err() {
                        hide(&app);
                        break;
                    }
                    if !should_track(&state) {
                        break;
                    }
                }
                _ => {
                    hide(&app);
                    break;
                }
            }
        }
    });
}
