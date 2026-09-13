//! Small presentation projection. Worker/process authority never lives in React.
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub state: &'static str,
    pub generation_id: Option<String>,
    pub revision: u64,
    pub message: &'static str,
}
impl Default for Status {
    fn default() -> Self {
        Self {
            state: "stopped",
            generation_id: None,
            revision: 0,
            message: "Ready to start a diagnostic session.",
        }
    }
}
impl Status {
    pub fn transition(
        &mut self,
        state: &'static str,
        generation: Option<String>,
        message: &'static str,
    ) {
        self.state = state;
        self.generation_id = generation;
        self.message = message;
        self.revision += 1;
    }
}
