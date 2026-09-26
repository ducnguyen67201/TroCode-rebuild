#[cfg(feature = "desktop")]
pub mod audio;
pub mod chunks;
pub mod settings;
pub mod transcript;

use serde::{Deserialize, Serialize};
#[cfg(feature = "desktop")]
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::watch;
#[cfg(feature = "desktop")]
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

use settings::{TranscriptionLanguage, VoiceSettingsStore};

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
pub struct VoiceStatus {
    pub phase: String,
    pub revision: u64,
    pub utterance_id: Option<String>,
    pub guidance_id: Option<String>,
    pub partial_transcript: String,
    pub final_transcript: String,
    pub target_title: Option<String>,
    pub message: String,
    pub shortcut: String,
    pub transcription_language: TranscriptionLanguage,
    pub permissions: VoicePermissions,
}

impl Default for VoiceStatus {
    fn default() -> Self {
        Self {
            phase: "disabled".into(),
            revision: 0,
            utterance_id: None,
            guidance_id: None,
            partial_transcript: String::new(),
            final_transcript: String::new(),
            target_title: None,
            message: "Enable voice guidance to use push-to-talk.".into(),
            shortcut: if cfg!(target_os = "macos") {
                "Command+Control"
            } else if cfg!(target_os = "windows") {
                "Left Control+Left Alt"
            } else {
                "Unavailable"
            }
            .into(),
            transcription_language: TranscriptionLanguage::default(),
            permissions: VoicePermissions {
                microphone: "unknown".into(),
                keyboard_monitoring: "unknown".into(),
                ready: false,
                recovery: String::new(),
            },
        }
    }
}

pub struct VoiceManager {
    status: watch::Sender<VoiceStatus>,
    settings: VoiceSettingsStore,
    manual_disable: AtomicBool,
    #[cfg(feature = "desktop")]
    capture: Mutex<Option<ActiveCapture>>,
}

#[cfg(any(feature = "desktop", test))]
#[derive(Default)]
pub(crate) struct AutoArmGate {
    authenticated: bool,
}

#[cfg(any(feature = "desktop", test))]
impl AutoArmGate {
    pub(crate) fn update(&mut self, authenticated: bool) -> bool {
        let should_arm = authenticated && !self.authenticated;
        self.authenticated = authenticated;
        should_arm
    }
}

#[cfg(feature = "desktop")]
struct ActiveCapture {
    release: Option<oneshot::Sender<()>>,
    task: tokio::task::JoinHandle<()>,
}

impl Default for VoiceManager {
    fn default() -> Self {
        Self::from_settings_store(VoiceSettingsStore::default())
    }
}

impl VoiceManager {
    pub fn with_settings_path(path: std::path::PathBuf) -> Self {
        Self::from_settings_store(VoiceSettingsStore::at(path))
    }

    fn from_settings_store(settings: VoiceSettingsStore) -> Self {
        let initial = VoiceStatus {
            transcription_language: settings.load(),
            ..VoiceStatus::default()
        };
        let (status, _) = watch::channel(initial);
        Self {
            status,
            settings,
            manual_disable: AtomicBool::new(false),
            #[cfg(feature = "desktop")]
            capture: Mutex::new(None),
        }
    }

    pub fn status(&self) -> VoiceStatus {
        self.status.borrow().clone()
    }

    pub fn subscribe(&self) -> watch::Receiver<VoiceStatus> {
        self.status.subscribe()
    }

    pub fn set_transcription_language(
        &self,
        value: &str,
    ) -> Result<VoiceStatus, crate::worker::WorkerError> {
        let language = value.parse::<TranscriptionLanguage>().map_err(|_| {
            crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "Choose Auto, English, or Vietnamese.",
            )
        })?;
        self.settings.save(language).map_err(|_| {
            crate::worker::WorkerError::new(
                "SETTINGS_UNAVAILABLE",
                "Transcription language could not be saved. Try again.",
            )
        })?;
        self.status.send_modify(|status| {
            status.revision += 1;
            status.transcription_language = language;
        });
        Ok(self.status())
    }

    #[cfg(feature = "desktop")]
    fn transcription_language_snapshot(&self) -> Vec<TranscriptionLanguage> {
        settings::request_languages(self.status.borrow().transcription_language)
    }

    pub fn enable(&self, permissions: VoicePermissions) -> VoiceStatus {
        self.manual_disable.store(false, Ordering::Release);
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
                format!("Hold {} to ask for help.", status.shortcut)
            } else {
                "Voice permissions are required.".into()
            };
        });
        self.status()
    }

    pub fn auto_arm_allowed(&self) -> bool {
        !self.manual_disable.load(Ordering::Acquire)
    }

    #[cfg(feature = "desktop")]
    fn transition(&self, phase: &str, message: &str) {
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = phase.to_owned();
            status.message = message.to_owned();
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
                "Voice guidance is not ready.",
            ));
        }
        let id = Uuid::new_v4();
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "listening".into();
            status.utterance_id = Some(id.to_string());
            status.guidance_id = None;
            status.partial_transcript.clear();
            status.final_transcript.clear();
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
            runtime.stop().await;
        }
        if self.status.borrow().phase != "disabled" {
            self.status.send_modify(|status| {
                status.revision += 1;
                status.phase = "cancelled".into();
                status.message = "Voice guidance cancelled.".into();
            });
        }
        self.status()
    }

    pub fn disable(&self) -> VoiceStatus {
        self.disable_with_reason(true, "Voice guidance is disabled.")
    }

    pub fn disable_due_to_auth_loss(&self) -> VoiceStatus {
        self.disable_with_reason(false, "Voice guidance is disabled until you sign in again.")
    }

    fn disable_with_reason(&self, manual: bool, message: &str) -> VoiceStatus {
        if manual {
            self.manual_disable.store(true, Ordering::Release);
        }
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "disabled".into();
            status.utterance_id = None;
            status.guidance_id = None;
            status.partial_transcript.clear();
            status.final_transcript.clear();
            status.target_title = None;
            status.message = message.into();
        });
        self.status()
    }

    #[cfg(feature = "desktop")]
    async fn prepare_guidance(
        &self,
        instruction: &str,
        utterance_id: Uuid,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<(serde_json::Value, Option<String>), crate::worker::WorkerError> {
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
        self.transition("planning", "Preparing a simple walkthrough…");
        let state = runtime
            .start_guidance(
                utterance_id,
                preparation_id,
                instruction,
                "en",
                serde_json::json!({"origin":origin,"grant":grant.grant,"model":grant.model}),
            )
            .await?;
        Ok((state, target_title))
    }

    #[cfg(feature = "desktop")]
    fn guidance_ready(&self, state: &serde_json::Value, target_title: Option<String>) {
        let guidance_id = state["journey"]["id"].as_str().map(str::to_owned);
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "guiding".into();
            status.guidance_id = guidance_id;
            status.target_title = target_title;
            status.message = "Follow the cursor in the selected window.".into();
        });
    }

    #[cfg(feature = "desktop")]
    pub(crate) fn guidance_completed(&self, guidance_id: Option<&str>) {
        let current = self.status.borrow().guidance_id.clone();
        if current.as_deref() != guidance_id || self.status.borrow().phase != "guiding" {
            return;
        }
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "completed".into();
            status.message = "Your guidance is ready.".into();
        });
    }

    #[cfg(feature = "desktop")]
    pub async fn execute_text(
        &self,
        instruction: String,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<(VoiceStatus, serde_json::Value), crate::worker::WorkerError> {
        if instruction.trim().is_empty() || instruction.len() > 2_000 {
            return Err(crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "A short guidance request is required.",
            ));
        }
        if matches!(
            self.status.borrow().phase.as_str(),
            "listening" | "transcribing" | "dispatching" | "planning" | "guiding"
        ) {
            return Err(crate::worker::WorkerError::new(
                "BUSY",
                "A guidance request is already active.",
            ));
        }
        runtime.start().await?;
        let utterance_id = Uuid::new_v4();
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "dispatching".into();
            status.utterance_id = Some(utterance_id.to_string());
            status.guidance_id = None;
            status.partial_transcript.clear();
            status.final_transcript = instruction.clone();
            status.message = "Preparing the selected window…".into();
        });
        let (state, target_title) = self
            .prepare_guidance(&instruction, utterance_id, runtime, auth)
            .await?;
        self.guidance_ready(&state, target_title);
        Ok((self.status(), state))
    }
    #[cfg(feature = "desktop")]
    pub async fn begin_capture(
        self: &Arc<Self>,
        app: tauri::AppHandle,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<(), crate::worker::WorkerError> {
        if self.capture.lock().await.is_some() {
            return Ok(());
        }
        let utterance_id = self.begin_listening()?;
        let request_languages = self.transcription_language_snapshot();
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
            let (prepared, voice_grant, guidance_grant) = tokio::join!(
                runtime.prepare_instruction(utterance_id),
                auth.provider_grant(utterance_id, true),
                auth.provider_grant(utterance_id, false),
            );
            let outcome = async {
                let prepared = prepared?;
                let (voice_grant, _) = voice_grant?;
                let (guidance_grant, origin) = guidance_grant?;
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
                        let languages = request_languages.clone();
                        uploads.spawn(async move {
                            auth.transcribe(crate::auth::TranscriptionRequest::new(
                                &grant,
                                chunk.sequence,
                                chunk.duration_ms,
                                chunk.final_chunk,
                                &prompt,
                                &languages,
                                chunk.bytes,
                            ))
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
                            manager.transition("transcribing", "Finishing your question…");
                            while let Ok(value) = microphone.samples.try_recv() {
                                for chunk in chunks.push(&value).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Voice instruction is too long."))? {
                                    expected += 1;
                                    pending.push_back(chunk);
                                }
                            }
                            microphone.stop();
                            while let Ok(value) = microphone.samples.try_recv() {
                                for chunk in chunks.push(&value).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Voice instruction is too long."))? {
                                    expected += 1;
                                    pending.push_back(chunk);
                                }
                            }
                            if let Some(message) = microphone.failure_message() { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", message)); }
                            if !chunks.has_speech() { return Err(crate::worker::WorkerError::new("NO_SPEECH", "No speech was detected.")); }
                            if let Some(chunk) = chunks.finish().map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription is unavailable."))? {
                                expected += 1;
                                pending.push_back(chunk);
                            }
                            if pending.len() > 1 { return Err(crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription could not keep up. Retry.")); }
                        }
                        value = microphone.samples.recv(), if !release_seen => {
                            let Some(value) = value else { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone capture stopped.")); };
                            if let Some(message) = microphone.failure_message() { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", message)); }
                            for chunk in chunks.push(&value).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Voice instruction is too long."))? {
                                expected += 1;
                                pending.push_back(chunk);
                            }
                            if pending.len() > 1 { return Err(crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription could not keep up. Retry.")); }
                        }
                    }
                }
                let final_text = transcript.finish(expected).map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription is incomplete. Retry."))?;
                manager.status.send_modify(|status| { status.revision += 1; status.phase = "dispatching".into(); status.final_transcript = final_text.clone(); status.message = "Preparing the selected window…".into(); });
                let preparation_id = prepared["preparationId"].as_str().and_then(|value| Uuid::parse_str(value).ok()).ok_or(crate::worker::WorkerError::new("INVALID_MESSAGE", "Runtime returned an invalid preparation."))?;
                let target_title = prepared["target"]["title"].as_str().map(str::to_owned);
                manager.transition("planning", "Preparing a simple walkthrough…");
                let state = runtime.start_guidance(utterance_id, preparation_id, &final_text, "en", serde_json::json!({"origin":origin,"grant":guidance_grant.grant,"model":guidance_grant.model})).await?;
                crate::overlay::present_guidance(&app, runtime.clone(), &state).await?;
                manager.guidance_ready(&state, target_title);
                Ok::<(), crate::worker::WorkerError>(())
            }.await;
            if let Err(error) = outcome {
                manager.status.send_modify(|status| {
                    status.revision += 1;
                    status.phase = "failed".into();
                    status.message = error.message.into();
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

#[cfg(test)]
mod auto_arm_tests {
    use super::{AutoArmGate, VoiceManager, VoicePermissions};

    fn ready_permissions() -> VoicePermissions {
        VoicePermissions {
            microphone: "granted".into(),
            keyboard_monitoring: "granted".into(),
            ready: true,
            recovery: String::new(),
        }
    }

    #[test]
    fn arms_once_per_authenticated_session() {
        let mut gate = AutoArmGate::default();

        assert!(!gate.update(false));
        assert!(gate.update(true));
        assert!(!gate.update(true));
        assert!(!gate.update(false));
        assert!(gate.update(true));
    }

    #[test]
    fn manual_disable_blocks_auto_arm_until_explicit_enable() {
        let voice = VoiceManager::default();

        assert!(voice.auto_arm_allowed());
        assert_eq!(voice.disable().phase, "disabled");
        assert!(!voice.auto_arm_allowed());

        assert_eq!(voice.disable_due_to_auth_loss().phase, "disabled");
        assert!(!voice.auto_arm_allowed());

        assert_eq!(voice.enable(ready_permissions()).phase, "idle");
        assert!(voice.auto_arm_allowed());
    }

    #[test]
    fn auth_loss_projects_disabled_voice_status() {
        let voice = VoiceManager::default();

        assert_eq!(voice.enable(ready_permissions()).phase, "idle");
        let status = voice.disable_due_to_auth_loss();

        assert_eq!(status.phase, "disabled");
        assert_eq!(
            status.message,
            "Voice guidance is disabled until you sign in again."
        );
    }

    #[cfg(feature = "desktop")]
    #[test]
    fn only_current_guidance_can_complete_voice_status() {
        let voice = VoiceManager::default();
        let permissions = ready_permissions();
        voice.enable(permissions);
        voice.status.send_modify(|status| {
            status.phase = "guiding".into();
            status.guidance_id = Some("current".into());
        });

        voice.guidance_completed(Some("stale"));
        assert_eq!(voice.status().phase, "guiding");
        voice.guidance_completed(Some("current"));
        assert_eq!(voice.status().phase, "completed");
        assert_eq!(voice.status().message, "Your guidance is ready.");
    }
}
