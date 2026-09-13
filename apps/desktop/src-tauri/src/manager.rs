use crate::{
    lifecycle::Status,
    worker::{Worker, WorkerError, WorkerProgram},
};
use serde_json::json;
use std::time::Duration;
use tokio::sync::{Mutex, watch};
use uuid::Uuid;

pub struct RuntimeManager {
    // Serializes lifecycle operations only; the worker actor never holds this lock.
    lifecycle: Mutex<Option<Worker>>,
    program: WorkerProgram,
    status: watch::Sender<Status>,
    account: Mutex<Option<String>>,
    selection: std::sync::atomic::AtomicU64,
}
impl RuntimeManager {
    pub fn new(program: WorkerProgram) -> Self {
        let (status, _) = watch::channel(Status::default());
        Self {
            lifecycle: Mutex::new(None),
            program,
            status,
            account: Mutex::new(None),
            selection: std::sync::atomic::AtomicU64::new(0),
        }
    }
    pub fn subscribe(&self) -> watch::Receiver<Status> {
        self.status.subscribe()
    }
    pub fn status(&self) -> Status {
        self.status.borrow().clone()
    }
    fn update(&self, state: &'static str, generation: Option<String>, message: &'static str) {
        self.status
            .send_modify(|status| status.transition(state, generation, message));
    }
    pub async fn start(&self) -> Result<Status, WorkerError> {
        let mut guard = self.lifecycle.lock().await;
        if let Some(worker) = guard.as_ref()
            && !worker.has_ended()
        {
            return Ok(self.status());
        }
        self.update("starting", None, "Starting private runtime…");
        let account = self.account.lock().await.clone();
        match Worker::launch(&self.program, account.as_deref()).await {
            Ok(worker) => {
                let generation = worker.generation.to_string();
                if let Err(error) = worker
                    .request(
                        "start",
                        json!({"sessionId": Uuid::new_v4().to_string()}),
                        Duration::from_secs(2),
                    )
                    .await
                {
                    worker.shutdown().await;
                    self.update("failed", None, error.message);
                    return Err(error);
                }
                self.update(
                    "running",
                    Some(generation.clone()),
                    "Diagnostic runtime is running.",
                );
                let mut exited = worker.exit_status();
                let status = self.status.clone();
                tokio::spawn(async move {
                    while !*exited.borrow_and_update() {
                        if exited.changed().await.is_err() {
                            break;
                        }
                    }
                    status.send_modify(|value| {
                        if value.generation_id.as_deref() == Some(&generation)
                            && value.state == "running"
                        {
                            value.transition(
                                "failed",
                                Some(generation.clone()),
                                "Runtime exited. Restart explicitly.",
                            );
                        }
                    });
                });
                *guard = Some(worker);
                Ok(self.status())
            }
            Err(error) => {
                self.update("failed", None, error.message);
                Err(error)
            }
        }
    }
    pub async fn health(&self) -> Result<Status, WorkerError> {
        let worker = self
            .lifecycle
            .lock()
            .await
            .clone()
            .ok_or(WorkerError::new("NOT_READY", "Start the runtime first."))?;
        worker
            .request("health", json!({}), Duration::from_secs(2))
            .await?;
        Ok(self.status())
    }
    pub async fn stop(&self) -> Status {
        let mut guard = self.lifecycle.lock().await;
        self.stop_locked(&mut guard).await;
        self.status()
    }
    async fn stop_locked(&self, guard: &mut Option<Worker>) {
        self.update("stopping", self.status().generation_id, "Stopping runtime…");
        if let Some(worker) = guard.take() {
            let _ = worker
                .request("stop", json!({}), Duration::from_secs(1))
                .await;
            worker.shutdown().await;
        }
        self.update("stopped", None, "Runtime stopped.");
    }
    pub async fn begin_account_change(&self) -> u64 {
        let mut guard = self.lifecycle.lock().await;
        let ticket = self
            .selection
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            + 1;
        self.stop_locked(&mut guard).await;
        *self.account.lock().await = None;
        ticket
    }
    pub async fn finish_account_change(
        &self,
        ticket: u64,
        account: Option<String>,
    ) -> Result<(), WorkerError> {
        let mut guard = self.lifecycle.lock().await;
        if self.selection.load(std::sync::atomic::Ordering::SeqCst) != ticket {
            return Err(WorkerError::new(
                "UNAUTHORIZED",
                "Account selection was superseded.",
            ));
        }
        self.stop_locked(&mut guard).await;
        *self.account.lock().await = account;
        Ok(())
    }
    pub async fn select_account(&self, account: Option<String>) {
        let ticket = self.begin_account_change().await;
        let _ = self.finish_account_change(ticket, account).await;
    }
}
