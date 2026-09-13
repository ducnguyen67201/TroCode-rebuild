#[cfg(feature = "desktop")]
mod commands;
pub mod config;
pub mod lifecycle;
pub mod manager;
pub mod worker;
#[cfg(feature = "desktop")]
pub fn run() {
    use std::sync::Arc;
    use tauri::{Emitter, Manager};
    let runtime = Arc::new(manager::RuntimeManager::new(
        config::development_program().expect("P1 bundled runtime not configured"),
    ));
    let app = tauri::Builder::default()
        .manage(runtime.clone())
        .invoke_handler(tauri::generate_handler![
            commands::runtime_start,
            commands::runtime_health,
            commands::runtime_stop,
            commands::runtime_status,
            commands::runtime_restart,
            commands::account_select
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            let mut status = runtime.subscribe();
            tauri::async_runtime::spawn(async move {
                while status.changed().await.is_ok() {
                    let value = status.borrow_and_update().clone();
                    let _ = handle.emit_to("main", "runtime-status", value);
                }
            });
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
