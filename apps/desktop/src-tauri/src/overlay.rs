//! Host-owned, expiring visual presentation. These windows never receive input.
use crate::worker::WorkerError;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{Emitter, Manager};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PresentationOutcome {
    Presented,
    Waiting,
    Superseded,
}

#[cfg(debug_assertions)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct GuidanceDebugShape {
    status: String,
    grounding: String,
    cue: bool,
    cue_grounding: String,
    journey_index: u64,
    step_count: usize,
    observation_complete: bool,
    element_count: usize,
    readiness_observation: String,
    readiness_accessibility: String,
    readiness_screen: String,
    readiness_model: String,
    readiness_reason: String,
}

#[cfg(debug_assertions)]
fn guidance_debug_shape(state: &Value) -> GuidanceDebugShape {
    GuidanceDebugShape {
        status: state["journey"]["status"].as_str().unwrap_or("none").into(),
        grounding: state["journey"]["grounding"]
            .as_str()
            .unwrap_or("unknown")
            .into(),
        cue: !state["cue"].is_null(),
        cue_grounding: state["cue"]["grounding"].as_str().unwrap_or("none").into(),
        journey_index: state["journey"]["index"].as_u64().unwrap_or(0),
        step_count: state["journey"]["steps"].as_array().map_or(0, Vec::len),
        observation_complete: state["observation"]["complete"].as_bool().unwrap_or(false),
        element_count: state["observation"]["elements"]
            .as_array()
            .map_or(0, Vec::len),
        readiness_observation: state["readiness"]["observation"]
            .as_str()
            .unwrap_or("unknown")
            .into(),
        readiness_accessibility: state["readiness"]["accessibility"]
            .as_str()
            .unwrap_or("unknown")
            .into(),
        readiness_screen: state["readiness"]["screen"]
            .as_str()
            .unwrap_or("unknown")
            .into(),
        readiness_model: state["readiness"]["model"]
            .as_str()
            .unwrap_or("unknown")
            .into(),
        readiness_reason: state["readiness"]["reason"]
            .as_str()
            .unwrap_or("unknown")
            .into(),
    }
}

#[cfg(debug_assertions)]
fn log_guidance_state(stage: &str, state: &Value, outcome: PresentationOutcome) {
    let shape = guidance_debug_shape(state);
    log::debug!(
        target: "tro::guidance",
        "tro diagnostic: guidance_state stage={stage} status={} grounding={} cue={} cue_grounding={} journey_index={} step_count={} observation_complete={} element_count={} readiness_observation={} readiness_accessibility={} readiness_screen={} readiness_model={} readiness_reason={} outcome={outcome:?}",
        shape.status,
        shape.grounding,
        shape.cue,
        shape.cue_grounding,
        shape.journey_index,
        shape.step_count,
        shape.observation_complete,
        shape.element_count,
        shape.readiness_observation,
        shape.readiness_accessibility,
        shape.readiness_screen,
        shape.readiness_model,
        shape.readiness_reason,
    );
}

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
    let companion = tauri::WebviewWindowBuilder::new(
        app,
        "cursor-companion",
        tauri::WebviewUrl::App("index.html?cursorCompanion=1".into()),
    )
    .title("Tro cursor companion")
    .inner_size(1.0, 1.0)
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
    .map_err(|_| WorkerError::new("NOT_READY", "Cursor companion unavailable."))?;
    companion.set_ignore_cursor_events(true).map_err(|_| {
        WorkerError::new("NOT_READY", "Click-through cursor companion unavailable.")
    })?;
    update_companion(app, &companion);
    companion
        .show()
        .map_err(|_| WorkerError::new("NOT_READY", "Cursor companion unavailable."))?;
    track_companion(app.clone());
    let hud = tauri::WebviewWindowBuilder::new(
        app,
        "voice-hud",
        tauri::WebviewUrl::App("index.html?voiceHud=1".into()),
    )
    .title("Tro voice status")
    .inner_size(92.0, 40.0)
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
    position_voice_hud(app, &hud);
    Ok(())
}

fn update_companion(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    let Ok(cursor) = app.cursor_position() else {
        return;
    };
    let Ok(Some(monitor)) = app.monitor_from_point(cursor.x, cursor.y) else {
        return;
    };
    let Some(projection) = companion_projection(
        cursor.x,
        cursor.y,
        f64::from(monitor.position().x),
        f64::from(monitor.position().y),
        f64::from(monitor.size().width),
        f64::from(monitor.size().height),
        monitor.scale_factor(),
    ) else {
        return;
    };
    let monitor_position = monitor.position();
    let monitor_size = monitor.size();
    if window.outer_position().ok().as_ref() != Some(monitor_position) {
        let _ = window.set_position(*monitor_position);
    }
    if window.outer_size().ok().as_ref() != Some(monitor_size) {
        let _ = window.set_size(*monitor_size);
    }
    if let Ok(mut current) = app.state::<OverlayState>().companion.lock() {
        *current = Some(projection.clone());
    }
    let _ = window.emit("cursor-companion-position", projection);
}

fn companion_projection(
    cursor_x: f64,
    cursor_y: f64,
    monitor_x: f64,
    monitor_y: f64,
    monitor_width: f64,
    monitor_height: f64,
    scale: f64,
) -> Option<Value> {
    if !cursor_x.is_finite()
        || !cursor_y.is_finite()
        || !monitor_x.is_finite()
        || !monitor_y.is_finite()
        || !monitor_width.is_finite()
        || !monitor_height.is_finite()
        || !scale.is_finite()
        || monitor_width <= 0.0
        || monitor_height <= 0.0
        || scale <= 0.0
    {
        return None;
    }
    Some(serde_json::json!({
        "x": (cursor_x - monitor_x) / scale,
        "y": (cursor_y - monitor_y) / scale,
        "width": monitor_width / scale,
        "height": monitor_height / scale
    }))
}

fn track_companion(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut previous = None;
        loop {
            tokio::time::sleep(Duration::from_millis(16)).await;
            let Ok(cursor) = app.cursor_position() else {
                continue;
            };
            let current = (cursor.x, cursor.y);
            if previous.is_some_and(|position: (f64, f64)| {
                (position.0 - current.0).abs() < 0.25 && (position.1 - current.1).abs() < 0.25
            }) {
                continue;
            }
            previous = Some(current);
            let Some(window) = app.get_webview_window("cursor-companion") else {
                break;
            };
            update_companion(&app, &window);
        }
    });
}

fn position_voice_hud(app: &tauri::AppHandle, hud: &tauri::WebviewWindow) {
    let monitor = app
        .get_webview_window("main")
        .and_then(|main| main.current_monitor().ok().flatten())
        .or_else(|| hud.current_monitor().ok().flatten());
    let Some(monitor) = monitor else {
        let _ = hud.center();
        return;
    };
    let scale = monitor.scale_factor();
    let logical_width = 92.0_f64;
    let physical_width = (logical_width * scale).round().max(1.0) as u32;
    let top_margin = (18.0_f64 * scale).round() as i32;
    let monitor_position = monitor.position();
    let monitor_size = monitor.size();
    let x = monitor_position.x + monitor_size.width.saturating_sub(physical_width) as i32 / 2;
    let y = monitor_position.y + top_margin;
    let _ = hud.set_position(tauri::PhysicalPosition::new(x, y));
}

pub fn hide(app: &tauri::AppHandle) {
    hide_windows(app, true);
}

fn hide_guidance(app: &tauri::AppHandle) {
    hide_windows(app, false);
}

fn hide_windows(app: &tauri::AppHandle, include_voice: bool) {
    app.state::<OverlayState>()
        .epoch
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        if let Ok(mut values) = handle.state::<OverlayState>().values.lock() {
            values.clear();
        }
        for (label, window) in handle.webview_windows() {
            if label.starts_with("teaching-overlay-") || (include_voice && label == "voice-hud") {
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
        let visible = voice_hud_visible(&status.phase);
        if visible {
            position_voice_hud(&handle, &window);
            let _ = window.emit("voice-hud-status", serde_json::json!({"phase":status.phase,"revision":status.revision,"message":status.message}));
            let _ = window.show();
        } else {
            let _ = window.hide();
        }
    });
}

fn voice_hud_visible(phase: &str) -> bool {
    matches!(
        phase,
        "listening" | "transcribing" | "dispatching" | "planning" | "guiding" | "failed"
    )
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
) -> Result<PresentationOutcome, WorkerError> {
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
) -> Result<PresentationOutcome, WorkerError> {
    let overlay = app.state::<OverlayState>();
    if overlay.epoch.load(std::sync::atomic::Ordering::SeqCst) != expected_epoch {
        return Ok(PresentationOutcome::Superseded);
    }
    let cue = &state["cue"];
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.emit("teaching-state", state);
    }
    if cue.is_null() {
        if let Ok(mut values) = overlay.values.lock() {
            values.clear();
        }
        for (label, window) in app.webview_windows() {
            if label.starts_with("teaching-overlay-") {
                let _ = window.hide();
            }
        }
        return Ok(PresentationOutcome::Waiting);
    }
    if !crate::geometry::grounded(state) {
        return Err(WorkerError::new(
            "GUIDANCE_UNAVAILABLE",
            "The guidance target changed before it could be shown. Try again.",
        ));
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let expires = cue["expires_at"].as_f64().unwrap_or(0.0);
    if expires <= now || expires - now > 1.0 {
        return Err(WorkerError::new(
            "GUIDANCE_UNAVAILABLE",
            "The guidance became stale before it could be shown. Try again.",
        ));
    }
    let main = app
        .get_webview_window("main")
        .ok_or(WorkerError::new("NOT_READY", "Main window unavailable."))?;
    let monitors = main
        .available_monitors()
        .map_err(|_| WorkerError::new("NOT_READY", "Display geometry unavailable."))?;
    let mut presented = false;
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
        let payload = serde_json::json!({
            "state": state,
            "origin": {"x": origin_x, "y": origin_y},
            "units": units
        });
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
        presented = true;
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
    if !presented {
        return Err(WorkerError::new(
            "GUIDANCE_UNAVAILABLE",
            "Guidance cannot be shown on the selected display yet.",
        ));
    }
    Ok(PresentationOutcome::Presented)
}

#[derive(Default)]
pub struct OverlayState {
    values: std::sync::Mutex<std::collections::HashMap<String, Value>>,
    companion: std::sync::Mutex<Option<Value>>,
    epoch: std::sync::atomic::AtomicU64,
}

#[cfg(test)]
mod tests {
    use super::{companion_projection, guidance_debug_shape, voice_hud_visible};
    use serde_json::json;

    #[test]
    fn voice_hud_exists_only_for_guidance_or_failure_states() {
        for phase in [
            "listening",
            "transcribing",
            "dispatching",
            "planning",
            "guiding",
            "failed",
        ] {
            assert!(voice_hud_visible(phase));
        }
        for phase in ["disabled", "idle", "completed", "cancelled"] {
            assert!(!voice_hud_visible(phase));
        }
    }

    #[test]
    fn companion_projects_the_pointer_into_its_fixed_display_layer() {
        assert_eq!(
            companion_projection(200.0, 100.0, 0.0, 0.0, 2880.0, 1800.0, 2.0),
            Some(json!({"x":100.0,"y":50.0,"width":1440.0,"height":900.0}))
        );
        assert_eq!(
            companion_projection(-1800.0, 40.0, -1920.0, 0.0, 1920.0, 1080.0, 1.0),
            Some(json!({"x":120.0,"y":40.0,"width":1920.0,"height":1080.0}))
        );
        assert_eq!(
            companion_projection(1.0, 1.0, 0.0, 0.0, 100.0, 100.0, 0.0),
            None
        );
    }

    #[test]
    fn guidance_diagnostics_report_only_bounded_structure() {
        let state = json!({
            "journey": {
                "status": "paused",
                "grounding": "target_missing",
                "index": 0,
                "steps": ["private caption"],
                "message": "private message"
            },
            "cue": null,
            "observation": {
                "complete": true,
                "elements": [{"label": "private label"}]
            },
            "readiness": {
                "observation": "available",
                "accessibility": "available",
                "screen": "available",
                "model": "ready",
                "reason": "none"
            }
        });

        let shape = guidance_debug_shape(&state);

        assert_eq!(shape.grounding, "target_missing");
        assert_eq!(shape.element_count, 1);
        assert!(!format!("{shape:?}").contains("private"));
    }
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

#[tauri::command]
pub fn cursor_companion_current(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, OverlayState>,
) -> Option<Value> {
    if window.label() != "cursor-companion" {
        return None;
    }
    state.companion.lock().ok()?.clone()
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
    guidance_id: Option<String>,
) {
    tauri::async_runtime::spawn(async move {
        #[cfg(debug_assertions)]
        let mut previous_debug_shape = None;
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
                    let outcome = match present(&app, &state, expected_epoch).await {
                        Ok(PresentationOutcome::Presented) => {
                            app.state::<std::sync::Arc<crate::voice::VoiceManager>>()
                                .guidance_presented(&state);
                            PresentationOutcome::Presented
                        }
                        Ok(PresentationOutcome::Waiting) => PresentationOutcome::Waiting,
                        Ok(PresentationOutcome::Superseded) => break,
                        Err(error) => {
                            app.state::<std::sync::Arc<crate::voice::VoiceManager>>()
                                .guidance_failed(guidance_id.as_deref(), error.message);
                            hide(&app);
                            break;
                        }
                    };
                    #[cfg(debug_assertions)]
                    {
                        let current = guidance_debug_shape(&state);
                        if previous_debug_shape.as_ref() != Some(&current) {
                            log_guidance_state("refresh", &state, outcome);
                            previous_debug_shape = Some(current);
                        }
                    }
                    if state["journey"]["status"].as_str() == Some("completed") {
                        app.state::<std::sync::Arc<crate::voice::VoiceManager>>()
                            .guidance_completed(state["journey"]["id"].as_str());
                    }
                    if !should_track(&state) {
                        if outcome == PresentationOutcome::Waiting {
                            app.state::<std::sync::Arc<crate::voice::VoiceManager>>()
                                .guidance_failed(
                                    guidance_id.as_deref(),
                                    "No visible guidance is ready. Try asking again.",
                                );
                        }
                        hide_guidance(&app);
                        break;
                    }
                }
                _ => {
                    app.state::<std::sync::Arc<crate::voice::VoiceManager>>()
                        .guidance_failed(
                            guidance_id.as_deref(),
                            "Guidance could not refresh. Try again.",
                        );
                    hide(&app);
                    break;
                }
            }
        }
    });
}

/// Present a validated teaching projection from any input path, then keep it fresh.
pub async fn present_guidance(
    app: &tauri::AppHandle,
    manager: std::sync::Arc<crate::manager::RuntimeManager>,
    state: &Value,
) -> Result<PresentationOutcome, WorkerError> {
    hide_guidance(app);
    let expected_epoch = epoch(app);
    let outcome = present(app, state, expected_epoch).await?;
    #[cfg(debug_assertions)]
    log_guidance_state("initial", state, outcome);
    if should_track(state) {
        track(
            app.clone(),
            manager,
            expected_epoch,
            state["journey"]["id"].as_str().map(str::to_owned),
        );
    } else if outcome == PresentationOutcome::Waiting {
        return Err(WorkerError::new(
            "GUIDANCE_UNAVAILABLE",
            "No visible guidance is ready. Try asking again.",
        ));
    }
    Ok(outcome)
}
