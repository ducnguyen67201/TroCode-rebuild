use serde_json::Value;
use std::sync::LazyLock;
pub const MAX_FRAME_BYTES: usize = 256 * 1024;
pub const PROTOCOL_VERSION: u64 = 3;
pub const SCHEMA_DIGEST: &str = include_str!("../schema/digest.txt");
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod generated;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod generated_voice;
static VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    let schema: Value = serde_json::from_str(include_str!("../schema/protocol.schema.json"))
        .expect("embedded schema");
    jsonschema::draft7::new(&schema).expect("valid embedded schema")
});
static VOICE_VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    let schema: Value = serde_json::from_str(include_str!("../schema/voice.schema.json"))
        .expect("embedded voice schema");
    jsonschema::draft7::new(&schema).expect("valid embedded voice schema")
});
#[derive(Debug, thiserror::Error)]
#[error("Invalid runtime message")]
pub struct InvalidMessage;
pub fn parse_message(bytes: &[u8]) -> Result<Value, InvalidMessage> {
    if bytes.is_empty() || bytes.len() > MAX_FRAME_BYTES {
        return Err(InvalidMessage);
    }
    let value: Value = serde_json::from_slice(bytes).map_err(|_| InvalidMessage)?;
    if !VALIDATOR.is_valid(&value) {
        return Err(InvalidMessage);
    }
    Ok(value)
}

pub fn parse_voice_status(value: &Value) -> Result<(), InvalidMessage> {
    if VOICE_VALIDATOR.is_valid(value) {
        Ok(())
    } else {
        Err(InvalidMessage)
    }
}
