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
