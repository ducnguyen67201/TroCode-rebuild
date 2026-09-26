fn main() {
    #[cfg(feature = "desktop")]
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "app_relaunch",
            "auth_status",
            "auth_sign_in_google",
            "auth_retry",
            "auth_sign_out",
            "workspace_members",
            "workspace_add_member",
            "workspace_remove_member",
            "runtime_start",
            "runtime_health",
            "runtime_stop",
            "runtime_status",
            "runtime_restart",
            "account_select",
            "teaching_request",
            "proof_connect",
            "overlay_current",
            "voice_status",
            "voice_enable",
            "voice_disable",
            "voice_execute_text",
            "voice_cancel",
            "voice_decide",
            "permission_settings_guide_current",
            "device_readiness",
            "device_permission_request",
            "device_permission_settings",
        ]),
    ))
    .expect("Tauri build configuration");
}
