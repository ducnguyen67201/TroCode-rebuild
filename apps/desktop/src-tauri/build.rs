fn main() {
    #[cfg(feature = "desktop")]
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "runtime_start",
            "runtime_health",
            "runtime_stop",
            "runtime_status",
            "runtime_restart",
            "account_select",
            "teaching_request",
            "proof_connect",
            "overlay_current",
            "observation_permissions",
        ]),
    ))
    .expect("Tauri build configuration");
}
