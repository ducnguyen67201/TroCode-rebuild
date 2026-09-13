//! An actor owns the child, pipes, deadlines and pending requests. No shared child mutex.
use serde_json::{Value, json};
use std::{collections::HashMap, path::PathBuf, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::{mpsc, oneshot, watch},
    time::{Instant, timeout},
};
use tro_contracts::{MAX_FRAME_BYTES, SCHEMA_DIGEST, parse_message};
use uuid::Uuid;

#[derive(Debug, Clone, thiserror::Error, serde::Serialize)]
#[error("{message}")]
pub struct WorkerError {
    pub code: &'static str,
    pub message: &'static str,
}
impl WorkerError {
    pub fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
    fn exited() -> Self {
        Self::new("WORKER_EXITED", "Runtime disconnected. Restart explicitly.")
    }
}
type Reply = Result<Value, WorkerError>;
struct Request {
    value: Value,
    deadline: Instant,
    reply: oneshot::Sender<Reply>,
}
struct Pending {
    deadline: Instant,
    correlation: Value,
    expected: String,
    reply: oneshot::Sender<Reply>,
}
#[derive(Clone)]
pub struct Worker {
    ordinary: mpsc::Sender<Request>,
    control: mpsc::Sender<Request>,
    pub generation: Uuid,
    ended: watch::Receiver<bool>,
}
#[derive(Clone)]
pub struct WorkerProgram {
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub directory: PathBuf,
}

impl Worker {
    pub async fn launch(
        program: &WorkerProgram,
        account: Option<&str>,
    ) -> Result<Self, WorkerError> {
        let mut command = Command::new(&program.executable);
        // Forward only OS/runtime necessities; provider credentials stay out of the worker.
        command.env_clear();
        for name in [
            "PATH",
            "SYSTEMROOT",
            "WINDIR",
            "TEMP",
            "TMP",
            "HOME",
            "USERPROFILE",
            "LANG",
            "LC_ALL",
            "APPDATA",
            "LOCALAPPDATA",
            "CUA_DRIVER_POLICY_FILE",
            "CUA_DRIVER_MANAGED_POLICY_FILE",
        ] {
            if let Some(value) = std::env::var_os(name) {
                command.env(name, value);
            }
        }
        command.env("PYTHONUTF8", "1").env("PYTHONUNBUFFERED", "1");
        command
            .args(&program.args)
            .current_dir(&program.directory)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            // Worker diagnostics must never leak unvalidated stderr into UI/logs.
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true);
        let child = command.spawn().map_err(|_| {
            WorkerError::new(
                "NOT_READY",
                "Python runtime unavailable. Run npm run setup.",
            )
        })?;
        let (ordinary, rx) = mpsc::channel(32);
        let (control, controls) = mpsc::channel(4);
        let (end_tx, ended) = watch::channel(false);
        let generation = Uuid::new_v4();
        tokio::spawn(drive(child, rx, controls, end_tx, generation));
        let worker = Self {
            ordinary,
            control,
            generation,
            ended,
        };
        let result = worker
            .request(
                "initialize",
                json!({"schemaDigest": SCHEMA_DIGEST, "accountId": account}),
                Duration::from_secs(4),
            )
            .await;
        match result {
            Ok(value)
                if value["schemaDigest"] == SCHEMA_DIGEST
                    && value["capabilities"] == json!(["diagnostic"]) =>
            {
                Ok(worker)
            }
            _ => {
                worker.shutdown().await;
                Err(WorkerError::new(
                    "PROTOCOL_MISMATCH",
                    "Runtime handshake failed. Check setup and protocol versions.",
                ))
            }
        }
    }
    pub fn exit_status(&self) -> watch::Receiver<bool> {
        self.ended.clone()
    }
    pub fn has_ended(&self) -> bool {
        *self.ended.borrow()
    }
    pub async fn request(&self, kind: &str, payload: Value, duration: Duration) -> Reply {
        if self.has_ended() {
            return Err(WorkerError::exited());
        }
        let mut value = json!({"protocolVersion": 2, "kind": format!("runtime.{kind}"), "requestId": Uuid::new_v4().to_string(), "correlationId": Uuid::new_v4().to_string(), "generationId": self.generation.to_string()});
        if let (Some(target), Some(source)) = (value.as_object_mut(), payload.as_object()) {
            if source.keys().any(|key| {
                matches!(
                    key.as_str(),
                    "protocolVersion" | "kind" | "requestId" | "correlationId" | "generationId"
                )
            }) {
                return Err(WorkerError::new(
                    "INVALID_MESSAGE",
                    "Request payload cannot replace protocol identity.",
                ));
            }
            target.extend(source.clone());
        }
        parse_message(&serde_json::to_vec(&value).map_err(|_| WorkerError::exited())?)
            .map_err(|_| WorkerError::new("INVALID_MESSAGE", "Invalid runtime request."))?;
        let (reply, result) = oneshot::channel();
        let channel = if matches!(kind, "stop" | "shutdown") {
            &self.control
        } else {
            &self.ordinary
        };
        channel
            .try_send(Request {
                value,
                deadline: Instant::now() + duration,
                reply,
            })
            .map_err(|_| WorkerError::new("BUSY", "Runtime queue is full or closed."))?;
        timeout(duration + Duration::from_millis(100), result)
            .await
            .map_err(|_| WorkerError::new("TIMEOUT", "Runtime response timed out."))?
            .map_err(|_| WorkerError::exited())?
    }
    pub async fn shutdown(&self) {
        let _ = self
            .request("shutdown", json!({}), Duration::from_secs(3))
            .await;
        let mut ended = self.ended.clone();
        let _ = timeout(Duration::from_secs(4), async {
            while !*ended.borrow_and_update() {
                if ended.changed().await.is_err() {
                    break;
                }
            }
        })
        .await;
    }
}

async fn read_frame(
    reader: &mut BufReader<tokio::process::ChildStdout>,
) -> Result<Vec<u8>, WorkerError> {
    let mut frame = Vec::new();
    loop {
        let buffer = reader.fill_buf().await.map_err(|_| WorkerError::exited())?;
        if buffer.is_empty() {
            return Err(WorkerError::exited());
        }
        let newline = buffer.iter().position(|byte| *byte == b'\n');
        let length = newline.map_or(buffer.len(), |position| position + 1);
        if frame.len() + length > MAX_FRAME_BYTES + 1 {
            return Err(WorkerError::new(
                "INVALID_MESSAGE",
                "Runtime frame exceeds limit.",
            ));
        }
        frame.extend_from_slice(&buffer[..length]);
        reader.consume(length);
        if newline.is_some() {
            frame.pop();
            return Ok(frame);
        }
    }
}

fn expected_response(kind: &str) -> Option<&'static str> {
    match kind {
        "runtime.initialize" => Some("runtime.ready"),
        "runtime.health" => Some("runtime.healthResult"),
        "runtime.start" => Some("runtime.started"),
        "runtime.stop" => Some("runtime.stopped"),
        "runtime.shutdown" => Some("runtime.shutdownComplete"),
        "runtime.listTargets" => Some("runtime.targetsResult"),
        "runtime.selectTarget" => Some("runtime.targetSelected"),
        "runtime.observe" => Some("runtime.observationResult"),
        "runtime.explain" => Some("runtime.explanationResult"),
        "runtime.check" => Some("runtime.checkResult"),
        "runtime.presentationAck" => Some("runtime.presentationAckResult"),
        "runtime.configure" => Some("runtime.configured"),
        "runtime.ask" => Some("runtime.askResult"),
        "runtime.planControl" => Some("runtime.planControlResult"),
        "runtime.refreshCue" => Some("runtime.cueRefreshResult"),
        _ => None,
    }
}

async fn drive(
    mut child: Child,
    mut ordinary: mpsc::Receiver<Request>,
    mut control: mpsc::Receiver<Request>,
    ended: watch::Sender<bool>,
    generation: Uuid,
) {
    let mut input = child.stdin.take().expect("piped stdin");
    let output = child.stdout.take().expect("piped stdout");
    let (frames, mut incoming) = mpsc::channel(32);
    let reader = tokio::spawn(async move {
        let mut reader = BufReader::new(output);
        loop {
            let frame = read_frame(&mut reader).await;
            let failed = frame.is_err();
            if frames.send(frame).await.is_err() || failed {
                break;
            }
        }
    });
    let mut pending: HashMap<String, Pending> = HashMap::new();
    let mut tick = tokio::time::interval(Duration::from_millis(25));
    let mut shutting_down: Option<Instant> = None;
    loop {
        tokio::select! {
            biased;
            request = control.recv(), if !control.is_closed() || !control.is_empty() => {
                if let Some(request) = request {
                    if request.value["kind"] == "runtime.shutdown" { shutting_down = Some(Instant::now() + Duration::from_secs(3)); }
                    if send_request(request, &mut input, &mut pending, true).await.is_err() { break; }
                }
            }
            frame = incoming.recv() => {
                let Some(Ok(frame)) = frame else { break; };
                let Ok(value) = parse_message(&frame) else { break; };
                if value["generationId"] != generation.to_string() { continue; }
                let Some(id) = value["requestId"].as_str() else { break; };
                if let Some(waiter) = pending.remove(id) {
                    if waiter.correlation != value["correlationId"] || (value["kind"] != waiter.expected && value["kind"] != "runtime.error") {
                        let _ = waiter.reply.send(Err(WorkerError::new("INVALID_MESSAGE", "Runtime response identity mismatch.")));
                        break;
                    }
                    let result = if value["kind"] == "runtime.error" { Err(WorkerError::new("NOT_READY", "Runtime refused the request.")) } else { Ok(value) };
                    let _ = waiter.reply.send(result);
                }
            }
            request = ordinary.recv(), if shutting_down.is_none() && (!ordinary.is_closed() || !ordinary.is_empty()) => {
                if let Some(request) = request && send_request(request, &mut input, &mut pending, false).await.is_err() { break; }
            }
            _ = tick.tick() => {
                let now = Instant::now();
                let expired: Vec<_> = pending.iter().filter(|(_, entry)| entry.deadline <= now).map(|(id, _)| id.clone()).collect();
                for id in expired { if let Some(waiter) = pending.remove(&id) { let _ = waiter.reply.send(Err(WorkerError::new("TIMEOUT", "Runtime response timed out."))); } }
                if shutting_down.is_some_and(|deadline| now >= deadline) || (ordinary.is_closed() && control.is_closed()) { break; }
                if child.try_wait().ok().flatten().is_some() { break; }
            }
        }
    }
    drop(input);
    reader.abort();
    let _ = child.kill().await;
    let _ = child.wait().await;
    for (_, waiter) in pending {
        let _ = waiter.reply.send(Err(WorkerError::exited()));
    }
    let _ = ended.send(true);
}

async fn send_request(
    request: Request,
    input: &mut tokio::process::ChildStdin,
    pending: &mut HashMap<String, Pending>,
    control: bool,
) -> Result<(), ()> {
    if request.reply.is_closed() || request.deadline <= Instant::now() {
        return Ok(());
    }
    let id = request.value["requestId"].as_str().ok_or(())?.to_owned();
    if pending.contains_key(&id) || (!control && pending.len() >= 32) {
        let _ = request
            .reply
            .send(Err(WorkerError::new("BUSY", "Too many runtime requests.")));
        return Ok(());
    }
    let expected = expected_response(request.value["kind"].as_str().ok_or(())?)
        .ok_or(())?
        .to_owned();
    let mut bytes = serde_json::to_vec(&request.value).map_err(|_| ())?;
    bytes.push(b'\n');
    if timeout(Duration::from_millis(200), input.write_all(&bytes))
        .await
        .map_err(|_| ())?
        .is_err()
    {
        return Err(());
    }
    pending.insert(
        id,
        Pending {
            deadline: request.deadline,
            correlation: request.value["correlationId"].clone(),
            expected,
            reply: request.reply,
        },
    );
    Ok(())
}
