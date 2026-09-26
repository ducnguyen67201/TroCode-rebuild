pub mod account;
pub mod auth;
#[cfg(feature = "desktop")]
mod commands;
pub mod config;
pub mod geometry;
pub mod lifecycle;
pub mod manager;
pub mod modifier_chord;
#[cfg(feature = "desktop")]
mod overlay;
#[cfg(feature = "desktop")]
mod permissions;
pub mod voice;
pub mod worker;
#[cfg(feature = "desktop")]
pub fn run() {
    use std::sync::Arc;
    use tauri::{Emitter, Manager};

    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().with_handler(|app, _, event| {
            if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                overlay::hide(app);
                let manager=app.state::<Arc<manager::RuntimeManager>>().inner().clone();
                let voice=app.state::<Arc<voice::VoiceManager>>().inner().clone();
                tauri::async_runtime::spawn(async move { voice.cancel(Some(&manager)).await; manager.stop().await; });
            }
        }).build())
        .manage(overlay::OverlayState::default())
        .invoke_handler(tauri::generate_handler![
            commands::auth_status,
            commands::auth_sign_in_google,
            commands::auth_retry,
            commands::auth_sign_out,
            commands::workspace_members,
            commands::workspace_add_member,
            commands::workspace_remove_member,
            commands::runtime_start,
            commands::runtime_health,
            commands::runtime_stop,
            commands::runtime_status,
            commands::runtime_restart,
            commands::account_select,
            commands::teaching_request,
            commands::proof_connect,
            commands::voice_status,
            commands::voice_enable,
            commands::voice_disable,
            commands::voice_execute_text,
            commands::voice_cancel,
            overlay::overlay_current,
            permissions::observation_permissions
        ])
        .setup(move |app| {
            let program=if cfg!(debug_assertions) { config::development_program()? }
                else { config::bundled_program(&app.path().resource_dir()?)? };
            let runtime=Arc::new(manager::RuntimeManager::new(program));
            app.manage(runtime.clone());
            let auth = Arc::new(auth::AuthManager::from_env(runtime.clone()));
            app.manage(auth.clone());
            let voice = Arc::new(voice::VoiceManager::default());
            app.manage(voice.clone());
            let (chord_sender, chord_receiver) = std::sync::mpsc::sync_channel(8);
            let listener = Arc::new(modifier_chord::ModifierListener::start(chord_sender)
                .map_err(std::io::Error::other)?);
            app.manage(listener.clone());
            overlay::prepare(app.handle())?;
            use tauri_plugin_global_shortcut::GlobalShortcutExt;
            app.global_shortcut().register("CommandOrControl+Shift+Escape")
                .map_err(|_| "Emergency Stop shortcut is unavailable; resolve the shortcut conflict before starting Tro.")?;
            let handle = app.handle().clone();
            let mut status = runtime.subscribe();
            tauri::async_runtime::spawn(async move {
                while status.changed().await.is_ok() {
                    let value = status.borrow_and_update().clone();
                    if value.state != "running" { overlay::hide(&handle); }
                    let _ = handle.emit_to("main", "runtime-status", value);
                }
            });
            let handle = app.handle().clone();
            let mut voice_status = voice.subscribe();
            tauri::async_runtime::spawn(async move {
                while voice_status.changed().await.is_ok() {
                    let value = voice_status.borrow_and_update().clone();
                    let _ = handle.emit_to("main", "voice-status", &value);
                    overlay::show_voice(&handle, &value);
                }
            });
            let chord_handle = app.handle().clone();
            std::thread::Builder::new().name("tro-voice-chord-dispatch".into()).spawn(move || {
                while let Ok(edge) = chord_receiver.recv() {
                    let handle = chord_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let voice = handle.state::<Arc<voice::VoiceManager>>().inner().clone();
                        match edge {
                            modifier_chord::ChordEdge::Pressed(_) => {
                                let runtime = handle.state::<Arc<manager::RuntimeManager>>().inner().clone();
                                let auth = handle.state::<Arc<auth::AuthManager>>().inner().clone();
                                let _ = voice.begin_capture(handle.clone(), runtime, auth).await;
                            }
                            modifier_chord::ChordEdge::Released => voice.release_capture().await,
                        }
                    });
                }
            }).map_err(std::io::Error::other)?;
            let handle = app.handle().clone();
            let mut status = auth.subscribe();
            tauri::async_runtime::spawn(async move {
                let mut auto_arm = voice::AutoArmGate::default();
                while status.changed().await.is_ok() {
                    let value = status.borrow_and_update().clone();
                    let authenticated = value.state == "authenticated";
                    let should_arm = auto_arm.update(authenticated);
                    if !authenticated {
                        overlay::hide(&handle);
                        let _ = handle.state::<Arc<modifier_chord::ModifierListener>>().set_enabled(false);
                        let voice = handle.state::<Arc<voice::VoiceManager>>().inner().clone();
                        let runtime = handle.state::<Arc<manager::RuntimeManager>>().inner().clone();
                        voice.disable_due_to_auth_loss();
                        tauri::async_runtime::spawn(async move {
                            voice.cancel(Some(&runtime)).await;
                        });
                    } else if should_arm {
                        let voice = handle.state::<Arc<voice::VoiceManager>>().inner().clone();
                        if voice.auto_arm_allowed() {
                            let listener = handle.state::<Arc<modifier_chord::ModifierListener>>().inner().clone();
                            let runtime = handle.state::<Arc<manager::RuntimeManager>>().inner().clone();
                            let _ = commands::arm_voice_control(voice, listener, runtime).await;
                        }
                    }
                    let _ = handle.emit_to("main", "auth-status", value);
                }
            });
            let restore = auth.clone();
            tauri::async_runtime::spawn(async move {
                restore.restore().await;
            });
            tauri::async_runtime::spawn(auth.refresh_loop());
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("desktop initialization failed");
    let cleanup_started = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let cleanup_complete = Arc::new(std::sync::atomic::AtomicBool::new(false));
    app.run(move |handle, event| {
        use std::sync::atomic::Ordering;
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            if cleanup_complete.load(Ordering::SeqCst) {
                return;
            }
            api.prevent_exit();
            if cleanup_started.swap(true, Ordering::SeqCst) {
                return;
            }
            let handle = handle.clone();
            let complete = cleanup_complete.clone();
            tauri::async_runtime::spawn(async move {
                let manager = handle
                    .state::<Arc<manager::RuntimeManager>>()
                    .inner()
                    .clone();
                let voice = handle.state::<Arc<voice::VoiceManager>>().inner().clone();
                voice.cancel(Some(&manager)).await;
                manager.stop().await;
                complete.store(true, Ordering::SeqCst);
                handle.exit(0);
            });
        }
    });
}
