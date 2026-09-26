pub mod grants;
pub mod responses;
pub mod transcription;

use reqwest::Client;
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct ProviderService {
    pub database: DatabaseConnection,
    pub client: Client,
    pub api_key: Arc<str>,
    pub transcription_model: Arc<str>,
    pub action_model: Arc<str>,
}

impl ProviderService {
    pub fn new(
        database: DatabaseConnection,
        api_key: String,
        transcription_model: String,
        action_model: String,
    ) -> Result<Self, &'static str> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| "Provider client unavailable.")?;
        Ok(Self {
            database,
            client,
            api_key: api_key.into(),
            transcription_model: transcription_model.into(),
            action_model: action_model.into(),
        })
    }
}

pub async fn bounded_body(
    mut response: reqwest::Response,
    max_bytes: usize,
) -> Result<Vec<u8>, ()> {
    if response
        .content_length()
        .is_some_and(|length| length > max_bytes as u64)
    {
        return Err(());
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| ())? {
        if body.len().saturating_add(chunk.len()) > max_bytes {
            return Err(());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
