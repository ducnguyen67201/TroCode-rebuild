#[cfg(feature = "desktop")]
pub mod audio;
pub mod chunks;
pub mod transcript;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::watch;
#[cfg(feature = "desktop")]
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoicePermissions {
    pub microphone: String,
    pub keyboard_monitoring: String,
    pub ready: bool,
    pub recovery: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceConfirmation {
    pub confirmation_id: String,
    pub summary: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceStatus {
    pub phase: String,
    pub revision: u64,
    pub utterance_id: Option<String>,
    pub run_id: Option<String>,
    pub partial_transcript: String,
    pub final_transcript: String,
    pub target_title: Option<String>,
    pub message: String,
    pub shortcut: String,
    pub permissions: VoicePermissions,
    pub confirmation: Option<VoiceConfirmation>,
    pub actions_used: u8,
}

impl Default for VoiceStatus {
    fn default() -> Self {
        Self {
            phase: "disabled".into(),
            revision: 0,
            utterance_id: None,
            run_id: None,
            partial_transcript: String::new(),
            final_transcript: String::new(),
            target_title: None,
            message: "Enable voice control to use push-to-talk.".into(),
            shortcut: if cfg!(target_os = "macos") {
                "Command+Control"
            } else if cfg!(target_os = "windows") {
                "Left Control+Left Alt"
            } else {
                "Unavailable"
            }
            .into(),
            permissions: VoicePermissions {
                microphone: "unknown".into(),
                keyboard_monitoring: "unknown".into(),
                ready: false,
                recovery: String::new(),
            },
            confirmation: None,
            actions_used: 0,
        }
    }
}

pub struct VoiceManager {
    status: watch::Sender<VoiceStatus>,
    #[cfg(feature = "desktop")]
    capture: Mutex<Option<ActiveCapture>>,
}

#[cfg(feature = "desktop")]
struct ActiveCapture {
    release: Option<oneshot::Sender<()>>,
    task: tokio::task::JoinHandle<()>,
}

impl Default for VoiceManager {
    fn default() -> Self {
        let (status, _) = watch::channel(VoiceStatus::default());
        Self {
            status,
            #[cfg(feature = "desktop")]
            capture: Mutex::new(None),
        }
    }
}

impl VoiceManager {
    pub fn status(&self) -> VoiceStatus {
        self.status.borrow().clone()
    }

    pub fn subscribe(&self) -> watch::Receiver<VoiceStatus> {
        self.status.subscribe()
    }

    pub fn enable(&self, permissions: VoicePermissions) -> VoiceStatus {
        self.status.send_modify(|status| {
            status.revision += 1;
            status.permissions = permissions;
            status.phase = if status.permissions.ready {
                "idle"
            } else {
                "failed"
            }
            .into();
            status.message = if status.permissions.ready {
                format!("Hold {} to speak.", status.shortcut)
            } else {
                "Voice permissions are required.".into()
            };
        });
        self.status()
    }

    #[cfg(feature = "desktop")]
    fn transition(&self, phase: &str, message: &str) {
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = phase.to_owned();
            status.message = message.to_owned();
            if phase != "confirmation" {
                status.confirmation = None;
            }
        });
    }

    pub fn begin_listening(&self) -> Result<Uuid, crate::worker::WorkerError> {
        if !matches!(
            self.status.borrow().phase.as_str(),
            "idle" | "completed" | "cancelled" | "failed"
        ) || !self.status.borrow().permissions.ready
        {
            return Err(crate::worker::WorkerError::new(
                "VOICE_NOT_READY",
                "Voice control is not ready.",
            ));
        }
        let id = Uuid::new_v4();
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "listening".into();
            status.utterance_id = Some(id.to_string());
            status.run_id = None;
            status.partial_transcript.clear();
            status.final_transcript.clear();
            status.confirmation = None;
            status.actions_used = 0;
            status.message = "Listening…".into();
        });
        Ok(id)
    }

    pub async fn cancel(&self, runtime: Option<&crate::manager::RuntimeManager>) -> VoiceStatus {
        #[cfg(feature = "desktop")]
        if let Some(active) = self.capture.lock().await.take() {
            if let Some(release) = active.release {
                let _ = release.send(());
            }
            active.task.abort();
        }
        if let Some(runtime) = runtime {
            let run_id = self
                .status
                .borrow()
                .run_id
                .as_deref()
                .and_then(|value| Uuid::parse_str(value).ok());
            let _ = runtime.cancel_action(run_id).await;
        }
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "cancelled".into();
            status.message = "Voice instruction cancelled.".into();
            status.confirmation = None;
        });
        self.status()
    }

    pub fn disable(&self) -> VoiceStatus {
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "disabled".into();
            status.utterance_id = None;
            status.run_id = None;
            status.partial_transcript.clear();
            status.final_transcript.clear();
            status.target_title = None;
            status.confirmation = None;
            status.message = "Voice control is disabled.".into();
        });
        self.status()
    }

    pub async fn execute_text(
        &self,
        instruction: String,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<VoiceStatus, crate::worker::WorkerError> {
        if instruction.trim().is_empty() || instruction.len() > 2_000 {
            return Err(crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "A short instruction is required.",
            ));
        }
        if matches!(
            self.status.borrow().phase.as_str(),
            "listening" | "transcribing" | "dispatching" | "executing" | "confirmation"
        ) {
            return Err(crate::worker::WorkerError::new(
                "BUSY",
                "A voice instruction is already active.",
            ));
        }
        runtime.start().await?;
        let utterance_id = Uuid::new_v4();
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "dispatching".into();
            status.utterance_id = Some(utterance_id.to_string());
            status.partial_transcript.clear();
            status.final_transcript = instruction.clone();
            status.message = "Preparing the selected window…".into();
        });
        let (prepared, grant) = tokio::join!(
            runtime.prepare_instruction(utterance_id),
            auth.provider_grant(utterance_id, false),
        );
        let prepared = prepared?;
        let (grant, origin) = grant?;
        let preparation_id = prepared["preparationId"]
            .as_str()
            .and_then(|value| Uuid::parse_str(value).ok())
            .ok_or(crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "Runtime returned an invalid preparation.",
            ))?;
        let target_title = prepared["target"]["title"].as_str().map(str::to_owned);
        let run_id = runtime
            .execute_instruction(
                utterance_id,
                preparation_id,
                &instruction,
                serde_json::json!({"origin":origin,"grant":grant.grant,"model":grant.model}),
            )
            .await?;
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "executing".into();
            status.run_id = Some(run_id.to_string());
            status.target_title = target_title;
            status.message = "Working in the selected window…".into();
        });
        Ok(self.status())
    }

    pub fn apply_action_event(&self, event: &serde_json::Value) {
        let Some(run_id) = event["runId"].as_str() else {
            return;
        };
        if self.status.borrow().run_id.as_deref() != Some(run_id) {
            return;
        }
        let phase = event["phase"].as_str().unwrap_or("failed");
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = phase.to_owned();
            status.actions_used = event["actionsUsed"].as_u64().unwrap_or(0).min(12) as u8;
            status.message = event["summary"]
                .as_str()
                .unwrap_or("Action status changed.")
                .to_owned();
            status.confirmation = match (
                event["confirmationId"].as_str(),
                event["confirmationReason"].as_str(),
            ) {
                (Some(id), Some(summary)) => Some(VoiceConfirmation {
                    confirmation_id: id.to_owned(),
                    summary: summary.to_owned(),
                }),
                _ => None,
            };
        });
    }

    #[cfg(feature = "desktop")]
    pub async fn begin_capture(
        self: &Arc<Self>,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<(), crate::worker::WorkerError> {
        if self.capture.lock().await.is_some() {
            return Ok(());
        }
        let utterance_id = self.begin_listening()?;
        let mut microphone = audio::MicrophoneCapture::start().map_err(|_| {
            crate::worker::WorkerError::new(
                "VOICE_PERMISSION_REQUIRED",
                "Microphone access is required.",
            )
        })?;
        runtime.start().await?;
        let sample_rate = microphone.sample_rate;
        let (release, mut released) = oneshot::channel();
        let manager = self.clone();
        let task = tokio::spawn(async move {
            let (prepared, voice_grant, action_grant) = tokio::join!(
                runtime.prepare_instruction(utterance_id),
                auth.provider_grant(utterance_id, true),
                auth.provider_grant(utterance_id, false),
            );
            let outcome = async {
                let prepared = prepared?;
                let (voice_grant, _) = voice_grant?;
                let (action_grant, origin) = action_grant?;
                let mut chunks = chunks::ChunkAssembler::new(sample_rate).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone format is unavailable."))?;
                let mut transcript = transcript::TranscriptAssembler::default();
                let mut expected = 0_u32;
                let mut pending =
                    std::collections::VecDeque::<chunks::WavChunk>::new();
                let mut uploads = tokio::task::JoinSet::new();
                let mut release_seen = false;
                loop {
                    while uploads.len() < transcript::MAX_IN_FLIGHT
                        && let Some(chunk) = pending.pop_front()
                    {
                        let auth = auth.clone();
                        let grant = voice_grant.grant.clone();
                        let prompt = transcript.prompt_tail();
                        uploads.spawn(async move {
                            auth.transcribe(
                                &grant,
                                chunk.sequence,
                                chunk.duration_ms,
                                chunk.final_chunk,
                                &prompt,
                                chunk.bytes,
                            )
                            .await
                        });
                    }
                    if release_seen && pending.is_empty() && uploads.is_empty() {
                        break;
                    }
                    tokio::select! {
                        biased;
                        result = uploads.join_next(), if !uploads.is_empty() => {
                            let Some(result) = result else { continue; };
                            let response = result
                                .map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription was interrupted. Retry."))??;
                            let partial = transcript.insert(response.sequence, response.text).map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription is incomplete. Retry."))?.to_owned();
                            manager.status.send_modify(|status| { status.revision += 1; status.partial_transcript = partial.clone(); });
                        }
                        _ = &mut released, if !release_seen => {
                            release_seen = true;
                            manager.transition("transcribing", "Finalizing the instruction…");
                            while let Ok(value) = microphone.samples.try_recv() {
                                for chunk in chunks.push(&value).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Voice instruction is too long."))? {
                                    expected += 1;
                                    pending.push_back(chunk);
                                }
                            }
                            if microphone.failed.load(std::sync::atomic::Ordering::Acquire) { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone capture stopped.")); }
                            if !chunks.has_speech() { return Err(crate::worker::WorkerError::new("NO_SPEECH", "No speech was detected.")); }
                            if let Some(chunk) = chunks.finish().map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription is unavailable."))? {
                                expected += 1;
                                pending.push_back(chunk);
                            }
                            if pending.len() > 1 { return Err(crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription could not keep up. Retry.")); }
                        }
                        value = microphone.samples.recv(), if !release_seen => {
                            let Some(value) = value else { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone capture stopped.")); };
                            if microphone.failed.load(std::sync::atomic::Ordering::Acquire) { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone capture stopped.")); }
                            for chunk in chunks.push(&value).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Voice instruction is too long."))? {
                                expected += 1;
                                pending.push_back(chunk);
                            }
                            if pending.len() > 1 { return Err(crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription could not keep up. Retry.")); }
                        }
                    }
                }
                let final_text = transcript.finish(expected).map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription is incomplete. Retry."))?;
                manager.status.send_modify(|status| { status.revision += 1; status.phase = "dispatching".into(); status.final_transcript = final_text.clone(); status.message = "Starting the selected-window action…".into(); });
                let preparation_id = prepared["preparationId"].as_str().and_then(|value| Uuid::parse_str(value).ok()).ok_or(crate::worker::WorkerError::new("INVALID_MESSAGE", "Runtime returned an invalid preparation."))?;
                let run_id = runtime.execute_instruction(utterance_id, preparation_id, &final_text, serde_json::json!({"origin":origin,"grant":action_grant.grant,"model":action_grant.model})).await?;
                manager.status.send_modify(|status| { status.revision += 1; status.phase = "executing".into(); status.run_id = Some(run_id.to_string()); status.target_title = prepared["target"]["title"].as_str().map(str::to_owned); status.message = "Working in the selected window…".into(); });
                Ok::<(), crate::worker::WorkerError>(())
            }.await;
            if let Err(error) = outcome {
                manager.status.send_modify(|status| {
                    status.revision += 1;
                    status.phase = "failed".into();
                    status.message = error.message.into();
                    status.confirmation = None;
                });
            }
            manager.capture.lock().await.take();
        });
        *self.capture.lock().await = Some(ActiveCapture {
            release: Some(release),
            task,
        });
        Ok(())
    }

    #[cfg(feature = "desktop")]
    pub async fn release_capture(&self) {
        if let Some(active) = self.capture.lock().await.as_mut()
            && let Some(release) = active.release.take()
        {
            let _ = release.send(());
        }
    }
}
