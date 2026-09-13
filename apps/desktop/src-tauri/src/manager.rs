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
    proof: std::sync::atomic::AtomicBool,
    epoch: std::sync::atomic::AtomicU64,
    bootstrap: Mutex<()>,
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
            proof: std::sync::atomic::AtomicBool::new(false),
            epoch: std::sync::atomic::AtomicU64::new(0),
            bootstrap: Mutex::new(()),
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
        let ticket = self.epoch.load(std::sync::atomic::Ordering::SeqCst);
        let _bootstrap = self.bootstrap.lock().await;
        if self.epoch.load(std::sync::atomic::Ordering::SeqCst) != ticket {
            return Err(WorkerError::new(
                "NOT_READY",
                "Runtime start was cancelled.",
            ));
        }
        let guard = self.lifecycle.lock().await;
        if let Some(worker) = guard.as_ref()
            && !worker.has_ended()
        {
            return Ok(self.status());
        }
        drop(guard);
        self.update("starting", None, "Starting private runtime…");
        let account = self.account.lock().await.clone();
        match Worker::launch(&self.program, account.as_deref()).await {
            Ok(worker) => {
                if self.epoch.load(std::sync::atomic::Ordering::SeqCst) != ticket {
                    worker.shutdown().await;
                    return Err(WorkerError::new(
                        "NOT_READY",
                        "Runtime start was cancelled.",
                    ));
                }
                let generation = worker.generation.to_string();
                let session = Uuid::new_v4().to_string();
                if let Err(error) = worker
                    .request(
                        "start",
                        json!({"sessionId": session}),
                        Duration::from_secs(2),
                    )
                    .await
                {
                    worker.shutdown().await;
                    if self.epoch.load(std::sync::atomic::Ordering::SeqCst) == ticket {
                        self.update("failed", None, error.message);
                    }
                    return Err(error);
                }
                if let Some(account) = account
                    .as_deref()
                    .filter(|value| Uuid::parse_str(value).is_ok())
                {
                    let configure = async {
                        let model = if self.proof.load(std::sync::atomic::Ordering::SeqCst) {
                            crate::account::model_context(account, &session).await?
                        } else { serde_json::Value::Null };
                        worker.request("configure",json!({"storageRoot":crate::account::storage_root()?,"modelConfig":model}),Duration::from_secs(3)).await
                    }.await;
                    if let Err(error) = configure {
                        worker.shutdown().await;
                        self.update("failed", None, error.message);
                        return Err(error);
                    }
                }
                let mut guard = self.lifecycle.lock().await;
                if self.epoch.load(std::sync::atomic::Ordering::SeqCst) != ticket {
                    drop(guard);
                    worker.shutdown().await;
                    return Err(WorkerError::new(
                        "NOT_READY",
                        "Runtime start was cancelled.",
                    ));
                }
                self.update(
                    "running",
                    Some(generation.clone()),
                    "Teaching runtime is running.",
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
                if self.epoch.load(std::sync::atomic::Ordering::SeqCst) == ticket {
                    self.update("failed", None, error.message);
                }
                Err(error)
            }
        }
    }
    pub async fn teaching(
        &self,
        kind: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, WorkerError> {
        if !matches!(
            kind,
            "listTargets"
                | "selectTarget"
                | "observe"
                | "explain"
                | "check"
                | "presentationAck"
                | "ask"
                | "refreshCue"
        ) {
            return Err(WorkerError::new(
                "FORBIDDEN",
                "Unsupported teaching request.",
            ));
        }
        let worker = self
            .lifecycle
            .lock()
            .await
            .clone()
            .ok_or(WorkerError::new("NOT_READY", "Start the runtime first."))?;
        let generation = worker.generation.to_string();
        let result = worker
            .request(
                kind,
                payload,
                Duration::from_secs(if kind == "ask" { 50 } else { 15 }),
            )
            .await?;
        if self.status().generation_id.as_deref() != Some(&generation)
            || self.status().state != "running"
        {
            return Err(WorkerError::new(
                "NOT_READY",
                "Teaching session was stopped.",
            ));
        }
        Ok(result["state"].clone())
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
        self.epoch.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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
        self.epoch.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut guard = self.lifecycle.lock().await;
        let ticket = self
            .selection
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            + 1;
        self.stop_locked(&mut guard).await;
        *self.account.lock().await = None;
        self.proof.store(false, std::sync::atomic::Ordering::SeqCst);
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
    pub async fn connect_proof(&self) -> Result<Status, WorkerError> {
        let ticket = self.begin_account_change().await;
        let account = crate::account::identity().await?;
        self.finish_account_change(ticket, Some(account)).await?;
        let _guard = self.lifecycle.lock().await;
        if self.selection.load(std::sync::atomic::Ordering::SeqCst) != ticket {
            return Err(WorkerError::new(
                "UNAUTHORIZED",
                "Account selection was superseded.",
            ));
        }
        self.proof.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(self.status())
    }
    pub async fn select_account(&self, account: Option<String>) {
        let ticket = self.begin_account_change().await;
        let _ = self.finish_account_change(ticket, account).await;
    }
}
