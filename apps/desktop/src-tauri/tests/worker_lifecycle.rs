use serde_json::json;
use std::{path::PathBuf, time::Duration};
use tro_desktop::{config::development_program, manager::RuntimeManager, worker::Worker};
#[tokio::test]
async fn real_worker_round_trip_and_account_generation() {
    let program = development_program().unwrap();
    let manager = RuntimeManager::new(program);
    let (first, second) = tokio::join!(manager.start(), manager.start());
    assert_eq!(first.unwrap().generation_id, second.unwrap().generation_id);
    let old = manager.health().await.unwrap().generation_id;
    manager.select_account(Some("account-b".into())).await;
    assert_eq!(manager.status().state, "stopped");
    let next = manager.start().await.unwrap();
    assert_ne!(old, next.generation_id);
    manager.stop().await;
    assert_eq!(manager.status().state, "stopped");
}
fn fixture(mode: &str) -> tro_desktop::worker::WorkerProgram {
    let mut program = development_program().unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tests/fixtures/workers/adversarial.py");
    program.args = vec![script.to_string_lossy().into_owned(), mode.into()];
    program
}
#[tokio::test]
async fn refuses_wrong_digest() {
    assert!(Worker::launch(&fixture("digest"), None).await.is_err());
}
#[tokio::test]
async fn crash_is_not_replayed() {
    let worker = Worker::launch(&fixture("crash"), None).await.unwrap();
    assert!(
        worker
            .request("health", json!({}), Duration::from_secs(1))
            .await
            .is_err()
    );
    worker.shutdown().await;
    assert!(worker.has_ended());
}
#[tokio::test]
async fn stale_generation_does_not_fulfill_request() {
    let worker = Worker::launch(&fixture("stale"), None).await.unwrap();
    let response = worker
        .request("health", json!({}), Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(response["generationId"], worker.generation.to_string());
    worker.shutdown().await;
}
#[tokio::test]
async fn hung_shutdown_is_reaped() {
    let worker = Worker::launch(&fixture("hang"), None).await.unwrap();
    tokio::time::timeout(Duration::from_secs(8), worker.shutdown())
        .await
        .unwrap();
    assert!(worker.has_ended());
}
#[tokio::test]
async fn oversized_frame_fails_boundedly() {
    let worker = Worker::launch(&fixture("oversize"), None).await.unwrap();
    assert!(
        worker
            .request("health", json!({}), Duration::from_secs(1))
            .await
            .is_err()
    );
    worker.shutdown().await;
}
#[tokio::test]
async fn ordinary_load_leaves_shutdown_available() {
    let worker = Worker::launch(&development_program().unwrap(), None)
        .await
        .unwrap();
    let mut tasks = Vec::new();
    for _ in 0..64 {
        let worker = worker.clone();
        tasks.push(tokio::spawn(async move {
            let _ = worker
                .request("health", json!({}), Duration::from_secs(1))
                .await;
        }));
    }
    worker.shutdown().await;
    for task in tasks {
        task.await.unwrap();
    }
    assert!(worker.has_ended());
}
#[tokio::test]
async fn superseded_account_resolution_cannot_override_new_selection() {
    let manager = RuntimeManager::new(development_program().unwrap());
    let first = manager.begin_account_change().await;
    let second = manager.begin_account_change().await;
    manager
        .finish_account_change(second, Some("account-b".into()))
        .await
        .unwrap();
    assert!(
        manager
            .finish_account_change(first, Some("account-a".into()))
            .await
            .is_err()
    );
}
#[tokio::test]
async fn handles_script_paths_containing_spaces() {
    let directory = tempfile::Builder::new()
        .prefix("tro worker with spaces ")
        .tempdir()
        .unwrap();
    let script = directory.path().join("worker script.py");
    std::fs::write(
        &script,
        include_str!("../../../../tests/fixtures/workers/adversarial.py"),
    )
    .unwrap();
    let mut program = development_program().unwrap();
    program.args = vec![script.to_string_lossy().into_owned(), "normal".into()];
    let worker = Worker::launch(&program, None).await.unwrap();
    worker.shutdown().await;
    assert!(worker.has_ended());
}
#[tokio::test]
async fn worker_does_not_inherit_arbitrary_parent_environment() {
    assert!(std::env::var_os("CARGO_MANIFEST_DIR").is_some());
    let worker = Worker::launch(&fixture("environment"), None).await.unwrap();
    assert!(
        worker
            .request("health", json!({}), Duration::from_secs(1))
            .await
            .is_ok()
    );
    worker.shutdown().await;
}
