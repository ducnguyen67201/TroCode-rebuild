#[cfg(feature = "desktop")]
pub mod audio;
pub mod chunks;
pub mod settings;
pub mod transcript;

use serde::{Deserialize, Serialize};
#[cfg(feature = "desktop")]
use std::collections::VecDeque;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
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
    pub queued_instructions: Vec<String>,
    pub target_title: Option<String>,
    pub message: String,
    pub shortcut: String,
    pub transcription_language: TranscriptionLanguage,
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
            queued_instructions: Vec::new(),
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
            transcription_language: TranscriptionLanguage::default(),
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
    settings: VoiceSettingsStore,
    manual_disable: AtomicBool,
    #[cfg(feature = "desktop")]
    capture: Mutex<Option<ActiveCapture>>,
    #[cfg(feature = "desktop")]
    actions: Mutex<ActionState>,
    #[cfg(feature = "desktop")]
    dispatch: Mutex<()>,
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

#[cfg(feature = "desktop")]
const MAX_QUEUED_INSTRUCTIONS: usize = 4;

#[cfg(feature = "desktop")]
struct QueuedInstruction {
    utterance_id: Uuid,
    instruction: String,
    expected_target: Option<TargetIdentity>,
}

#[cfg(feature = "desktop")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TargetIdentity {
    pid: i64,
    window_id: i64,
}

#[cfg(feature = "desktop")]
impl TargetIdentity {
    fn from_prepared(prepared: &serde_json::Value) -> Option<Self> {
        Some(Self {
            pid: prepared["target"]["pid"].as_i64()?,
            window_id: prepared["target"]["window_id"].as_i64()?,
        })
    }
}

#[cfg(feature = "desktop")]
#[derive(Default)]
struct ActionState {
    run_id: Option<Uuid>,
    phase: String,
    target: Option<TargetIdentity>,
    queued: VecDeque<QueuedInstruction>,
}

#[cfg(feature = "desktop")]
impl ActionState {
    fn enqueue(
        &mut self,
        instruction: QueuedInstruction,
    ) -> Result<usize, crate::worker::WorkerError> {
        if self.queued.len() >= MAX_QUEUED_INSTRUCTIONS {
            return Err(crate::worker::WorkerError::new(
                "BUSY",
                "The follow-up queue is full. Wait for an instruction to finish.",
            ));
        }
        self.queued.push_back(instruction);
        Ok(self.queued.len())
    }

    fn queue_projection(&self) -> Vec<String> {
        self.queued
            .iter()
            .map(|item| item.instruction.clone())
            .collect()
    }
}

#[cfg(feature = "desktop")]
struct PreparedDispatch {
    prepared: serde_json::Value,
    grant: crate::auth::api_client::ProviderGrant,
    origin: String,
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
            #[cfg(feature = "desktop")]
            actions: Mutex::new(ActionState::default()),
            #[cfg(feature = "desktop")]
            dispatch: Mutex::new(()),
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

    #[cfg(any(feature = "desktop", test))]
    fn transcription_language_snapshot(&self) -> Vec<String> {
        self.status
            .borrow()
            .transcription_language
            .request_languages()
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
                format!("Hold {} to speak.", status.shortcut)
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
            if phase != "confirmation" {
                status.confirmation = None;
            }
        });
    }

    #[cfg(feature = "desktop")]
    fn begin_listening(&self, follow_up: bool) -> Result<Uuid, crate::worker::WorkerError> {
        if !matches!(
            self.status.borrow().phase.as_str(),
            "idle" | "completed" | "cancelled" | "failed" | "executing"
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
            if !follow_up {
                status.run_id = None;
            }
            status.partial_transcript.clear();
            status.final_transcript.clear();
            status.confirmation = None;
            if !follow_up {
                status.actions_used = 0;
            }
            status.message = if follow_up {
                "Listening for a follow-up…".into()
            } else {
                "Listening…".into()
            };
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
        #[cfg(feature = "desktop")]
        let run_id = {
            let mut actions = self.actions.lock().await;
            actions.queued.clear();
            actions.phase.clear();
            actions.target = None;
            actions.run_id.take()
        };
        #[cfg(not(feature = "desktop"))]
        let run_id = self
            .status
            .borrow()
            .run_id
            .as_deref()
            .and_then(|value| Uuid::parse_str(value).ok());
        if let Some(runtime) = runtime {
            let _ = runtime.cancel_action(run_id).await;
        }
        if self.status.borrow().phase != "disabled" {
            self.status.send_modify(|status| {
                status.revision += 1;
                status.phase = "cancelled".into();
                status.message = "Voice instruction cancelled.".into();
                status.confirmation = None;
                status.queued_instructions.clear();
            });
        }
        self.status()
    }

    pub fn disable(&self) -> VoiceStatus {
        self.disable_with_reason(true, "Voice control is disabled.")
    }

    pub fn disable_due_to_auth_loss(&self) -> VoiceStatus {
        self.disable_with_reason(false, "Voice control is disabled until you sign in again.")
    }

    fn disable_with_reason(&self, manual: bool, message: &str) -> VoiceStatus {
        if manual {
            self.manual_disable.store(true, Ordering::Release);
        }
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "disabled".into();
            status.utterance_id = None;
            status.run_id = None;
            status.partial_transcript.clear();
            status.final_transcript.clear();
            status.queued_instructions.clear();
            status.target_title = None;
            status.confirmation = None;
            status.message = message.into();
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
            status.queued_instructions.clear();
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
        #[cfg(feature = "desktop")]
        let target =
            TargetIdentity::from_prepared(&prepared).ok_or(crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "Runtime returned an invalid selected window.",
            ))?;
        let run_id = runtime
            .execute_instruction(
                utterance_id,
                preparation_id,
                &instruction,
                serde_json::json!({"origin":origin,"grant":grant.grant,"model":grant.model}),
            )
            .await?;
        #[cfg(feature = "desktop")]
        {
            let mut actions = self.actions.lock().await;
            actions.run_id = Some(run_id);
            actions.phase = "executing".into();
            actions.target = Some(target);
        }
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "executing".into();
            status.run_id = Some(run_id.to_string());
            status.target_title = target_title;
            status.message = "Working in the selected window…".into();
        });
        Ok(self.status())
    }

    #[cfg(feature = "desktop")]
    async fn dispatch_instruction_locked(
        &self,
        queued: QueuedInstruction,
        prefetched: Option<PreparedDispatch>,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<(), crate::worker::WorkerError> {
        let queued_projection = self.actions.lock().await.queue_projection();
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "dispatching".into();
            status.utterance_id = Some(queued.utterance_id.to_string());
            status.run_id = None;
            status.partial_transcript.clear();
            status.final_transcript = queued.instruction.clone();
            status.queued_instructions = queued_projection.clone();
            status.target_title = None;
            status.confirmation = None;
            status.actions_used = 0;
            status.message = "Preparing the selected window…".into();
        });
        runtime.start().await?;
        let dispatch = match prefetched {
            Some(value) => value,
            None => {
                let (prepared, grant) = tokio::join!(
                    runtime.prepare_instruction(queued.utterance_id),
                    auth.provider_grant(queued.utterance_id, false),
                );
                let (grant, origin) = grant?;
                PreparedDispatch {
                    prepared: prepared?,
                    grant,
                    origin,
                }
            }
        };
        let preparation_id = dispatch.prepared["preparationId"]
            .as_str()
            .and_then(|value| Uuid::parse_str(value).ok())
            .ok_or(crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "Runtime returned an invalid preparation.",
            ))?;
        let target = TargetIdentity::from_prepared(&dispatch.prepared).ok_or(
            crate::worker::WorkerError::new(
                "INVALID_MESSAGE",
                "Runtime returned an invalid selected window.",
            ),
        )?;
        if queued
            .expected_target
            .is_some_and(|expected| expected != target)
        {
            return Err(crate::worker::WorkerError::new(
                "NOT_READY",
                "The selected window changed. Repeat the follow-up.",
            ));
        }
        let target_title = dispatch.prepared["target"]["title"]
            .as_str()
            .map(str::to_owned);
        let run_id = runtime
            .execute_instruction(
                queued.utterance_id,
                preparation_id,
                &queued.instruction,
                serde_json::json!({
                    "origin": dispatch.origin,
                    "grant": dispatch.grant.grant,
                    "model": dispatch.grant.model,
                }),
            )
            .await?;
        {
            let mut actions = self.actions.lock().await;
            actions.run_id = Some(run_id);
            actions.phase = "executing".into();
            actions.target = Some(target);
        }
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "executing".into();
            status.run_id = Some(run_id.to_string());
            status.target_title = target_title;
            status.message = "Working in the selected window…".into();
        });
        Ok(())
    }

    #[cfg(feature = "desktop")]
    async fn submit_instruction(
        &self,
        queued: QueuedInstruction,
        prefetched: Option<PreparedDispatch>,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) -> Result<(), crate::worker::WorkerError> {
        if self.status.borrow().phase == "disabled" {
            return Err(crate::worker::WorkerError::new(
                "NOT_READY",
                "Voice control is disabled.",
            ));
        }
        let _dispatch = self.dispatch.lock().await;
        let submitted_id = queued.utterance_id;
        let (active_run, active_phase, queued_projection) = {
            let mut actions = self.actions.lock().await;
            actions.enqueue(queued)?;
            (
                actions.run_id,
                actions.phase.clone(),
                actions.queue_projection(),
            )
        };
        if let Some(run_id) = active_run {
            let waiting = queued_projection.len();
            self.status.send_modify(|status| {
                status.revision += 1;
                status.phase = if active_phase == "confirmation" {
                    "confirmation"
                } else {
                    "executing"
                }
                .into();
                status.run_id = Some(run_id.to_string());
                status.queued_instructions = queued_projection.clone();
                status.message = format!(
                    "Working on the current instruction. {waiting} follow-up{} queued.",
                    if waiting == 1 { "" } else { "s" }
                );
            });
            return Ok(());
        }
        let next = self
            .actions
            .lock()
            .await
            .queued
            .pop_front()
            .expect("the submitted instruction is queued");
        let prefetched = (next.utterance_id == submitted_id)
            .then_some(prefetched)
            .flatten();
        self.dispatch_instruction_locked(next, prefetched, runtime, auth)
            .await
    }

    #[cfg(feature = "desktop")]
    async fn fail_dispatch(&self, error: &crate::worker::WorkerError) {
        let mut actions = self.actions.lock().await;
        actions.run_id = None;
        actions.phase.clear();
        actions.target = None;
        actions.queued.clear();
        drop(actions);
        self.status.send_modify(|status| {
            status.revision += 1;
            status.phase = "failed".into();
            status.run_id = None;
            status.message = error.message.into();
            status.confirmation = None;
            status.queued_instructions.clear();
        });
    }

    #[cfg(feature = "desktop")]
    async fn resume_queue(
        &self,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) {
        let _dispatch = self.dispatch.lock().await;
        if self.status.borrow().phase == "disabled" {
            self.actions.lock().await.queued.clear();
            return;
        }
        let next = {
            let mut actions = self.actions.lock().await;
            if actions.run_id.is_some() {
                return;
            }
            actions.queued.pop_front()
        };
        if let Some(next) = next
            && let Err(error) = self
                .dispatch_instruction_locked(next, None, runtime, auth)
                .await
        {
            #[cfg(debug_assertions)]
            eprintln!(
                "tro diagnostic: queued_instruction_failed code={} message={}",
                error.code, error.message
            );
            self.fail_dispatch(&error).await;
        }
    }

    #[cfg(feature = "desktop")]
    pub async fn apply_action_event(
        self: &Arc<Self>,
        event: &serde_json::Value,
        runtime: Arc<crate::manager::RuntimeManager>,
        auth: Arc<crate::auth::AuthManager>,
    ) {
        if self.status.borrow().phase == "disabled" {
            return;
        }
        let Some(run_id) = event["runId"].as_str() else {
            return;
        };
        let Ok(parsed_run_id) = Uuid::parse_str(run_id) else {
            return;
        };
        let phase = event["phase"].as_str().unwrap_or("failed");
        let terminal = matches!(phase, "completed" | "cancelled" | "failed");
        let _dispatch = if terminal {
            Some(self.dispatch.lock().await)
        } else {
            None
        };
        {
            let mut actions = self.actions.lock().await;
            if actions.run_id != Some(parsed_run_id) {
                return;
            }
            actions.phase = phase.to_owned();
            if terminal {
                actions.run_id = None;
            }
        }
        let capture_active = self.capture.lock().await.is_some();
        if !capture_active || phase == "confirmation" {
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
        if !terminal || capture_active {
            return;
        }
        drop(_dispatch);
        self.resume_queue(runtime, auth).await;
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
        let follow_up_target = {
            let actions = self.actions.lock().await;
            actions.run_id.and(actions.target)
        };
        let follow_up = follow_up_target.is_some();
        let utterance_id = self.begin_listening(follow_up)?;
        let request_languages = self.transcription_language_snapshot();
        let mut microphone = audio::MicrophoneCapture::start().map_err(|_| {
            crate::worker::WorkerError::new(
                "VOICE_PERMISSION_REQUIRED",
                "Microphone access is required.",
            )
        })?;
        let sample_rate = microphone.sample_rate;
        let (release, mut released) = oneshot::channel();
        let manager = self.clone();
        let task = tokio::spawn(async move {
            let outcome = async {
                let mut chunks = chunks::ChunkAssembler::new(sample_rate).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone format is unavailable."))?;
                let mut transcript = transcript::TranscriptAssembler::default();
                let mut expected = 0_u32;
                let mut pending =
                    std::collections::VecDeque::<chunks::WavChunk>::new();
                let mut uploads = tokio::task::JoinSet::new();
                let mut release_seen = false;
                let preparation_runtime = runtime.clone();
                let preparation_auth = auth.clone();
                let preparation = async move {
                    preparation_runtime.start().await?;
                    if follow_up {
                        let (voice_grant, _) = preparation_auth
                            .provider_grant(utterance_id, true)
                            .await?;
                        Ok::<_, crate::worker::WorkerError>((voice_grant, None))
                    } else {
                        let (prepared, voice_grant, action_grant) = tokio::join!(
                            preparation_runtime.prepare_instruction(utterance_id),
                            preparation_auth.provider_grant(utterance_id, true),
                            preparation_auth.provider_grant(utterance_id, false),
                        );
                        let (voice_grant, _) = voice_grant?;
                        let (action_grant, origin) = action_grant?;
                        Ok((
                            voice_grant,
                            Some(PreparedDispatch {
                                prepared: prepared?,
                                grant: action_grant,
                                origin,
                            }),
                        ))
                    }
                };
                tokio::pin!(preparation);
                let mut preparation_complete = false;
                let mut voice_grant: Option<crate::auth::api_client::ProviderGrant> = None;
                let mut prefetched: Option<PreparedDispatch> = None;
                loop {
                    if let Some(voice_grant) = voice_grant.as_ref() {
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
                    }
                    if release_seen
                        && preparation_complete
                        && pending.is_empty()
                        && uploads.is_empty()
                    {
                        break;
                    }
                    tokio::select! {
                        biased;
                        result = &mut preparation, if !preparation_complete => {
                            let (voice_grant_value, prefetched_value) = result?;
                            preparation_complete = true;
                            voice_grant = Some(voice_grant_value);
                            prefetched = prefetched_value;
                            #[cfg(debug_assertions)]
                            eprintln!("tro diagnostic: voice_preparation_ready follow_up={follow_up} release_seen={release_seen}");
                        }
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
                        }
                        value = microphone.samples.recv(), if !release_seen => {
                            let Some(value) = value else { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Microphone capture stopped.")); };
                            if let Some(message) = microphone.failure_message() { return Err(crate::worker::WorkerError::new("VOICE_UNAVAILABLE", message)); }
                            for chunk in chunks.push(&value).map_err(|_| crate::worker::WorkerError::new("VOICE_UNAVAILABLE", "Voice instruction is too long."))? {
                                expected += 1;
                                pending.push_back(chunk);
                            }
                        }
                    }
                }
                let final_text = transcript.finish(expected).map_err(|_| crate::worker::WorkerError::new("TRANSCRIPTION_UNAVAILABLE", "Voice transcription is incomplete. Retry."))?;
                manager.status.send_modify(|status| {
                    status.revision += 1;
                    status.final_transcript = final_text.clone();
                    status.message = if follow_up {
                        "Adding the follow-up to the queue…".into()
                    } else {
                        "Starting the selected-window action…".into()
                    };
                });
                manager
                    .submit_instruction(
                        QueuedInstruction {
                            utterance_id,
                            instruction: final_text,
                            expected_target: follow_up_target,
                        },
                        prefetched,
                        runtime.clone(),
                        auth.clone(),
                    )
                    .await?;
                Ok::<(), crate::worker::WorkerError>(())
            }.await;
            if let Err(error) = outcome {
                #[cfg(debug_assertions)]
                eprintln!(
                    "tro diagnostic: voice_capture_failed code={} message={}",
                    error.code, error.message
                );
                let (active_run, active_phase) = {
                    let actions = manager.actions.lock().await;
                    (actions.run_id, actions.phase.clone())
                };
                manager.status.send_modify(|status| {
                    status.revision += 1;
                    if let Some(run_id) = active_run {
                        status.phase = if active_phase == "confirmation" {
                            "confirmation"
                        } else {
                            "executing"
                        }
                        .into();
                        status.run_id = Some(run_id.to_string());
                        status.message = format!("Follow-up was not queued: {}", error.message);
                    } else {
                        status.phase = "failed".into();
                        status.run_id = None;
                        status.message = error.message.into();
                        status.confirmation = None;
                    }
                });
            }
            manager.capture.lock().await.take();
            manager.resume_queue(runtime, auth).await;
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
    #[cfg(feature = "desktop")]
    use super::{ActionState, MAX_QUEUED_INSTRUCTIONS, QueuedInstruction};
    use super::{AutoArmGate, VoiceManager, VoicePermissions, settings::TranscriptionLanguage};
    #[cfg(feature = "desktop")]
    use uuid::Uuid;

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
    fn transcription_language_defaults_to_vietnamese_and_updates_only_its_projection() {
        let voice = VoiceManager::default();
        let before = voice.status();

        assert_eq!(before.transcription_language, TranscriptionLanguage::Vi);
        let after = voice.set_transcription_language("en").unwrap();

        assert_eq!(after.transcription_language, TranscriptionLanguage::En);
        assert_eq!(after.revision, before.revision + 1);
        assert_eq!(after.phase, before.phase);
        assert_eq!(after.run_id, before.run_id);
        assert_eq!(
            serde_json::to_value(after).unwrap()["transcriptionLanguage"],
            "en"
        );
    }

    #[test]
    fn invalid_or_failed_language_changes_leave_status_unchanged() {
        let directory = tempfile::tempdir().unwrap();
        let blocked_parent = directory.path().join("not-a-directory");
        std::fs::write(&blocked_parent, b"blocked").unwrap();
        let voice = VoiceManager::with_settings_path(blocked_parent.join("voice-settings.json"));
        let before = voice.status();

        assert_eq!(
            voice.set_transcription_language("fr").unwrap_err().code,
            "INVALID_MESSAGE"
        );
        assert_eq!(
            voice.set_transcription_language("en").unwrap_err().code,
            "SETTINGS_UNAVAILABLE"
        );
        assert_eq!(voice.status().revision, before.revision);
        assert_eq!(
            voice.status().transcription_language,
            before.transcription_language
        );
    }

    #[test]
    fn capture_language_snapshot_is_stable_until_the_next_capture() {
        let voice = VoiceManager::default();
        let current_capture = voice.transcription_language_snapshot();

        voice.set_transcription_language("en").unwrap();

        assert_eq!(current_capture, vec!["vi"]);
        assert_eq!(voice.transcription_language_snapshot(), vec!["en"]);
    }

    #[test]
    fn persisted_language_is_loaded_by_the_next_manager() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("voice-settings.json");
        let voice = VoiceManager::with_settings_path(path.clone());

        voice.set_transcription_language("auto").unwrap();
        drop(voice);

        assert_eq!(
            VoiceManager::with_settings_path(path)
                .status()
                .transcription_language,
            TranscriptionLanguage::Auto
        );
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
            "Voice control is disabled until you sign in again."
        );
    }

    #[cfg(feature = "desktop")]
    #[test]
    fn listening_for_follow_up_preserves_the_active_run() {
        let voice = VoiceManager::default();
        voice.enable(ready_permissions());
        let run_id = Uuid::new_v4().to_string();
        voice.status.send_modify(|status| {
            status.phase = "executing".into();
            status.run_id = Some(run_id.clone());
        });

        voice.begin_listening(true).unwrap();

        let status = voice.status();
        assert_eq!(status.phase, "listening");
        assert_eq!(status.run_id.as_deref(), Some(run_id.as_str()));
        assert_eq!(status.message, "Listening for a follow-up…");
    }

    #[cfg(feature = "desktop")]
    #[test]
    fn follow_up_queue_is_bounded_and_fifo() {
        let mut actions = ActionState::default();
        let ids = (0..MAX_QUEUED_INSTRUCTIONS)
            .map(|index| {
                let utterance_id = Uuid::new_v4();
                actions
                    .enqueue(QueuedInstruction {
                        utterance_id,
                        instruction: format!("instruction {index}"),
                        expected_target: None,
                    })
                    .unwrap();
                utterance_id
            })
            .collect::<Vec<_>>();

        let error = actions
            .enqueue(QueuedInstruction {
                utterance_id: Uuid::new_v4(),
                instruction: "one too many".into(),
                expected_target: None,
            })
            .unwrap_err();
        assert_eq!(error.code, "BUSY");
        assert_eq!(
            actions.queued.pop_front().map(|item| item.utterance_id),
            Some(ids[0])
        );
    }
}
