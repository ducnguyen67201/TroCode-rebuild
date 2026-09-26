// Generated. Do not edit.
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`PermissionState`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "unknown",
///    "notNeeded",
///    "prompt",
///    "granted",
///    "denied",
///    "unavailable"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PermissionState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "notNeeded")]
    NotNeeded,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::convert::From<&Self> for PermissionState {
    fn from(value: &PermissionState) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for PermissionState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::NotNeeded => f.write_str("notNeeded"),
            Self::Prompt => f.write_str("prompt"),
            Self::Granted => f.write_str("granted"),
            Self::Denied => f.write_str("denied"),
            Self::Unavailable => f.write_str("unavailable"),
        }
    }
}
impl ::std::str::FromStr for PermissionState {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "notNeeded" => Ok(Self::NotNeeded),
            "prompt" => Ok(Self::Prompt),
            "granted" => Ok(Self::Granted),
            "denied" => Ok(Self::Denied),
            "unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PermissionState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PermissionState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PermissionState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TranscriptionLanguage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TranscriptionLanguage",
///  "default": "vi",
///  "type": "string",
///  "enum": [
///    "vi",
///    "en",
///    "auto"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TranscriptionLanguage {
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::convert::From<&Self> for TranscriptionLanguage {
    fn from(value: &TranscriptionLanguage) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranscriptionLanguage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Vi => f.write_str("vi"),
            Self::En => f.write_str("en"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for TranscriptionLanguage {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vi" => Ok(Self::Vi),
            "en" => Ok(Self::En),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TranscriptionLanguage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranscriptionLanguage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranscriptionLanguage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for TranscriptionLanguage {
    fn default() -> Self {
        TranscriptionLanguage::Vi
    }
}
///`VoiceConfirmation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "VoiceConfirmation",
///  "type": "object",
///  "required": [
///    "confirmationId",
///    "summary"
///  ],
///  "properties": {
///    "confirmationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "summary": {
///      "type": "string",
///      "maxLength": 256,
///      "minLength": 1
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VoiceConfirmation {
    #[serde(rename = "confirmationId")]
    pub confirmation_id: VoiceConfirmationConfirmationId,
    pub summary: VoiceConfirmationSummary,
}
impl ::std::convert::From<&VoiceConfirmation> for VoiceConfirmation {
    fn from(value: &VoiceConfirmation) -> Self {
        value.clone()
    }
}
///`VoiceConfirmationConfirmationId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceConfirmationConfirmationId(::std::string::String);
impl ::std::ops::Deref for VoiceConfirmationConfirmationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceConfirmationConfirmationId> for ::std::string::String {
    fn from(value: VoiceConfirmationConfirmationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceConfirmationConfirmationId>
for VoiceConfirmationConfirmationId {
    fn from(value: &VoiceConfirmationConfirmationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceConfirmationConfirmationId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceConfirmationConfirmationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for VoiceConfirmationConfirmationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceConfirmationConfirmationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceConfirmationConfirmationId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceConfirmationSummary`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 256,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceConfirmationSummary(::std::string::String);
impl ::std::ops::Deref for VoiceConfirmationSummary {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceConfirmationSummary> for ::std::string::String {
    fn from(value: VoiceConfirmationSummary) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceConfirmationSummary> for VoiceConfirmationSummary {
    fn from(value: &VoiceConfirmationSummary) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceConfirmationSummary {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceConfirmationSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceConfirmationSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceConfirmationSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceConfirmationSummary {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoicePermissions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "VoicePermissions",
///  "type": "object",
///  "required": [
///    "keyboardMonitoring",
///    "microphone",
///    "ready",
///    "recovery"
///  ],
///  "properties": {
///    "keyboardMonitoring": {
///      "$ref": "#/definitions/PermissionState"
///    },
///    "microphone": {
///      "$ref": "#/definitions/PermissionState"
///    },
///    "ready": {
///      "type": "boolean"
///    },
///    "recovery": {
///      "type": "string",
///      "maxLength": 256
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VoicePermissions {
    #[serde(rename = "keyboardMonitoring")]
    pub keyboard_monitoring: PermissionState,
    pub microphone: PermissionState,
    pub ready: bool,
    pub recovery: VoicePermissionsRecovery,
}
impl ::std::convert::From<&VoicePermissions> for VoicePermissions {
    fn from(value: &VoicePermissions) -> Self {
        value.clone()
    }
}
///`VoicePermissionsRecovery`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 256
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoicePermissionsRecovery(::std::string::String);
impl ::std::ops::Deref for VoicePermissionsRecovery {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoicePermissionsRecovery> for ::std::string::String {
    fn from(value: VoicePermissionsRecovery) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoicePermissionsRecovery> for VoicePermissionsRecovery {
    fn from(value: &VoicePermissionsRecovery) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoicePermissionsRecovery {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoicePermissionsRecovery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoicePermissionsRecovery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoicePermissionsRecovery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoicePermissionsRecovery {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "VoiceStatus",
///  "type": "object",
///  "required": [
///    "actionsUsed",
///    "confirmation",
///    "finalTranscript",
///    "message",
///    "partialTranscript",
///    "permissions",
///    "phase",
///    "queuedInstructions",
///    "revision",
///    "runId",
///    "shortcut",
///    "targetTitle",
///    "transcriptionLanguage",
///    "utteranceId"
///  ],
///  "properties": {
///    "actionsUsed": {
///      "type": "integer",
///      "maximum": 12.0,
///      "minimum": 0.0
///    },
///    "confirmation": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/VoiceConfirmation"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "finalTranscript": {
///      "type": "string",
///      "maxLength": 2000
///    },
///    "message": {
///      "type": "string",
///      "maxLength": 256
///    },
///    "partialTranscript": {
///      "type": "string",
///      "maxLength": 2000
///    },
///    "permissions": {
///      "$ref": "#/definitions/VoicePermissions"
///    },
///    "phase": {
///      "type": "string",
///      "enum": [
///        "disabled",
///        "idle",
///        "listening",
///        "transcribing",
///        "dispatching",
///        "executing",
///        "confirmation",
///        "completed",
///        "cancelled",
///        "failed"
///      ]
///    },
///    "queuedInstructions": {
///      "type": "array",
///      "items": {
///        "type": "string",
///        "maxLength": 2000,
///        "minLength": 1
///      },
///      "maxItems": 4
///    },
///    "revision": {
///      "type": "integer",
///      "maximum": 9007199254740991.0,
///      "minimum": 0.0
///    },
///    "runId": {
///      "type": [
///        "string",
///        "null"
///      ],
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "shortcut": {
///      "type": "string",
///      "enum": [
///        "Command+Control",
///        "Left Control+Left Alt",
///        "Unavailable"
///      ]
///    },
///    "targetTitle": {
///      "type": [
///        "string",
///        "null"
///      ],
///      "maxLength": 256
///    },
///    "transcriptionLanguage": {
///      "$ref": "#/definitions/TranscriptionLanguage"
///    },
///    "utteranceId": {
///      "type": [
///        "string",
///        "null"
///      ],
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VoiceStatus {
    #[serde(rename = "actionsUsed")]
    pub actions_used: i64,
    pub confirmation: ::std::option::Option<VoiceConfirmation>,
    #[serde(rename = "finalTranscript")]
    pub final_transcript: VoiceStatusFinalTranscript,
    pub message: VoiceStatusMessage,
    #[serde(rename = "partialTranscript")]
    pub partial_transcript: VoiceStatusPartialTranscript,
    pub permissions: VoicePermissions,
    pub phase: VoiceStatusPhase,
    #[serde(rename = "queuedInstructions")]
    pub queued_instructions: ::std::vec::Vec<VoiceStatusQueuedInstructionsItem>,
    pub revision: i64,
    #[serde(rename = "runId")]
    pub run_id: ::std::option::Option<VoiceStatusRunId>,
    pub shortcut: VoiceStatusShortcut,
    #[serde(rename = "targetTitle")]
    pub target_title: ::std::option::Option<VoiceStatusTargetTitle>,
    #[serde(rename = "transcriptionLanguage")]
    pub transcription_language: TranscriptionLanguage,
    #[serde(rename = "utteranceId")]
    pub utterance_id: ::std::option::Option<VoiceStatusUtteranceId>,
}
impl ::std::convert::From<&VoiceStatus> for VoiceStatus {
    fn from(value: &VoiceStatus) -> Self {
        value.clone()
    }
}
///`VoiceStatusFinalTranscript`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 2000
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusFinalTranscript(::std::string::String);
impl ::std::ops::Deref for VoiceStatusFinalTranscript {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusFinalTranscript> for ::std::string::String {
    fn from(value: VoiceStatusFinalTranscript) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusFinalTranscript> for VoiceStatusFinalTranscript {
    fn from(value: &VoiceStatusFinalTranscript) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusFinalTranscript {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusFinalTranscript {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusFinalTranscript {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusFinalTranscript {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusFinalTranscript {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatusMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 256
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusMessage(::std::string::String);
impl ::std::ops::Deref for VoiceStatusMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusMessage> for ::std::string::String {
    fn from(value: VoiceStatusMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusMessage> for VoiceStatusMessage {
    fn from(value: &VoiceStatusMessage) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusMessage {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusMessage {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatusPartialTranscript`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 2000
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusPartialTranscript(::std::string::String);
impl ::std::ops::Deref for VoiceStatusPartialTranscript {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusPartialTranscript> for ::std::string::String {
    fn from(value: VoiceStatusPartialTranscript) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusPartialTranscript>
for VoiceStatusPartialTranscript {
    fn from(value: &VoiceStatusPartialTranscript) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusPartialTranscript {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusPartialTranscript {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusPartialTranscript {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusPartialTranscript {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusPartialTranscript {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatusPhase`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "disabled",
///    "idle",
///    "listening",
///    "transcribing",
///    "dispatching",
///    "executing",
///    "confirmation",
///    "completed",
///    "cancelled",
///    "failed"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum VoiceStatusPhase {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "listening")]
    Listening,
    #[serde(rename = "transcribing")]
    Transcribing,
    #[serde(rename = "dispatching")]
    Dispatching,
    #[serde(rename = "executing")]
    Executing,
    #[serde(rename = "confirmation")]
    Confirmation,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::convert::From<&Self> for VoiceStatusPhase {
    fn from(value: &VoiceStatusPhase) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for VoiceStatusPhase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("disabled"),
            Self::Idle => f.write_str("idle"),
            Self::Listening => f.write_str("listening"),
            Self::Transcribing => f.write_str("transcribing"),
            Self::Dispatching => f.write_str("dispatching"),
            Self::Executing => f.write_str("executing"),
            Self::Confirmation => f.write_str("confirmation"),
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for VoiceStatusPhase {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "idle" => Ok(Self::Idle),
            "listening" => Ok(Self::Listening),
            "transcribing" => Ok(Self::Transcribing),
            "dispatching" => Ok(Self::Dispatching),
            "executing" => Ok(Self::Executing),
            "confirmation" => Ok(Self::Confirmation),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`VoiceStatusQueuedInstructionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 2000,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusQueuedInstructionsItem(::std::string::String);
impl ::std::ops::Deref for VoiceStatusQueuedInstructionsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusQueuedInstructionsItem> for ::std::string::String {
    fn from(value: VoiceStatusQueuedInstructionsItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusQueuedInstructionsItem>
for VoiceStatusQueuedInstructionsItem {
    fn from(value: &VoiceStatusQueuedInstructionsItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusQueuedInstructionsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusQueuedInstructionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for VoiceStatusQueuedInstructionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for VoiceStatusQueuedInstructionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusQueuedInstructionsItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatusRunId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusRunId(::std::string::String);
impl ::std::ops::Deref for VoiceStatusRunId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusRunId> for ::std::string::String {
    fn from(value: VoiceStatusRunId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusRunId> for VoiceStatusRunId {
    fn from(value: &VoiceStatusRunId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusRunId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusRunId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusRunId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusRunId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusRunId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatusShortcut`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Command+Control",
///    "Left Control+Left Alt",
///    "Unavailable"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum VoiceStatusShortcut {
    #[serde(rename = "Command+Control")]
    CommandControl,
    #[serde(rename = "Left Control+Left Alt")]
    LeftControlLeftAlt,
    Unavailable,
}
impl ::std::convert::From<&Self> for VoiceStatusShortcut {
    fn from(value: &VoiceStatusShortcut) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for VoiceStatusShortcut {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CommandControl => f.write_str("Command+Control"),
            Self::LeftControlLeftAlt => f.write_str("Left Control+Left Alt"),
            Self::Unavailable => f.write_str("Unavailable"),
        }
    }
}
impl ::std::str::FromStr for VoiceStatusShortcut {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Command+Control" => Ok(Self::CommandControl),
            "Left Control+Left Alt" => Ok(Self::LeftControlLeftAlt),
            "Unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusShortcut {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusShortcut {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusShortcut {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`VoiceStatusTargetTitle`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 256
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusTargetTitle(::std::string::String);
impl ::std::ops::Deref for VoiceStatusTargetTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusTargetTitle> for ::std::string::String {
    fn from(value: VoiceStatusTargetTitle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusTargetTitle> for VoiceStatusTargetTitle {
    fn from(value: &VoiceStatusTargetTitle) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusTargetTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusTargetTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusTargetTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusTargetTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusTargetTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VoiceStatusUtteranceId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VoiceStatusUtteranceId(::std::string::String);
impl ::std::ops::Deref for VoiceStatusUtteranceId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceStatusUtteranceId> for ::std::string::String {
    fn from(value: VoiceStatusUtteranceId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceStatusUtteranceId> for VoiceStatusUtteranceId {
    fn from(value: &VoiceStatusUtteranceId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for VoiceStatusUtteranceId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VoiceStatusUtteranceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VoiceStatusUtteranceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VoiceStatusUtteranceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VoiceStatusUtteranceId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
