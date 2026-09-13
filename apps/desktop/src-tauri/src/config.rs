use crate::worker::{WorkerError, WorkerProgram};
use std::path::PathBuf;
pub fn development_program() -> Result<WorkerProgram, WorkerError> {
    if !cfg!(debug_assertions) {
        return Err(WorkerError::new(
            "NOT_READY",
            "Bundled runtime requires P1 packaging.",
        ));
    }
    let directory =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../services/teaching-runtime");
    let executable = directory.join(if cfg!(windows) {
        ".venv/Scripts/python.exe"
    } else {
        ".venv/bin/python"
    });
    Ok(WorkerProgram {
        executable,
        args: vec!["-m".into(), "tro_runtime".into()],
        directory,
    })
}

/// Installed bundles resolve only inside their own resources; never fall back to system Python.
pub fn bundled_program(resources: &std::path::Path) -> Result<WorkerProgram, WorkerError> {
    let directory = resources.join("runtime/tro-runtime");
    let executable = directory.join(if cfg!(windows) {
        "tro-runtime.exe"
    } else {
        "tro-runtime"
    });
    let manifest = resources.join("runtime/manifest.json");
    if !executable.is_file() || !manifest.is_file() {
        return Err(WorkerError::new(
            "NOT_READY",
            "Bundled teaching runtime is missing. Reinstall Tro.",
        ));
    }
    let value: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&manifest)
            .map_err(|_| WorkerError::new("NOT_READY", "Runtime manifest unavailable."))?,
    )
    .map_err(|_| WorkerError::new("NOT_READY", "Runtime manifest is invalid."))?;
    let platform = if cfg!(windows) { "win32" } else { "darwin" };
    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x64"
    };
    if value["version"] != 1
        || value["platform"] != platform
        || value["arch"] != arch
        || value["protocolDigest"] != tro_contracts::SCHEMA_DIGEST
    {
        return Err(WorkerError::new(
            "PROTOCOL_MISMATCH",
            "Bundled runtime is incompatible. Reinstall Tro.",
        ));
    }
    Ok(WorkerProgram {
        executable,
        args: vec![],
        directory,
    })
}
