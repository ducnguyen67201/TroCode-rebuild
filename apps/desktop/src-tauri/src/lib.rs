pub mod account;
pub mod auth;
#[cfg(feature = "desktop")]
mod commands;
pub mod config;
pub mod geometry;
pub mod lifecycle;
pub mod manager;
#[cfg(feature = "desktop")]
mod overlay;
#[cfg(feature = "desktop")]
mod permissions;
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
                tauri::async_runtime::spawn(async move { manager.stop().await; });
            }
        }).build())
        .manage(overlay::OverlayState::default())
        .invoke_handler(tauri::generate_handler![
            commands::auth_status,
            commands::auth_sign_in_google,
            commands::auth_retry,
            commands::auth_sign_out,
            commands::runtime_start,
            commands::runtime_health,
            commands::runtime_stop,
            commands::runtime_status,
            commands::runtime_restart,
            commands::account_select,
            commands::teaching_request,
            commands::proof_connect,
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
            let mut status = auth.subscribe();
            tauri::async_runtime::spawn(async move {
                while status.changed().await.is_ok() {
                    let value = status.borrow_and_update().clone();
                    if value.state != "authenticated" {
                        overlay::hide(&handle);
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
                handle.state::<Arc<manager::RuntimeManager>>().stop().await;
                complete.store(true, Ordering::SeqCst);
                handle.exit(0);
            });
        }
    });
}
