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
///`CheckResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "CheckResult",
///  "type": "object",
///  "required": [
///    "checked_at",
///    "message",
///    "observation_id",
///    "outcome",
///    "source"
///  ],
///  "properties": {
///    "checked_at": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "message": {
///      "type": "string",
///      "maxLength": 256
///    },
///    "observation_id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "outcome": {
///      "type": "string",
///      "enum": [
///        "confirmed",
///        "mismatch",
///        "unknown"
///      ]
///    },
///    "source": {
///      "type": "string",
///      "enum": [
///        "fresh_observation"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CheckResult {
    pub checked_at: f64,
    pub message: CheckResultMessage,
    pub observation_id: CheckResultObservationId,
    pub outcome: CheckResultOutcome,
    pub source: CheckResultSource,
}
impl ::std::convert::From<&CheckResult> for CheckResult {
    fn from(value: &CheckResult) -> Self {
        value.clone()
    }
}
///`CheckResultMessage`
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
pub struct CheckResultMessage(::std::string::String);
impl ::std::ops::Deref for CheckResultMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CheckResultMessage> for ::std::string::String {
    fn from(value: CheckResultMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CheckResultMessage> for CheckResultMessage {
    fn from(value: &CheckResultMessage) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CheckResultMessage {
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
impl ::std::convert::TryFrom<&str> for CheckResultMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CheckResultMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CheckResultMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CheckResultMessage {
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
///`CheckResultObservationId`
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
pub struct CheckResultObservationId(::std::string::String);
impl ::std::ops::Deref for CheckResultObservationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CheckResultObservationId> for ::std::string::String {
    fn from(value: CheckResultObservationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CheckResultObservationId> for CheckResultObservationId {
    fn from(value: &CheckResultObservationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CheckResultObservationId {
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
impl ::std::convert::TryFrom<&str> for CheckResultObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CheckResultObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CheckResultObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CheckResultObservationId {
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
///`CheckResultOutcome`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "confirmed",
///    "mismatch",
///    "unknown"
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
pub enum CheckResultOutcome {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "mismatch")]
    Mismatch,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::convert::From<&Self> for CheckResultOutcome {
    fn from(value: &CheckResultOutcome) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CheckResultOutcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Confirmed => f.write_str("confirmed"),
            Self::Mismatch => f.write_str("mismatch"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for CheckResultOutcome {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "confirmed" => Ok(Self::Confirmed),
            "mismatch" => Ok(Self::Mismatch),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CheckResultOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CheckResultOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CheckResultOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CheckResultSource`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "fresh_observation"
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
pub enum CheckResultSource {
    #[serde(rename = "fresh_observation")]
    FreshObservation,
}
impl ::std::convert::From<&Self> for CheckResultSource {
    fn from(value: &CheckResultSource) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CheckResultSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FreshObservation => f.write_str("fresh_observation"),
        }
    }
}
impl ::std::str::FromStr for CheckResultSource {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fresh_observation" => Ok(Self::FreshObservation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CheckResultSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CheckResultSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CheckResultSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Element`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Element",
///  "type": "object",
///  "required": [
///    "bounds",
///    "id",
///    "label",
///    "role",
///    "value"
///  ],
///  "properties": {
///    "bounds": {
///      "$ref": "#/definitions/Rect"
///    },
///    "id": {
///      "type": "string",
///      "maxLength": 80
///    },
///    "label": {
///      "type": "string",
///      "maxLength": 256
///    },
///    "role": {
///      "type": "string",
///      "maxLength": 80
///    },
///    "value": {
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
pub struct Element {
    pub bounds: Rect,
    pub id: ElementId,
    pub label: ElementLabel,
    pub role: ElementRole,
    pub value: ElementValue,
}
impl ::std::convert::From<&Element> for Element {
    fn from(value: &Element) -> Self {
        value.clone()
    }
}
///`ElementId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 80
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ElementId(::std::string::String);
impl ::std::ops::Deref for ElementId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ElementId> for ::std::string::String {
    fn from(value: ElementId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ElementId> for ElementId {
    fn from(value: &ElementId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ElementId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 80usize {
            return Err("longer than 80 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ElementId {
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
///`ElementLabel`
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
pub struct ElementLabel(::std::string::String);
impl ::std::ops::Deref for ElementLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ElementLabel> for ::std::string::String {
    fn from(value: ElementLabel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ElementLabel> for ElementLabel {
    fn from(value: &ElementLabel) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ElementLabel {
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
impl ::std::convert::TryFrom<&str> for ElementLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ElementLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElementLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ElementLabel {
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
///`ElementRole`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 80
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ElementRole(::std::string::String);
impl ::std::ops::Deref for ElementRole {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ElementRole> for ::std::string::String {
    fn from(value: ElementRole) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ElementRole> for ElementRole {
    fn from(value: &ElementRole) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ElementRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 80usize {
            return Err("longer than 80 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ElementRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ElementRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElementRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ElementRole {
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
///`ElementValue`
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
pub struct ElementValue(::std::string::String);
impl ::std::ops::Deref for ElementValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ElementValue> for ::std::string::String {
    fn from(value: ElementValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ElementValue> for ElementValue {
    fn from(value: &ElementValue) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ElementValue {
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
impl ::std::convert::TryFrom<&str> for ElementValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ElementValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElementValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ElementValue {
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
///`Journey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Journey",
///  "type": "object",
///  "required": [
///    "id",
///    "index",
///    "message",
///    "status",
///    "steps"
///  ],
///  "properties": {
///    "id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "index": {
///      "type": "integer",
///      "maximum": 3.0,
///      "minimum": 0.0
///    },
///    "message": {
///      "type": "string",
///      "maxLength": 400
///    },
///    "status": {
///      "type": "string",
///      "enum": [
///        "running",
///        "awaiting_confirmation",
///        "paused",
///        "completed"
///      ]
///    },
///    "steps": {
///      "type": "array",
///      "items": {
///        "type": "string",
///        "maxLength": 400,
///        "minLength": 1
///      },
///      "maxItems": 3,
///      "minItems": 1
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Journey {
    pub id: JourneyId,
    pub index: i64,
    pub message: JourneyMessage,
    pub status: JourneyStatus,
    pub steps: ::std::vec::Vec<JourneyStepsItem>,
}
impl ::std::convert::From<&Journey> for Journey {
    fn from(value: &Journey) -> Self {
        value.clone()
    }
}
///`JourneyId`
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
pub struct JourneyId(::std::string::String);
impl ::std::ops::Deref for JourneyId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JourneyId> for ::std::string::String {
    fn from(value: JourneyId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JourneyId> for JourneyId {
    fn from(value: &JourneyId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for JourneyId {
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
impl ::std::convert::TryFrom<&str> for JourneyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JourneyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JourneyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JourneyId {
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
///`JourneyMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 400
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JourneyMessage(::std::string::String);
impl ::std::ops::Deref for JourneyMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JourneyMessage> for ::std::string::String {
    fn from(value: JourneyMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JourneyMessage> for JourneyMessage {
    fn from(value: &JourneyMessage) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for JourneyMessage {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 400usize {
            return Err("longer than 400 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JourneyMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JourneyMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JourneyMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JourneyMessage {
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
///`JourneyStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "running",
///    "awaiting_confirmation",
///    "paused",
///    "completed"
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
pub enum JourneyStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "awaiting_confirmation")]
    AwaitingConfirmation,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::convert::From<&Self> for JourneyStatus {
    fn from(value: &JourneyStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JourneyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
            Self::AwaitingConfirmation => f.write_str("awaiting_confirmation"),
            Self::Paused => f.write_str("paused"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for JourneyStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            "awaiting_confirmation" => Ok(Self::AwaitingConfirmation),
            "paused" => Ok(Self::Paused),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JourneyStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JourneyStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JourneyStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JourneyStepsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 400,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JourneyStepsItem(::std::string::String);
impl ::std::ops::Deref for JourneyStepsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JourneyStepsItem> for ::std::string::String {
    fn from(value: JourneyStepsItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JourneyStepsItem> for JourneyStepsItem {
    fn from(value: &JourneyStepsItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for JourneyStepsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 400usize {
            return Err("longer than 400 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JourneyStepsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JourneyStepsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JourneyStepsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JourneyStepsItem {
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
///`Observation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Observation",
///  "type": "object",
///  "required": [
///    "captured_at",
///    "complete",
///    "elements",
///    "id",
///    "target"
///  ],
///  "properties": {
///    "captured_at": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "complete": {
///      "type": "boolean"
///    },
///    "elements": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Element"
///      },
///      "maxItems": 200
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "screenshot_id": {
///      "anyOf": [
///        {
///          "type": "string",
///          "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "target": {
///      "$ref": "#/definitions/Target"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub captured_at: f64,
    pub complete: bool,
    pub elements: ::std::vec::Vec<Element>,
    pub id: ObservationId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub screenshot_id: ::std::option::Option<ObservationScreenshotId>,
    pub target: Target,
}
impl ::std::convert::From<&Observation> for Observation {
    fn from(value: &Observation) -> Self {
        value.clone()
    }
}
///`ObservationId`
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
pub struct ObservationId(::std::string::String);
impl ::std::ops::Deref for ObservationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ObservationId> for ::std::string::String {
    fn from(value: ObservationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ObservationId> for ObservationId {
    fn from(value: &ObservationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ObservationId {
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
impl ::std::convert::TryFrom<&str> for ObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ObservationId {
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
///`ObservationScreenshotId`
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
pub struct ObservationScreenshotId(::std::string::String);
impl ::std::ops::Deref for ObservationScreenshotId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ObservationScreenshotId> for ::std::string::String {
    fn from(value: ObservationScreenshotId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ObservationScreenshotId> for ObservationScreenshotId {
    fn from(value: &ObservationScreenshotId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ObservationScreenshotId {
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
impl ::std::convert::TryFrom<&str> for ObservationScreenshotId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ObservationScreenshotId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ObservationScreenshotId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ObservationScreenshotId {
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
///`Rect`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Rect",
///  "type": "object",
///  "required": [
///    "height",
///    "width",
///    "x",
///    "y"
///  ],
///  "properties": {
///    "height": {
///      "type": "number",
///      "maximum": 100000.0,
///      "exclusiveMinimum": 0.0
///    },
///    "width": {
///      "type": "number",
///      "maximum": 100000.0,
///      "exclusiveMinimum": 0.0
///    },
///    "x": {
///      "type": "number",
///      "maximum": 100000.0,
///      "minimum": -100000.0
///    },
///    "y": {
///      "type": "number",
///      "maximum": 100000.0,
///      "minimum": -100000.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rect {
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl ::std::convert::From<&Rect> for Rect {
    fn from(value: &Rect) -> Self {
        value.clone()
    }
}
///`RuntimeAsk`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeAsk",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "locale",
///    "protocolVersion",
///    "question",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.ask"
///      ]
///    },
///    "locale": {
///      "type": "string",
///      "enum": [
///        "en",
///        "vi"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "question": {
///      "type": "string",
///      "maxLength": 1000,
///      "minLength": 1
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeAsk {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeAskCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeAskGenerationId,
    pub kind: RuntimeAskKind,
    pub locale: RuntimeAskLocale,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeAskProtocolVersion,
    pub question: RuntimeAskQuestion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeAskRequestId,
}
impl ::std::convert::From<&RuntimeAsk> for RuntimeAsk {
    fn from(value: &RuntimeAsk) -> Self {
        value.clone()
    }
}
///`RuntimeAskCorrelationId`
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
pub struct RuntimeAskCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeAskCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskCorrelationId> for ::std::string::String {
    fn from(value: RuntimeAskCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskCorrelationId> for RuntimeAskCorrelationId {
    fn from(value: &RuntimeAskCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeAskCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskCorrelationId {
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
///`RuntimeAskGenerationId`
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
pub struct RuntimeAskGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeAskGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskGenerationId> for ::std::string::String {
    fn from(value: RuntimeAskGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskGenerationId> for RuntimeAskGenerationId {
    fn from(value: &RuntimeAskGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeAskGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskGenerationId {
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
///`RuntimeAskKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.ask"
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
pub enum RuntimeAskKind {
    #[serde(rename = "runtime.ask")]
    RuntimeAsk,
}
impl ::std::convert::From<&Self> for RuntimeAskKind {
    fn from(value: &RuntimeAskKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeAskKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeAsk => f.write_str("runtime.ask"),
        }
    }
}
impl ::std::str::FromStr for RuntimeAskKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.ask" => Ok(Self::RuntimeAsk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeAskKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeAskLocale`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "en",
///    "vi"
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
pub enum RuntimeAskLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "vi")]
    Vi,
}
impl ::std::convert::From<&Self> for RuntimeAskLocale {
    fn from(value: &RuntimeAskLocale) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeAskLocale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::En => f.write_str("en"),
            Self::Vi => f.write_str("vi"),
        }
    }
}
impl ::std::str::FromStr for RuntimeAskLocale {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "en" => Ok(Self::En),
            "vi" => Ok(Self::Vi),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeAskLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeAskProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeAskProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeAskProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskProtocolVersion> for i64 {
    fn from(value: RuntimeAskProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskProtocolVersion> for RuntimeAskProtocolVersion {
    fn from(value: &RuntimeAskProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeAskProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeAskQuestion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 1000,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeAskQuestion(::std::string::String);
impl ::std::ops::Deref for RuntimeAskQuestion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskQuestion> for ::std::string::String {
    fn from(value: RuntimeAskQuestion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskQuestion> for RuntimeAskQuestion {
    fn from(value: &RuntimeAskQuestion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskQuestion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1000usize {
            return Err("longer than 1000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeAskQuestion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskQuestion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskQuestion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskQuestion {
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
///`RuntimeAskRequestId`
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
pub struct RuntimeAskRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeAskRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskRequestId> for ::std::string::String {
    fn from(value: RuntimeAskRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskRequestId> for RuntimeAskRequestId {
    fn from(value: &RuntimeAskRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeAskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskRequestId {
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
///`RuntimeAskResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeAskResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.askResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeAskResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeAskResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeAskResultGenerationId,
    pub kind: RuntimeAskResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeAskResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeAskResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeAskResult> for RuntimeAskResult {
    fn from(value: &RuntimeAskResult) -> Self {
        value.clone()
    }
}
///`RuntimeAskResultCorrelationId`
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
pub struct RuntimeAskResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeAskResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskResultCorrelationId> for ::std::string::String {
    fn from(value: RuntimeAskResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskResultCorrelationId>
for RuntimeAskResultCorrelationId {
    fn from(value: &RuntimeAskResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeAskResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskResultCorrelationId {
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
///`RuntimeAskResultGenerationId`
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
pub struct RuntimeAskResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeAskResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskResultGenerationId> for ::std::string::String {
    fn from(value: RuntimeAskResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskResultGenerationId>
for RuntimeAskResultGenerationId {
    fn from(value: &RuntimeAskResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeAskResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskResultGenerationId {
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
///`RuntimeAskResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.askResult"
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
pub enum RuntimeAskResultKind {
    #[serde(rename = "runtime.askResult")]
    RuntimeAskResult,
}
impl ::std::convert::From<&Self> for RuntimeAskResultKind {
    fn from(value: &RuntimeAskResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeAskResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeAskResult => f.write_str("runtime.askResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeAskResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.askResult" => Ok(Self::RuntimeAskResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeAskResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeAskResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeAskResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeAskResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskResultProtocolVersion> for i64 {
    fn from(value: RuntimeAskResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskResultProtocolVersion>
for RuntimeAskResultProtocolVersion {
    fn from(value: &RuntimeAskResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeAskResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeAskResultRequestId`
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
pub struct RuntimeAskResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeAskResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeAskResultRequestId> for ::std::string::String {
    fn from(value: RuntimeAskResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeAskResultRequestId> for RuntimeAskResultRequestId {
    fn from(value: &RuntimeAskResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeAskResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeAskResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeAskResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeAskResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeAskResultRequestId {
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
///`RuntimeCheck`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeCheck",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "cueId",
///    "expected",
///    "generationId",
///    "kind",
///    "label",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "cueId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "expected": {
///      "type": "string",
///      "maxLength": 256
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.check"
///      ]
///    },
///    "label": {
///      "type": "string",
///      "maxLength": 256
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCheck {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeCheckCorrelationId,
    #[serde(rename = "cueId")]
    pub cue_id: RuntimeCheckCueId,
    pub expected: RuntimeCheckExpected,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeCheckGenerationId,
    pub kind: RuntimeCheckKind,
    pub label: RuntimeCheckLabel,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeCheckProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeCheckRequestId,
}
impl ::std::convert::From<&RuntimeCheck> for RuntimeCheck {
    fn from(value: &RuntimeCheck) -> Self {
        value.clone()
    }
}
///`RuntimeCheckCorrelationId`
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
pub struct RuntimeCheckCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckCorrelationId> for ::std::string::String {
    fn from(value: RuntimeCheckCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckCorrelationId> for RuntimeCheckCorrelationId {
    fn from(value: &RuntimeCheckCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckCorrelationId {
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
///`RuntimeCheckCueId`
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
pub struct RuntimeCheckCueId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckCueId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckCueId> for ::std::string::String {
    fn from(value: RuntimeCheckCueId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckCueId> for RuntimeCheckCueId {
    fn from(value: &RuntimeCheckCueId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckCueId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckCueId {
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
///`RuntimeCheckExpected`
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
pub struct RuntimeCheckExpected(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckExpected {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckExpected> for ::std::string::String {
    fn from(value: RuntimeCheckExpected) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckExpected> for RuntimeCheckExpected {
    fn from(value: &RuntimeCheckExpected) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckExpected {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckExpected {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckExpected {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckExpected {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckExpected {
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
///`RuntimeCheckGenerationId`
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
pub struct RuntimeCheckGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckGenerationId> for ::std::string::String {
    fn from(value: RuntimeCheckGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckGenerationId> for RuntimeCheckGenerationId {
    fn from(value: &RuntimeCheckGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckGenerationId {
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
///`RuntimeCheckKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.check"
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
pub enum RuntimeCheckKind {
    #[serde(rename = "runtime.check")]
    RuntimeCheck,
}
impl ::std::convert::From<&Self> for RuntimeCheckKind {
    fn from(value: &RuntimeCheckKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeCheckKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeCheck => f.write_str("runtime.check"),
        }
    }
}
impl ::std::str::FromStr for RuntimeCheckKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.check" => Ok(Self::RuntimeCheck),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeCheckKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeCheckLabel`
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
pub struct RuntimeCheckLabel(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckLabel> for ::std::string::String {
    fn from(value: RuntimeCheckLabel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckLabel> for RuntimeCheckLabel {
    fn from(value: &RuntimeCheckLabel) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckLabel {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckLabel {
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
///`RuntimeCheckProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeCheckProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeCheckProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckProtocolVersion> for i64 {
    fn from(value: RuntimeCheckProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckProtocolVersion> for RuntimeCheckProtocolVersion {
    fn from(value: &RuntimeCheckProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeCheckProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeCheckRequestId`
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
pub struct RuntimeCheckRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckRequestId> for ::std::string::String {
    fn from(value: RuntimeCheckRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckRequestId> for RuntimeCheckRequestId {
    fn from(value: &RuntimeCheckRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckRequestId {
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
///`RuntimeCheckResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeCheckResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.checkResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCheckResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeCheckResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeCheckResultGenerationId,
    pub kind: RuntimeCheckResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeCheckResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeCheckResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeCheckResult> for RuntimeCheckResult {
    fn from(value: &RuntimeCheckResult) -> Self {
        value.clone()
    }
}
///`RuntimeCheckResultCorrelationId`
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
pub struct RuntimeCheckResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckResultCorrelationId> for ::std::string::String {
    fn from(value: RuntimeCheckResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckResultCorrelationId>
for RuntimeCheckResultCorrelationId {
    fn from(value: &RuntimeCheckResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeCheckResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckResultCorrelationId {
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
///`RuntimeCheckResultGenerationId`
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
pub struct RuntimeCheckResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckResultGenerationId> for ::std::string::String {
    fn from(value: RuntimeCheckResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckResultGenerationId>
for RuntimeCheckResultGenerationId {
    fn from(value: &RuntimeCheckResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckResultGenerationId {
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
///`RuntimeCheckResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.checkResult"
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
pub enum RuntimeCheckResultKind {
    #[serde(rename = "runtime.checkResult")]
    RuntimeCheckResult,
}
impl ::std::convert::From<&Self> for RuntimeCheckResultKind {
    fn from(value: &RuntimeCheckResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeCheckResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeCheckResult => f.write_str("runtime.checkResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeCheckResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.checkResult" => Ok(Self::RuntimeCheckResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeCheckResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeCheckResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeCheckResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeCheckResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckResultProtocolVersion> for i64 {
    fn from(value: RuntimeCheckResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckResultProtocolVersion>
for RuntimeCheckResultProtocolVersion {
    fn from(value: &RuntimeCheckResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeCheckResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeCheckResultRequestId`
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
pub struct RuntimeCheckResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeCheckResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeCheckResultRequestId> for ::std::string::String {
    fn from(value: RuntimeCheckResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeCheckResultRequestId> for RuntimeCheckResultRequestId {
    fn from(value: &RuntimeCheckResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeCheckResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeCheckResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeCheckResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeCheckResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeCheckResultRequestId {
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
///`RuntimeConfigure`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeConfigure",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "modelConfig",
///    "protocolVersion",
///    "requestId",
///    "storageRoot"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.configure"
///      ]
///    },
///    "modelConfig": {
///      "anyOf": [
///        {
///          "type": "null"
///        },
///        {
///          "type": "object",
///          "required": [
///            "grant",
///            "model",
///            "origin"
///          ],
///          "properties": {
///            "grant": {
///              "type": "string",
///              "maxLength": 64,
///              "minLength": 64
///            },
///            "model": {
///              "type": "string",
///              "maxLength": 128,
///              "minLength": 1
///            },
///            "origin": {
///              "type": "string",
///              "maxLength": 2048
///            }
///          },
///          "additionalProperties": false
///        }
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "storageRoot": {
///      "type": "string",
///      "maxLength": 4096,
///      "minLength": 1
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfigure {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeConfigureCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeConfigureGenerationId,
    pub kind: RuntimeConfigureKind,
    #[serde(rename = "modelConfig")]
    pub model_config: ::std::option::Option<RuntimeConfigureModelConfig>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeConfigureProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeConfigureRequestId,
    #[serde(rename = "storageRoot")]
    pub storage_root: RuntimeConfigureStorageRoot,
}
impl ::std::convert::From<&RuntimeConfigure> for RuntimeConfigure {
    fn from(value: &RuntimeConfigure) -> Self {
        value.clone()
    }
}
///`RuntimeConfigureCorrelationId`
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
pub struct RuntimeConfigureCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureCorrelationId> for ::std::string::String {
    fn from(value: RuntimeConfigureCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureCorrelationId>
for RuntimeConfigureCorrelationId {
    fn from(value: &RuntimeConfigureCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeConfigureCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfigureCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfigureCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureCorrelationId {
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
///`RuntimeConfigureGenerationId`
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
pub struct RuntimeConfigureGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureGenerationId> for ::std::string::String {
    fn from(value: RuntimeConfigureGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureGenerationId>
for RuntimeConfigureGenerationId {
    fn from(value: &RuntimeConfigureGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeConfigureGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfigureGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfigureGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureGenerationId {
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
///`RuntimeConfigureKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.configure"
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
pub enum RuntimeConfigureKind {
    #[serde(rename = "runtime.configure")]
    RuntimeConfigure,
}
impl ::std::convert::From<&Self> for RuntimeConfigureKind {
    fn from(value: &RuntimeConfigureKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeConfigureKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeConfigure => f.write_str("runtime.configure"),
        }
    }
}
impl ::std::str::FromStr for RuntimeConfigureKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.configure" => Ok(Self::RuntimeConfigure),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeConfigureKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfigureKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfigureKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeConfigureModelConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "grant",
///    "model",
///    "origin"
///  ],
///  "properties": {
///    "grant": {
///      "type": "string",
///      "maxLength": 64,
///      "minLength": 64
///    },
///    "model": {
///      "type": "string",
///      "maxLength": 128,
///      "minLength": 1
///    },
///    "origin": {
///      "type": "string",
///      "maxLength": 2048
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfigureModelConfig {
    pub grant: RuntimeConfigureModelConfigGrant,
    pub model: RuntimeConfigureModelConfigModel,
    pub origin: RuntimeConfigureModelConfigOrigin,
}
impl ::std::convert::From<&RuntimeConfigureModelConfig> for RuntimeConfigureModelConfig {
    fn from(value: &RuntimeConfigureModelConfig) -> Self {
        value.clone()
    }
}
///`RuntimeConfigureModelConfigGrant`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 64,
///  "minLength": 64
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeConfigureModelConfigGrant(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureModelConfigGrant {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureModelConfigGrant> for ::std::string::String {
    fn from(value: RuntimeConfigureModelConfigGrant) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureModelConfigGrant>
for RuntimeConfigureModelConfigGrant {
    fn from(value: &RuntimeConfigureModelConfigGrant) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureModelConfigGrant {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 64usize {
            return Err("shorter than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeConfigureModelConfigGrant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeConfigureModelConfigGrant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeConfigureModelConfigGrant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureModelConfigGrant {
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
///`RuntimeConfigureModelConfigModel`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 128,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeConfigureModelConfigModel(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureModelConfigModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureModelConfigModel> for ::std::string::String {
    fn from(value: RuntimeConfigureModelConfigModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureModelConfigModel>
for RuntimeConfigureModelConfigModel {
    fn from(value: &RuntimeConfigureModelConfigModel) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureModelConfigModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeConfigureModelConfigModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeConfigureModelConfigModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeConfigureModelConfigModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureModelConfigModel {
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
///`RuntimeConfigureModelConfigOrigin`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 2048
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeConfigureModelConfigOrigin(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureModelConfigOrigin {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureModelConfigOrigin> for ::std::string::String {
    fn from(value: RuntimeConfigureModelConfigOrigin) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureModelConfigOrigin>
for RuntimeConfigureModelConfigOrigin {
    fn from(value: &RuntimeConfigureModelConfigOrigin) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureModelConfigOrigin {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2048usize {
            return Err("longer than 2048 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeConfigureModelConfigOrigin {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeConfigureModelConfigOrigin {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeConfigureModelConfigOrigin {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureModelConfigOrigin {
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
///`RuntimeConfigureProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeConfigureProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeConfigureProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureProtocolVersion> for i64 {
    fn from(value: RuntimeConfigureProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureProtocolVersion>
for RuntimeConfigureProtocolVersion {
    fn from(value: &RuntimeConfigureProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeConfigureProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeConfigureRequestId`
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
pub struct RuntimeConfigureRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureRequestId> for ::std::string::String {
    fn from(value: RuntimeConfigureRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureRequestId> for RuntimeConfigureRequestId {
    fn from(value: &RuntimeConfigureRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeConfigureRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfigureRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfigureRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureRequestId {
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
///`RuntimeConfigureStorageRoot`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 4096,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeConfigureStorageRoot(::std::string::String);
impl ::std::ops::Deref for RuntimeConfigureStorageRoot {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfigureStorageRoot> for ::std::string::String {
    fn from(value: RuntimeConfigureStorageRoot) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfigureStorageRoot> for RuntimeConfigureStorageRoot {
    fn from(value: &RuntimeConfigureStorageRoot) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfigureStorageRoot {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeConfigureStorageRoot {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfigureStorageRoot {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfigureStorageRoot {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfigureStorageRoot {
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
///`RuntimeConfigured`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeConfigured",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.configured"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfigured {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeConfiguredCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeConfiguredGenerationId,
    pub kind: RuntimeConfiguredKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeConfiguredProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeConfiguredRequestId,
}
impl ::std::convert::From<&RuntimeConfigured> for RuntimeConfigured {
    fn from(value: &RuntimeConfigured) -> Self {
        value.clone()
    }
}
///`RuntimeConfiguredCorrelationId`
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
pub struct RuntimeConfiguredCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeConfiguredCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfiguredCorrelationId> for ::std::string::String {
    fn from(value: RuntimeConfiguredCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfiguredCorrelationId>
for RuntimeConfiguredCorrelationId {
    fn from(value: &RuntimeConfiguredCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfiguredCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeConfiguredCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfiguredCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfiguredCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfiguredCorrelationId {
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
///`RuntimeConfiguredGenerationId`
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
pub struct RuntimeConfiguredGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeConfiguredGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfiguredGenerationId> for ::std::string::String {
    fn from(value: RuntimeConfiguredGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfiguredGenerationId>
for RuntimeConfiguredGenerationId {
    fn from(value: &RuntimeConfiguredGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfiguredGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeConfiguredGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfiguredGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfiguredGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfiguredGenerationId {
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
///`RuntimeConfiguredKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.configured"
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
pub enum RuntimeConfiguredKind {
    #[serde(rename = "runtime.configured")]
    RuntimeConfigured,
}
impl ::std::convert::From<&Self> for RuntimeConfiguredKind {
    fn from(value: &RuntimeConfiguredKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeConfiguredKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeConfigured => f.write_str("runtime.configured"),
        }
    }
}
impl ::std::str::FromStr for RuntimeConfiguredKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.configured" => Ok(Self::RuntimeConfigured),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeConfiguredKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfiguredKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfiguredKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeConfiguredProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeConfiguredProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeConfiguredProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfiguredProtocolVersion> for i64 {
    fn from(value: RuntimeConfiguredProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfiguredProtocolVersion>
for RuntimeConfiguredProtocolVersion {
    fn from(value: &RuntimeConfiguredProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeConfiguredProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfiguredProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeConfiguredRequestId`
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
pub struct RuntimeConfiguredRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeConfiguredRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeConfiguredRequestId> for ::std::string::String {
    fn from(value: RuntimeConfiguredRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeConfiguredRequestId> for RuntimeConfiguredRequestId {
    fn from(value: &RuntimeConfiguredRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeConfiguredRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeConfiguredRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeConfiguredRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeConfiguredRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeConfiguredRequestId {
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
///`RuntimeError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeError",
///  "type": "object",
///  "required": [
///    "code",
///    "correlationId",
///    "generationId",
///    "kind",
///    "message",
///    "protocolVersion",
///    "requestId",
///    "retryable"
///  ],
///  "properties": {
///    "code": {
///      "type": "string",
///      "enum": [
///        "INVALID_MESSAGE",
///        "PROTOCOL_MISMATCH",
///        "NOT_READY",
///        "BUSY",
///        "TIMEOUT",
///        "WORKER_EXITED",
///        "UNAUTHORIZED",
///        "FORBIDDEN",
///        "INTERNAL"
///      ]
///    },
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.error"
///      ]
///    },
///    "message": {
///      "type": "string",
///      "maxLength": 256,
///      "minLength": 1
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "retryable": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeError {
    pub code: RuntimeErrorCode,
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeErrorCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeErrorGenerationId,
    pub kind: RuntimeErrorKind,
    pub message: RuntimeErrorMessage,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeErrorProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeErrorRequestId,
    pub retryable: bool,
}
impl ::std::convert::From<&RuntimeError> for RuntimeError {
    fn from(value: &RuntimeError) -> Self {
        value.clone()
    }
}
///`RuntimeErrorCode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "INVALID_MESSAGE",
///    "PROTOCOL_MISMATCH",
///    "NOT_READY",
///    "BUSY",
///    "TIMEOUT",
///    "WORKER_EXITED",
///    "UNAUTHORIZED",
///    "FORBIDDEN",
///    "INTERNAL"
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
pub enum RuntimeErrorCode {
    #[serde(rename = "INVALID_MESSAGE")]
    InvalidMessage,
    #[serde(rename = "PROTOCOL_MISMATCH")]
    ProtocolMismatch,
    #[serde(rename = "NOT_READY")]
    NotReady,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(rename = "WORKER_EXITED")]
    WorkerExited,
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
    #[serde(rename = "INTERNAL")]
    Internal,
}
impl ::std::convert::From<&Self> for RuntimeErrorCode {
    fn from(value: &RuntimeErrorCode) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InvalidMessage => f.write_str("INVALID_MESSAGE"),
            Self::ProtocolMismatch => f.write_str("PROTOCOL_MISMATCH"),
            Self::NotReady => f.write_str("NOT_READY"),
            Self::Busy => f.write_str("BUSY"),
            Self::Timeout => f.write_str("TIMEOUT"),
            Self::WorkerExited => f.write_str("WORKER_EXITED"),
            Self::Unauthorized => f.write_str("UNAUTHORIZED"),
            Self::Forbidden => f.write_str("FORBIDDEN"),
            Self::Internal => f.write_str("INTERNAL"),
        }
    }
}
impl ::std::str::FromStr for RuntimeErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "INVALID_MESSAGE" => Ok(Self::InvalidMessage),
            "PROTOCOL_MISMATCH" => Ok(Self::ProtocolMismatch),
            "NOT_READY" => Ok(Self::NotReady),
            "BUSY" => Ok(Self::Busy),
            "TIMEOUT" => Ok(Self::Timeout),
            "WORKER_EXITED" => Ok(Self::WorkerExited),
            "UNAUTHORIZED" => Ok(Self::Unauthorized),
            "FORBIDDEN" => Ok(Self::Forbidden),
            "INTERNAL" => Ok(Self::Internal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeErrorCorrelationId`
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
pub struct RuntimeErrorCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeErrorCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeErrorCorrelationId> for ::std::string::String {
    fn from(value: RuntimeErrorCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeErrorCorrelationId> for RuntimeErrorCorrelationId {
    fn from(value: &RuntimeErrorCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeErrorCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeErrorCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeErrorCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeErrorCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeErrorCorrelationId {
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
///`RuntimeErrorGenerationId`
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
pub struct RuntimeErrorGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeErrorGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeErrorGenerationId> for ::std::string::String {
    fn from(value: RuntimeErrorGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeErrorGenerationId> for RuntimeErrorGenerationId {
    fn from(value: &RuntimeErrorGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeErrorGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeErrorGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeErrorGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeErrorGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeErrorGenerationId {
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
///`RuntimeErrorKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.error"
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
pub enum RuntimeErrorKind {
    #[serde(rename = "runtime.error")]
    RuntimeError,
}
impl ::std::convert::From<&Self> for RuntimeErrorKind {
    fn from(value: &RuntimeErrorKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeError => f.write_str("runtime.error"),
        }
    }
}
impl ::std::str::FromStr for RuntimeErrorKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.error" => Ok(Self::RuntimeError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeErrorKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeErrorKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeErrorKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeErrorMessage`
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
pub struct RuntimeErrorMessage(::std::string::String);
impl ::std::ops::Deref for RuntimeErrorMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeErrorMessage> for ::std::string::String {
    fn from(value: RuntimeErrorMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeErrorMessage> for RuntimeErrorMessage {
    fn from(value: &RuntimeErrorMessage) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeErrorMessage {
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
impl ::std::convert::TryFrom<&str> for RuntimeErrorMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeErrorMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeErrorMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeErrorMessage {
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
///`RuntimeErrorProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeErrorProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeErrorProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeErrorProtocolVersion> for i64 {
    fn from(value: RuntimeErrorProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeErrorProtocolVersion> for RuntimeErrorProtocolVersion {
    fn from(value: &RuntimeErrorProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeErrorProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeErrorProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeErrorRequestId`
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
pub struct RuntimeErrorRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeErrorRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeErrorRequestId> for ::std::string::String {
    fn from(value: RuntimeErrorRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeErrorRequestId> for RuntimeErrorRequestId {
    fn from(value: &RuntimeErrorRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeErrorRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeErrorRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeErrorRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeErrorRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeErrorRequestId {
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
///`RuntimeExplain`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeExplain",
///  "type": "object",
///  "required": [
///    "caption",
///    "correlationId",
///    "destinationId",
///    "direction",
///    "elementId",
///    "generationId",
///    "gesture",
///    "kind",
///    "locale",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "caption": {
///      "type": "string",
///      "maxLength": 400,
///      "minLength": 1
///    },
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "destinationId": {
///      "anyOf": [
///        {
///          "type": "string",
///          "maxLength": 80
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "direction": {
///      "anyOf": [
///        {
///          "type": "string",
///          "enum": [
///            "up",
///            "down",
///            "left",
///            "right"
///          ]
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "elementId": {
///      "type": "string",
///      "maxLength": 80
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "gesture": {
///      "type": "string",
///      "enum": [
///        "point",
///        "click",
///        "drag",
///        "type",
///        "scroll"
///      ]
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.explain"
///      ]
///    },
///    "locale": {
///      "type": "string",
///      "enum": [
///        "en",
///        "vi"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeExplain {
    pub caption: RuntimeExplainCaption,
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeExplainCorrelationId,
    #[serde(rename = "destinationId")]
    pub destination_id: ::std::option::Option<RuntimeExplainDestinationId>,
    pub direction: ::std::option::Option<RuntimeExplainDirection>,
    #[serde(rename = "elementId")]
    pub element_id: RuntimeExplainElementId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeExplainGenerationId,
    pub gesture: RuntimeExplainGesture,
    pub kind: RuntimeExplainKind,
    pub locale: RuntimeExplainLocale,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeExplainProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeExplainRequestId,
}
impl ::std::convert::From<&RuntimeExplain> for RuntimeExplain {
    fn from(value: &RuntimeExplain) -> Self {
        value.clone()
    }
}
///`RuntimeExplainCaption`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 400,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeExplainCaption(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainCaption {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainCaption> for ::std::string::String {
    fn from(value: RuntimeExplainCaption) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainCaption> for RuntimeExplainCaption {
    fn from(value: &RuntimeExplainCaption) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainCaption {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 400usize {
            return Err("longer than 400 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainCaption {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainCaption {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainCaption {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainCaption {
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
///`RuntimeExplainCorrelationId`
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
pub struct RuntimeExplainCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainCorrelationId> for ::std::string::String {
    fn from(value: RuntimeExplainCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainCorrelationId> for RuntimeExplainCorrelationId {
    fn from(value: &RuntimeExplainCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeExplainCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainCorrelationId {
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
///`RuntimeExplainDestinationId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 80
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeExplainDestinationId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainDestinationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainDestinationId> for ::std::string::String {
    fn from(value: RuntimeExplainDestinationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainDestinationId> for RuntimeExplainDestinationId {
    fn from(value: &RuntimeExplainDestinationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainDestinationId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 80usize {
            return Err("longer than 80 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainDestinationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainDestinationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainDestinationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainDestinationId {
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
///`RuntimeExplainDirection`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "up",
///    "down",
///    "left",
///    "right"
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
pub enum RuntimeExplainDirection {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}
impl ::std::convert::From<&Self> for RuntimeExplainDirection {
    fn from(value: &RuntimeExplainDirection) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeExplainDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
        }
    }
}
impl ::std::str::FromStr for RuntimeExplainDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeExplainElementId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 80
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeExplainElementId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainElementId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainElementId> for ::std::string::String {
    fn from(value: RuntimeExplainElementId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainElementId> for RuntimeExplainElementId {
    fn from(value: &RuntimeExplainElementId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainElementId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 80usize {
            return Err("longer than 80 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainElementId {
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
///`RuntimeExplainGenerationId`
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
pub struct RuntimeExplainGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainGenerationId> for ::std::string::String {
    fn from(value: RuntimeExplainGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainGenerationId> for RuntimeExplainGenerationId {
    fn from(value: &RuntimeExplainGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeExplainGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainGenerationId {
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
///`RuntimeExplainGesture`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "point",
///    "click",
///    "drag",
///    "type",
///    "scroll"
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
pub enum RuntimeExplainGesture {
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "drag")]
    Drag,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "scroll")]
    Scroll,
}
impl ::std::convert::From<&Self> for RuntimeExplainGesture {
    fn from(value: &RuntimeExplainGesture) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeExplainGesture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Point => f.write_str("point"),
            Self::Click => f.write_str("click"),
            Self::Drag => f.write_str("drag"),
            Self::Type => f.write_str("type"),
            Self::Scroll => f.write_str("scroll"),
        }
    }
}
impl ::std::str::FromStr for RuntimeExplainGesture {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "point" => Ok(Self::Point),
            "click" => Ok(Self::Click),
            "drag" => Ok(Self::Drag),
            "type" => Ok(Self::Type),
            "scroll" => Ok(Self::Scroll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainGesture {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainGesture {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainGesture {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeExplainKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.explain"
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
pub enum RuntimeExplainKind {
    #[serde(rename = "runtime.explain")]
    RuntimeExplain,
}
impl ::std::convert::From<&Self> for RuntimeExplainKind {
    fn from(value: &RuntimeExplainKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeExplainKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeExplain => f.write_str("runtime.explain"),
        }
    }
}
impl ::std::str::FromStr for RuntimeExplainKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.explain" => Ok(Self::RuntimeExplain),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeExplainLocale`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "en",
///    "vi"
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
pub enum RuntimeExplainLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "vi")]
    Vi,
}
impl ::std::convert::From<&Self> for RuntimeExplainLocale {
    fn from(value: &RuntimeExplainLocale) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeExplainLocale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::En => f.write_str("en"),
            Self::Vi => f.write_str("vi"),
        }
    }
}
impl ::std::str::FromStr for RuntimeExplainLocale {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "en" => Ok(Self::En),
            "vi" => Ok(Self::Vi),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeExplainProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeExplainProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeExplainProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainProtocolVersion> for i64 {
    fn from(value: RuntimeExplainProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainProtocolVersion>
for RuntimeExplainProtocolVersion {
    fn from(value: &RuntimeExplainProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeExplainProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeExplainRequestId`
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
pub struct RuntimeExplainRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainRequestId> for ::std::string::String {
    fn from(value: RuntimeExplainRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainRequestId> for RuntimeExplainRequestId {
    fn from(value: &RuntimeExplainRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeExplainRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainRequestId {
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
///`RuntimeExplainResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeExplainResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.explanationResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeExplainResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeExplainResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeExplainResultGenerationId,
    pub kind: RuntimeExplainResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeExplainResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeExplainResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeExplainResult> for RuntimeExplainResult {
    fn from(value: &RuntimeExplainResult) -> Self {
        value.clone()
    }
}
///`RuntimeExplainResultCorrelationId`
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
pub struct RuntimeExplainResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainResultCorrelationId> for ::std::string::String {
    fn from(value: RuntimeExplainResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainResultCorrelationId>
for RuntimeExplainResultCorrelationId {
    fn from(value: &RuntimeExplainResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeExplainResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeExplainResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeExplainResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainResultCorrelationId {
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
///`RuntimeExplainResultGenerationId`
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
pub struct RuntimeExplainResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainResultGenerationId> for ::std::string::String {
    fn from(value: RuntimeExplainResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainResultGenerationId>
for RuntimeExplainResultGenerationId {
    fn from(value: &RuntimeExplainResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeExplainResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeExplainResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeExplainResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainResultGenerationId {
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
///`RuntimeExplainResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.explanationResult"
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
pub enum RuntimeExplainResultKind {
    #[serde(rename = "runtime.explanationResult")]
    RuntimeExplanationResult,
}
impl ::std::convert::From<&Self> for RuntimeExplainResultKind {
    fn from(value: &RuntimeExplainResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeExplainResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeExplanationResult => f.write_str("runtime.explanationResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeExplainResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.explanationResult" => Ok(Self::RuntimeExplanationResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeExplainResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeExplainResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeExplainResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeExplainResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainResultProtocolVersion> for i64 {
    fn from(value: RuntimeExplainResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainResultProtocolVersion>
for RuntimeExplainResultProtocolVersion {
    fn from(value: &RuntimeExplainResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeExplainResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeExplainResultRequestId`
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
pub struct RuntimeExplainResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeExplainResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeExplainResultRequestId> for ::std::string::String {
    fn from(value: RuntimeExplainResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeExplainResultRequestId>
for RuntimeExplainResultRequestId {
    fn from(value: &RuntimeExplainResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeExplainResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeExplainResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeExplainResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeExplainResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeExplainResultRequestId {
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
///`RuntimeHealth`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeHealth",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.health"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeHealth {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeHealthCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeHealthGenerationId,
    pub kind: RuntimeHealthKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeHealthProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeHealthRequestId,
}
impl ::std::convert::From<&RuntimeHealth> for RuntimeHealth {
    fn from(value: &RuntimeHealth) -> Self {
        value.clone()
    }
}
///`RuntimeHealthCorrelationId`
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
pub struct RuntimeHealthCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeHealthCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthCorrelationId> for ::std::string::String {
    fn from(value: RuntimeHealthCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthCorrelationId> for RuntimeHealthCorrelationId {
    fn from(value: &RuntimeHealthCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeHealthCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeHealthCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthCorrelationId {
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
///`RuntimeHealthGenerationId`
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
pub struct RuntimeHealthGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeHealthGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthGenerationId> for ::std::string::String {
    fn from(value: RuntimeHealthGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthGenerationId> for RuntimeHealthGenerationId {
    fn from(value: &RuntimeHealthGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeHealthGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeHealthGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthGenerationId {
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
///`RuntimeHealthKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.health"
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
pub enum RuntimeHealthKind {
    #[serde(rename = "runtime.health")]
    RuntimeHealth,
}
impl ::std::convert::From<&Self> for RuntimeHealthKind {
    fn from(value: &RuntimeHealthKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeHealthKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeHealth => f.write_str("runtime.health"),
        }
    }
}
impl ::std::str::FromStr for RuntimeHealthKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.health" => Ok(Self::RuntimeHealth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeHealthKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeHealthProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeHealthProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeHealthProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthProtocolVersion> for i64 {
    fn from(value: RuntimeHealthProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthProtocolVersion>
for RuntimeHealthProtocolVersion {
    fn from(value: &RuntimeHealthProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeHealthProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeHealthRequestId`
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
pub struct RuntimeHealthRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeHealthRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthRequestId> for ::std::string::String {
    fn from(value: RuntimeHealthRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthRequestId> for RuntimeHealthRequestId {
    fn from(value: &RuntimeHealthRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeHealthRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeHealthRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthRequestId {
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
///`RuntimeHealthResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeHealthResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.healthResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "type": "string",
///      "enum": [
///        "ready",
///        "running",
///        "stopped"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeHealthResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeHealthResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeHealthResultGenerationId,
    pub kind: RuntimeHealthResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeHealthResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeHealthResultRequestId,
    pub state: RuntimeHealthResultState,
}
impl ::std::convert::From<&RuntimeHealthResult> for RuntimeHealthResult {
    fn from(value: &RuntimeHealthResult) -> Self {
        value.clone()
    }
}
///`RuntimeHealthResultCorrelationId`
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
pub struct RuntimeHealthResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeHealthResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthResultCorrelationId> for ::std::string::String {
    fn from(value: RuntimeHealthResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthResultCorrelationId>
for RuntimeHealthResultCorrelationId {
    fn from(value: &RuntimeHealthResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeHealthResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeHealthResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeHealthResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeHealthResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthResultCorrelationId {
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
///`RuntimeHealthResultGenerationId`
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
pub struct RuntimeHealthResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeHealthResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthResultGenerationId> for ::std::string::String {
    fn from(value: RuntimeHealthResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthResultGenerationId>
for RuntimeHealthResultGenerationId {
    fn from(value: &RuntimeHealthResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeHealthResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeHealthResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeHealthResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthResultGenerationId {
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
///`RuntimeHealthResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.healthResult"
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
pub enum RuntimeHealthResultKind {
    #[serde(rename = "runtime.healthResult")]
    RuntimeHealthResult,
}
impl ::std::convert::From<&Self> for RuntimeHealthResultKind {
    fn from(value: &RuntimeHealthResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeHealthResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeHealthResult => f.write_str("runtime.healthResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeHealthResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.healthResult" => Ok(Self::RuntimeHealthResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeHealthResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeHealthResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeHealthResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeHealthResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthResultProtocolVersion> for i64 {
    fn from(value: RuntimeHealthResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthResultProtocolVersion>
for RuntimeHealthResultProtocolVersion {
    fn from(value: &RuntimeHealthResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeHealthResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeHealthResultRequestId`
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
pub struct RuntimeHealthResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeHealthResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeHealthResultRequestId> for ::std::string::String {
    fn from(value: RuntimeHealthResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeHealthResultRequestId>
for RuntimeHealthResultRequestId {
    fn from(value: &RuntimeHealthResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeHealthResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeHealthResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeHealthResultRequestId {
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
///`RuntimeHealthResultState`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ready",
///    "running",
///    "stopped"
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
pub enum RuntimeHealthResultState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}
impl ::std::convert::From<&Self> for RuntimeHealthResultState {
    fn from(value: &RuntimeHealthResultState) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeHealthResultState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ready => f.write_str("ready"),
            Self::Running => f.write_str("running"),
            Self::Stopped => f.write_str("stopped"),
        }
    }
}
impl ::std::str::FromStr for RuntimeHealthResultState {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ready" => Ok(Self::Ready),
            "running" => Ok(Self::Running),
            "stopped" => Ok(Self::Stopped),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeHealthResultState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeHealthResultState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeHealthResultState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeInitialize`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeInitialize",
///  "type": "object",
///  "required": [
///    "accountId",
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "schemaDigest"
///  ],
///  "properties": {
///    "accountId": {
///      "type": [
///        "string",
///        "null"
///      ],
///      "maxLength": 128
///    },
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.initialize"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "schemaDigest": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{64}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInitialize {
    #[serde(rename = "accountId")]
    pub account_id: ::std::option::Option<RuntimeInitializeAccountId>,
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeInitializeCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeInitializeGenerationId,
    pub kind: RuntimeInitializeKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeInitializeProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeInitializeRequestId,
    #[serde(rename = "schemaDigest")]
    pub schema_digest: RuntimeInitializeSchemaDigest,
}
impl ::std::convert::From<&RuntimeInitialize> for RuntimeInitialize {
    fn from(value: &RuntimeInitialize) -> Self {
        value.clone()
    }
}
///`RuntimeInitializeAccountId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 128
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeInitializeAccountId(::std::string::String);
impl ::std::ops::Deref for RuntimeInitializeAccountId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeInitializeAccountId> for ::std::string::String {
    fn from(value: RuntimeInitializeAccountId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeInitializeAccountId> for RuntimeInitializeAccountId {
    fn from(value: &RuntimeInitializeAccountId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeInitializeAccountId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeInitializeAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeInitializeAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeInitializeAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeInitializeAccountId {
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
///`RuntimeInitializeCorrelationId`
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
pub struct RuntimeInitializeCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeInitializeCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeInitializeCorrelationId> for ::std::string::String {
    fn from(value: RuntimeInitializeCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeInitializeCorrelationId>
for RuntimeInitializeCorrelationId {
    fn from(value: &RuntimeInitializeCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeInitializeCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeInitializeCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeInitializeCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeInitializeCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeInitializeCorrelationId {
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
///`RuntimeInitializeGenerationId`
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
pub struct RuntimeInitializeGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeInitializeGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeInitializeGenerationId> for ::std::string::String {
    fn from(value: RuntimeInitializeGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeInitializeGenerationId>
for RuntimeInitializeGenerationId {
    fn from(value: &RuntimeInitializeGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeInitializeGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeInitializeGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeInitializeGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeInitializeGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeInitializeGenerationId {
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
///`RuntimeInitializeKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.initialize"
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
pub enum RuntimeInitializeKind {
    #[serde(rename = "runtime.initialize")]
    RuntimeInitialize,
}
impl ::std::convert::From<&Self> for RuntimeInitializeKind {
    fn from(value: &RuntimeInitializeKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeInitializeKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeInitialize => f.write_str("runtime.initialize"),
        }
    }
}
impl ::std::str::FromStr for RuntimeInitializeKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.initialize" => Ok(Self::RuntimeInitialize),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeInitializeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeInitializeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeInitializeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeInitializeProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeInitializeProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeInitializeProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeInitializeProtocolVersion> for i64 {
    fn from(value: RuntimeInitializeProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeInitializeProtocolVersion>
for RuntimeInitializeProtocolVersion {
    fn from(value: &RuntimeInitializeProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeInitializeProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeInitializeProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeInitializeRequestId`
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
pub struct RuntimeInitializeRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeInitializeRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeInitializeRequestId> for ::std::string::String {
    fn from(value: RuntimeInitializeRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeInitializeRequestId> for RuntimeInitializeRequestId {
    fn from(value: &RuntimeInitializeRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeInitializeRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeInitializeRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeInitializeRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeInitializeRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeInitializeRequestId {
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
///`RuntimeInitializeSchemaDigest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{64}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeInitializeSchemaDigest(::std::string::String);
impl ::std::ops::Deref for RuntimeInitializeSchemaDigest {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeInitializeSchemaDigest> for ::std::string::String {
    fn from(value: RuntimeInitializeSchemaDigest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeInitializeSchemaDigest>
for RuntimeInitializeSchemaDigest {
    fn from(value: &RuntimeInitializeSchemaDigest) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeInitializeSchemaDigest {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9a-f]{64}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{64}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeInitializeSchemaDigest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeInitializeSchemaDigest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeInitializeSchemaDigest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeInitializeSchemaDigest {
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
///`RuntimeListTargets`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeListTargets",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.listTargets"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeListTargets {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeListTargetsCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeListTargetsGenerationId,
    pub kind: RuntimeListTargetsKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeListTargetsProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeListTargetsRequestId,
}
impl ::std::convert::From<&RuntimeListTargets> for RuntimeListTargets {
    fn from(value: &RuntimeListTargets) -> Self {
        value.clone()
    }
}
///`RuntimeListTargetsCorrelationId`
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
pub struct RuntimeListTargetsCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeListTargetsCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsCorrelationId> for ::std::string::String {
    fn from(value: RuntimeListTargetsCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsCorrelationId>
for RuntimeListTargetsCorrelationId {
    fn from(value: &RuntimeListTargetsCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeListTargetsCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeListTargetsCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeListTargetsCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsCorrelationId {
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
///`RuntimeListTargetsGenerationId`
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
pub struct RuntimeListTargetsGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeListTargetsGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsGenerationId> for ::std::string::String {
    fn from(value: RuntimeListTargetsGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsGenerationId>
for RuntimeListTargetsGenerationId {
    fn from(value: &RuntimeListTargetsGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeListTargetsGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeListTargetsGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeListTargetsGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsGenerationId {
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
///`RuntimeListTargetsKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.listTargets"
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
pub enum RuntimeListTargetsKind {
    #[serde(rename = "runtime.listTargets")]
    RuntimeListTargets,
}
impl ::std::convert::From<&Self> for RuntimeListTargetsKind {
    fn from(value: &RuntimeListTargetsKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeListTargetsKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeListTargets => f.write_str("runtime.listTargets"),
        }
    }
}
impl ::std::str::FromStr for RuntimeListTargetsKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.listTargets" => Ok(Self::RuntimeListTargets),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeListTargetsKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeListTargetsKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeListTargetsProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeListTargetsProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeListTargetsProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsProtocolVersion> for i64 {
    fn from(value: RuntimeListTargetsProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsProtocolVersion>
for RuntimeListTargetsProtocolVersion {
    fn from(value: &RuntimeListTargetsProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeListTargetsProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeListTargetsRequestId`
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
pub struct RuntimeListTargetsRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeListTargetsRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsRequestId> for ::std::string::String {
    fn from(value: RuntimeListTargetsRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsRequestId> for RuntimeListTargetsRequestId {
    fn from(value: &RuntimeListTargetsRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeListTargetsRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeListTargetsRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeListTargetsRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsRequestId {
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
///`RuntimeListTargetsResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeListTargetsResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.targetsResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeListTargetsResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeListTargetsResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeListTargetsResultGenerationId,
    pub kind: RuntimeListTargetsResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeListTargetsResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeListTargetsResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeListTargetsResult> for RuntimeListTargetsResult {
    fn from(value: &RuntimeListTargetsResult) -> Self {
        value.clone()
    }
}
///`RuntimeListTargetsResultCorrelationId`
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
pub struct RuntimeListTargetsResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeListTargetsResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsResultCorrelationId>
for ::std::string::String {
    fn from(value: RuntimeListTargetsResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsResultCorrelationId>
for RuntimeListTargetsResultCorrelationId {
    fn from(value: &RuntimeListTargetsResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeListTargetsResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeListTargetsResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeListTargetsResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsResultCorrelationId {
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
///`RuntimeListTargetsResultGenerationId`
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
pub struct RuntimeListTargetsResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeListTargetsResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsResultGenerationId>
for ::std::string::String {
    fn from(value: RuntimeListTargetsResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsResultGenerationId>
for RuntimeListTargetsResultGenerationId {
    fn from(value: &RuntimeListTargetsResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeListTargetsResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeListTargetsResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeListTargetsResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsResultGenerationId {
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
///`RuntimeListTargetsResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.targetsResult"
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
pub enum RuntimeListTargetsResultKind {
    #[serde(rename = "runtime.targetsResult")]
    RuntimeTargetsResult,
}
impl ::std::convert::From<&Self> for RuntimeListTargetsResultKind {
    fn from(value: &RuntimeListTargetsResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeListTargetsResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeTargetsResult => f.write_str("runtime.targetsResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeListTargetsResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.targetsResult" => Ok(Self::RuntimeTargetsResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeListTargetsResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeListTargetsResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeListTargetsResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeListTargetsResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeListTargetsResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsResultProtocolVersion> for i64 {
    fn from(value: RuntimeListTargetsResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsResultProtocolVersion>
for RuntimeListTargetsResultProtocolVersion {
    fn from(value: &RuntimeListTargetsResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeListTargetsResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeListTargetsResultRequestId`
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
pub struct RuntimeListTargetsResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeListTargetsResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeListTargetsResultRequestId> for ::std::string::String {
    fn from(value: RuntimeListTargetsResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeListTargetsResultRequestId>
for RuntimeListTargetsResultRequestId {
    fn from(value: &RuntimeListTargetsResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeListTargetsResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeListTargetsResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeListTargetsResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeListTargetsResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeListTargetsResultRequestId {
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
///`RuntimeMessage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeMessage",
///  "oneOf": [
///    {
///      "$ref": "#/definitions/RuntimeInitialize"
///    },
///    {
///      "$ref": "#/definitions/RuntimeReady"
///    },
///    {
///      "$ref": "#/definitions/RuntimeHealth"
///    },
///    {
///      "$ref": "#/definitions/RuntimeHealthResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeStart"
///    },
///    {
///      "$ref": "#/definitions/RuntimeStarted"
///    },
///    {
///      "$ref": "#/definitions/RuntimeStop"
///    },
///    {
///      "$ref": "#/definitions/RuntimeStopped"
///    },
///    {
///      "$ref": "#/definitions/RuntimeShutdown"
///    },
///    {
///      "$ref": "#/definitions/RuntimeShutdownComplete"
///    },
///    {
///      "$ref": "#/definitions/RuntimeError"
///    },
///    {
///      "$ref": "#/definitions/RuntimeListTargets"
///    },
///    {
///      "$ref": "#/definitions/RuntimeListTargetsResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeSelectTarget"
///    },
///    {
///      "$ref": "#/definitions/RuntimeSelectTargetResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeObserve"
///    },
///    {
///      "$ref": "#/definitions/RuntimeObserveResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeExplain"
///    },
///    {
///      "$ref": "#/definitions/RuntimeExplainResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeCheck"
///    },
///    {
///      "$ref": "#/definitions/RuntimeCheckResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimePresentationAck"
///    },
///    {
///      "$ref": "#/definitions/RuntimePresentationAckResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeConfigure"
///    },
///    {
///      "$ref": "#/definitions/RuntimeConfigured"
///    },
///    {
///      "$ref": "#/definitions/RuntimeAsk"
///    },
///    {
///      "$ref": "#/definitions/RuntimeAskResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimeRefreshCue"
///    },
///    {
///      "$ref": "#/definitions/RuntimeRefreshCueResult"
///    },
///    {
///      "$ref": "#/definitions/RuntimePlanControl"
///    },
///    {
///      "$ref": "#/definitions/RuntimePlanControlResult"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RuntimeMessage {
    Initialize(RuntimeInitialize),
    Ready(RuntimeReady),
    Health(RuntimeHealth),
    HealthResult(RuntimeHealthResult),
    Start(RuntimeStart),
    Started(RuntimeStarted),
    Stop(RuntimeStop),
    Stopped(RuntimeStopped),
    Shutdown(RuntimeShutdown),
    ShutdownComplete(RuntimeShutdownComplete),
    Error(RuntimeError),
    ListTargets(RuntimeListTargets),
    ListTargetsResult(RuntimeListTargetsResult),
    SelectTarget(RuntimeSelectTarget),
    SelectTargetResult(RuntimeSelectTargetResult),
    Observe(RuntimeObserve),
    ObserveResult(RuntimeObserveResult),
    Explain(RuntimeExplain),
    ExplainResult(RuntimeExplainResult),
    Check(RuntimeCheck),
    CheckResult(RuntimeCheckResult),
    PresentationAck(RuntimePresentationAck),
    PresentationAckResult(RuntimePresentationAckResult),
    Configure(RuntimeConfigure),
    Configured(RuntimeConfigured),
    Ask(RuntimeAsk),
    AskResult(RuntimeAskResult),
    RefreshCue(RuntimeRefreshCue),
    RefreshCueResult(RuntimeRefreshCueResult),
    PlanControl(RuntimePlanControl),
    PlanControlResult(RuntimePlanControlResult),
}
impl ::std::convert::From<&Self> for RuntimeMessage {
    fn from(value: &RuntimeMessage) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<RuntimeInitialize> for RuntimeMessage {
    fn from(value: RuntimeInitialize) -> Self {
        Self::Initialize(value)
    }
}
impl ::std::convert::From<RuntimeReady> for RuntimeMessage {
    fn from(value: RuntimeReady) -> Self {
        Self::Ready(value)
    }
}
impl ::std::convert::From<RuntimeHealth> for RuntimeMessage {
    fn from(value: RuntimeHealth) -> Self {
        Self::Health(value)
    }
}
impl ::std::convert::From<RuntimeHealthResult> for RuntimeMessage {
    fn from(value: RuntimeHealthResult) -> Self {
        Self::HealthResult(value)
    }
}
impl ::std::convert::From<RuntimeStart> for RuntimeMessage {
    fn from(value: RuntimeStart) -> Self {
        Self::Start(value)
    }
}
impl ::std::convert::From<RuntimeStarted> for RuntimeMessage {
    fn from(value: RuntimeStarted) -> Self {
        Self::Started(value)
    }
}
impl ::std::convert::From<RuntimeStop> for RuntimeMessage {
    fn from(value: RuntimeStop) -> Self {
        Self::Stop(value)
    }
}
impl ::std::convert::From<RuntimeStopped> for RuntimeMessage {
    fn from(value: RuntimeStopped) -> Self {
        Self::Stopped(value)
    }
}
impl ::std::convert::From<RuntimeShutdown> for RuntimeMessage {
    fn from(value: RuntimeShutdown) -> Self {
        Self::Shutdown(value)
    }
}
impl ::std::convert::From<RuntimeShutdownComplete> for RuntimeMessage {
    fn from(value: RuntimeShutdownComplete) -> Self {
        Self::ShutdownComplete(value)
    }
}
impl ::std::convert::From<RuntimeError> for RuntimeMessage {
    fn from(value: RuntimeError) -> Self {
        Self::Error(value)
    }
}
impl ::std::convert::From<RuntimeListTargets> for RuntimeMessage {
    fn from(value: RuntimeListTargets) -> Self {
        Self::ListTargets(value)
    }
}
impl ::std::convert::From<RuntimeListTargetsResult> for RuntimeMessage {
    fn from(value: RuntimeListTargetsResult) -> Self {
        Self::ListTargetsResult(value)
    }
}
impl ::std::convert::From<RuntimeSelectTarget> for RuntimeMessage {
    fn from(value: RuntimeSelectTarget) -> Self {
        Self::SelectTarget(value)
    }
}
impl ::std::convert::From<RuntimeSelectTargetResult> for RuntimeMessage {
    fn from(value: RuntimeSelectTargetResult) -> Self {
        Self::SelectTargetResult(value)
    }
}
impl ::std::convert::From<RuntimeObserve> for RuntimeMessage {
    fn from(value: RuntimeObserve) -> Self {
        Self::Observe(value)
    }
}
impl ::std::convert::From<RuntimeObserveResult> for RuntimeMessage {
    fn from(value: RuntimeObserveResult) -> Self {
        Self::ObserveResult(value)
    }
}
impl ::std::convert::From<RuntimeExplain> for RuntimeMessage {
    fn from(value: RuntimeExplain) -> Self {
        Self::Explain(value)
    }
}
impl ::std::convert::From<RuntimeExplainResult> for RuntimeMessage {
    fn from(value: RuntimeExplainResult) -> Self {
        Self::ExplainResult(value)
    }
}
impl ::std::convert::From<RuntimeCheck> for RuntimeMessage {
    fn from(value: RuntimeCheck) -> Self {
        Self::Check(value)
    }
}
impl ::std::convert::From<RuntimeCheckResult> for RuntimeMessage {
    fn from(value: RuntimeCheckResult) -> Self {
        Self::CheckResult(value)
    }
}
impl ::std::convert::From<RuntimePresentationAck> for RuntimeMessage {
    fn from(value: RuntimePresentationAck) -> Self {
        Self::PresentationAck(value)
    }
}
impl ::std::convert::From<RuntimePresentationAckResult> for RuntimeMessage {
    fn from(value: RuntimePresentationAckResult) -> Self {
        Self::PresentationAckResult(value)
    }
}
impl ::std::convert::From<RuntimeConfigure> for RuntimeMessage {
    fn from(value: RuntimeConfigure) -> Self {
        Self::Configure(value)
    }
}
impl ::std::convert::From<RuntimeConfigured> for RuntimeMessage {
    fn from(value: RuntimeConfigured) -> Self {
        Self::Configured(value)
    }
}
impl ::std::convert::From<RuntimeAsk> for RuntimeMessage {
    fn from(value: RuntimeAsk) -> Self {
        Self::Ask(value)
    }
}
impl ::std::convert::From<RuntimeAskResult> for RuntimeMessage {
    fn from(value: RuntimeAskResult) -> Self {
        Self::AskResult(value)
    }
}
impl ::std::convert::From<RuntimeRefreshCue> for RuntimeMessage {
    fn from(value: RuntimeRefreshCue) -> Self {
        Self::RefreshCue(value)
    }
}
impl ::std::convert::From<RuntimeRefreshCueResult> for RuntimeMessage {
    fn from(value: RuntimeRefreshCueResult) -> Self {
        Self::RefreshCueResult(value)
    }
}
impl ::std::convert::From<RuntimePlanControl> for RuntimeMessage {
    fn from(value: RuntimePlanControl) -> Self {
        Self::PlanControl(value)
    }
}
impl ::std::convert::From<RuntimePlanControlResult> for RuntimeMessage {
    fn from(value: RuntimePlanControlResult) -> Self {
        Self::PlanControlResult(value)
    }
}
///`RuntimeObserve`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeObserve",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.observe"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeObserve {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeObserveCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeObserveGenerationId,
    pub kind: RuntimeObserveKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeObserveProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeObserveRequestId,
}
impl ::std::convert::From<&RuntimeObserve> for RuntimeObserve {
    fn from(value: &RuntimeObserve) -> Self {
        value.clone()
    }
}
///`RuntimeObserveCorrelationId`
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
pub struct RuntimeObserveCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeObserveCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveCorrelationId> for ::std::string::String {
    fn from(value: RuntimeObserveCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveCorrelationId> for RuntimeObserveCorrelationId {
    fn from(value: &RuntimeObserveCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeObserveCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeObserveCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeObserveCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeObserveCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveCorrelationId {
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
///`RuntimeObserveGenerationId`
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
pub struct RuntimeObserveGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeObserveGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveGenerationId> for ::std::string::String {
    fn from(value: RuntimeObserveGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveGenerationId> for RuntimeObserveGenerationId {
    fn from(value: &RuntimeObserveGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeObserveGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeObserveGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeObserveGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeObserveGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveGenerationId {
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
///`RuntimeObserveKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.observe"
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
pub enum RuntimeObserveKind {
    #[serde(rename = "runtime.observe")]
    RuntimeObserve,
}
impl ::std::convert::From<&Self> for RuntimeObserveKind {
    fn from(value: &RuntimeObserveKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeObserveKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeObserve => f.write_str("runtime.observe"),
        }
    }
}
impl ::std::str::FromStr for RuntimeObserveKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.observe" => Ok(Self::RuntimeObserve),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeObserveKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeObserveKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeObserveKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeObserveProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeObserveProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeObserveProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveProtocolVersion> for i64 {
    fn from(value: RuntimeObserveProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveProtocolVersion>
for RuntimeObserveProtocolVersion {
    fn from(value: &RuntimeObserveProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeObserveProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeObserveRequestId`
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
pub struct RuntimeObserveRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeObserveRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveRequestId> for ::std::string::String {
    fn from(value: RuntimeObserveRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveRequestId> for RuntimeObserveRequestId {
    fn from(value: &RuntimeObserveRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeObserveRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeObserveRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeObserveRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeObserveRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveRequestId {
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
///`RuntimeObserveResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeObserveResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.observationResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeObserveResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeObserveResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeObserveResultGenerationId,
    pub kind: RuntimeObserveResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeObserveResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeObserveResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeObserveResult> for RuntimeObserveResult {
    fn from(value: &RuntimeObserveResult) -> Self {
        value.clone()
    }
}
///`RuntimeObserveResultCorrelationId`
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
pub struct RuntimeObserveResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeObserveResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveResultCorrelationId> for ::std::string::String {
    fn from(value: RuntimeObserveResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveResultCorrelationId>
for RuntimeObserveResultCorrelationId {
    fn from(value: &RuntimeObserveResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeObserveResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeObserveResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeObserveResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeObserveResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveResultCorrelationId {
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
///`RuntimeObserveResultGenerationId`
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
pub struct RuntimeObserveResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeObserveResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveResultGenerationId> for ::std::string::String {
    fn from(value: RuntimeObserveResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveResultGenerationId>
for RuntimeObserveResultGenerationId {
    fn from(value: &RuntimeObserveResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeObserveResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeObserveResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeObserveResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeObserveResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveResultGenerationId {
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
///`RuntimeObserveResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.observationResult"
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
pub enum RuntimeObserveResultKind {
    #[serde(rename = "runtime.observationResult")]
    RuntimeObservationResult,
}
impl ::std::convert::From<&Self> for RuntimeObserveResultKind {
    fn from(value: &RuntimeObserveResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeObserveResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeObservationResult => f.write_str("runtime.observationResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeObserveResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.observationResult" => Ok(Self::RuntimeObservationResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeObserveResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeObserveResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeObserveResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeObserveResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeObserveResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeObserveResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveResultProtocolVersion> for i64 {
    fn from(value: RuntimeObserveResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveResultProtocolVersion>
for RuntimeObserveResultProtocolVersion {
    fn from(value: &RuntimeObserveResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeObserveResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeObserveResultRequestId`
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
pub struct RuntimeObserveResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeObserveResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeObserveResultRequestId> for ::std::string::String {
    fn from(value: RuntimeObserveResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeObserveResultRequestId>
for RuntimeObserveResultRequestId {
    fn from(value: &RuntimeObserveResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeObserveResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeObserveResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeObserveResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeObserveResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeObserveResultRequestId {
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
///`RuntimePlanControl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimePlanControl",
///  "type": "object",
///  "required": [
///    "action",
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "action": {
///      "type": "string",
///      "enum": [
///        "pause",
///        "resume",
///        "confirm"
///      ]
///    },
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.planControl"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimePlanControl {
    pub action: RuntimePlanControlAction,
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimePlanControlCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimePlanControlGenerationId,
    pub kind: RuntimePlanControlKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimePlanControlProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimePlanControlRequestId,
}
impl ::std::convert::From<&RuntimePlanControl> for RuntimePlanControl {
    fn from(value: &RuntimePlanControl) -> Self {
        value.clone()
    }
}
///`RuntimePlanControlAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "pause",
///    "resume",
///    "confirm"
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
pub enum RuntimePlanControlAction {
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "confirm")]
    Confirm,
}
impl ::std::convert::From<&Self> for RuntimePlanControlAction {
    fn from(value: &RuntimePlanControlAction) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimePlanControlAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pause => f.write_str("pause"),
            Self::Resume => f.write_str("resume"),
            Self::Confirm => f.write_str("confirm"),
        }
    }
}
impl ::std::str::FromStr for RuntimePlanControlAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pause" => Ok(Self::Pause),
            "resume" => Ok(Self::Resume),
            "confirm" => Ok(Self::Confirm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimePlanControlAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePlanControlAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePlanControlAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimePlanControlCorrelationId`
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
pub struct RuntimePlanControlCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimePlanControlCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlCorrelationId> for ::std::string::String {
    fn from(value: RuntimePlanControlCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlCorrelationId>
for RuntimePlanControlCorrelationId {
    fn from(value: &RuntimePlanControlCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePlanControlCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePlanControlCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePlanControlCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePlanControlCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlCorrelationId {
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
///`RuntimePlanControlGenerationId`
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
pub struct RuntimePlanControlGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimePlanControlGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlGenerationId> for ::std::string::String {
    fn from(value: RuntimePlanControlGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlGenerationId>
for RuntimePlanControlGenerationId {
    fn from(value: &RuntimePlanControlGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePlanControlGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePlanControlGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePlanControlGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePlanControlGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlGenerationId {
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
///`RuntimePlanControlKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.planControl"
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
pub enum RuntimePlanControlKind {
    #[serde(rename = "runtime.planControl")]
    RuntimePlanControl,
}
impl ::std::convert::From<&Self> for RuntimePlanControlKind {
    fn from(value: &RuntimePlanControlKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimePlanControlKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimePlanControl => f.write_str("runtime.planControl"),
        }
    }
}
impl ::std::str::FromStr for RuntimePlanControlKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.planControl" => Ok(Self::RuntimePlanControl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimePlanControlKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePlanControlKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePlanControlKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimePlanControlProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimePlanControlProtocolVersion(i64);
impl ::std::ops::Deref for RuntimePlanControlProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlProtocolVersion> for i64 {
    fn from(value: RuntimePlanControlProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlProtocolVersion>
for RuntimePlanControlProtocolVersion {
    fn from(value: &RuntimePlanControlProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimePlanControlProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimePlanControlRequestId`
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
pub struct RuntimePlanControlRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimePlanControlRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlRequestId> for ::std::string::String {
    fn from(value: RuntimePlanControlRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlRequestId> for RuntimePlanControlRequestId {
    fn from(value: &RuntimePlanControlRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePlanControlRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimePlanControlRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePlanControlRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePlanControlRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlRequestId {
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
///`RuntimePlanControlResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimePlanControlResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.planControlResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimePlanControlResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimePlanControlResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimePlanControlResultGenerationId,
    pub kind: RuntimePlanControlResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimePlanControlResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimePlanControlResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimePlanControlResult> for RuntimePlanControlResult {
    fn from(value: &RuntimePlanControlResult) -> Self {
        value.clone()
    }
}
///`RuntimePlanControlResultCorrelationId`
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
pub struct RuntimePlanControlResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimePlanControlResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlResultCorrelationId>
for ::std::string::String {
    fn from(value: RuntimePlanControlResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlResultCorrelationId>
for RuntimePlanControlResultCorrelationId {
    fn from(value: &RuntimePlanControlResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePlanControlResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePlanControlResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePlanControlResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePlanControlResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlResultCorrelationId {
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
///`RuntimePlanControlResultGenerationId`
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
pub struct RuntimePlanControlResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimePlanControlResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlResultGenerationId>
for ::std::string::String {
    fn from(value: RuntimePlanControlResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlResultGenerationId>
for RuntimePlanControlResultGenerationId {
    fn from(value: &RuntimePlanControlResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePlanControlResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePlanControlResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePlanControlResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePlanControlResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlResultGenerationId {
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
///`RuntimePlanControlResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.planControlResult"
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
pub enum RuntimePlanControlResultKind {
    #[serde(rename = "runtime.planControlResult")]
    RuntimePlanControlResult,
}
impl ::std::convert::From<&Self> for RuntimePlanControlResultKind {
    fn from(value: &RuntimePlanControlResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimePlanControlResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimePlanControlResult => f.write_str("runtime.planControlResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimePlanControlResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.planControlResult" => Ok(Self::RuntimePlanControlResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimePlanControlResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePlanControlResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePlanControlResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimePlanControlResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimePlanControlResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimePlanControlResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlResultProtocolVersion> for i64 {
    fn from(value: RuntimePlanControlResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlResultProtocolVersion>
for RuntimePlanControlResultProtocolVersion {
    fn from(value: &RuntimePlanControlResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimePlanControlResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimePlanControlResultRequestId`
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
pub struct RuntimePlanControlResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimePlanControlResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePlanControlResultRequestId> for ::std::string::String {
    fn from(value: RuntimePlanControlResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePlanControlResultRequestId>
for RuntimePlanControlResultRequestId {
    fn from(value: &RuntimePlanControlResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePlanControlResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimePlanControlResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePlanControlResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePlanControlResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePlanControlResultRequestId {
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
///`RuntimePresentationAck`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimePresentationAck",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "cueId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "cueId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.presentationAck"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimePresentationAck {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimePresentationAckCorrelationId,
    #[serde(rename = "cueId")]
    pub cue_id: RuntimePresentationAckCueId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimePresentationAckGenerationId,
    pub kind: RuntimePresentationAckKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimePresentationAckProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimePresentationAckRequestId,
}
impl ::std::convert::From<&RuntimePresentationAck> for RuntimePresentationAck {
    fn from(value: &RuntimePresentationAck) -> Self {
        value.clone()
    }
}
///`RuntimePresentationAckCorrelationId`
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
pub struct RuntimePresentationAckCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckCorrelationId>
for ::std::string::String {
    fn from(value: RuntimePresentationAckCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckCorrelationId>
for RuntimePresentationAckCorrelationId {
    fn from(value: &RuntimePresentationAckCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePresentationAckCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckCorrelationId {
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
///`RuntimePresentationAckCueId`
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
pub struct RuntimePresentationAckCueId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckCueId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckCueId> for ::std::string::String {
    fn from(value: RuntimePresentationAckCueId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckCueId> for RuntimePresentationAckCueId {
    fn from(value: &RuntimePresentationAckCueId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckCueId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePresentationAckCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePresentationAckCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckCueId {
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
///`RuntimePresentationAckGenerationId`
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
pub struct RuntimePresentationAckGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckGenerationId> for ::std::string::String {
    fn from(value: RuntimePresentationAckGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckGenerationId>
for RuntimePresentationAckGenerationId {
    fn from(value: &RuntimePresentationAckGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePresentationAckGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckGenerationId {
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
///`RuntimePresentationAckKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.presentationAck"
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
pub enum RuntimePresentationAckKind {
    #[serde(rename = "runtime.presentationAck")]
    RuntimePresentationAck,
}
impl ::std::convert::From<&Self> for RuntimePresentationAckKind {
    fn from(value: &RuntimePresentationAckKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimePresentationAckKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimePresentationAck => f.write_str("runtime.presentationAck"),
        }
    }
}
impl ::std::str::FromStr for RuntimePresentationAckKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.presentationAck" => Ok(Self::RuntimePresentationAck),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimePresentationAckKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePresentationAckKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimePresentationAckProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimePresentationAckProtocolVersion(i64);
impl ::std::ops::Deref for RuntimePresentationAckProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckProtocolVersion> for i64 {
    fn from(value: RuntimePresentationAckProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckProtocolVersion>
for RuntimePresentationAckProtocolVersion {
    fn from(value: &RuntimePresentationAckProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimePresentationAckProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimePresentationAckRequestId`
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
pub struct RuntimePresentationAckRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckRequestId> for ::std::string::String {
    fn from(value: RuntimePresentationAckRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckRequestId>
for RuntimePresentationAckRequestId {
    fn from(value: &RuntimePresentationAckRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimePresentationAckRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckRequestId {
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
///`RuntimePresentationAckResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimePresentationAckResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.presentationAckResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimePresentationAckResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimePresentationAckResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimePresentationAckResultGenerationId,
    pub kind: RuntimePresentationAckResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimePresentationAckResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimePresentationAckResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimePresentationAckResult>
for RuntimePresentationAckResult {
    fn from(value: &RuntimePresentationAckResult) -> Self {
        value.clone()
    }
}
///`RuntimePresentationAckResultCorrelationId`
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
pub struct RuntimePresentationAckResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckResultCorrelationId>
for ::std::string::String {
    fn from(value: RuntimePresentationAckResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckResultCorrelationId>
for RuntimePresentationAckResultCorrelationId {
    fn from(value: &RuntimePresentationAckResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePresentationAckResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckResultCorrelationId {
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
///`RuntimePresentationAckResultGenerationId`
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
pub struct RuntimePresentationAckResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckResultGenerationId>
for ::std::string::String {
    fn from(value: RuntimePresentationAckResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckResultGenerationId>
for RuntimePresentationAckResultGenerationId {
    fn from(value: &RuntimePresentationAckResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePresentationAckResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckResultGenerationId {
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
///`RuntimePresentationAckResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.presentationAckResult"
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
pub enum RuntimePresentationAckResultKind {
    #[serde(rename = "runtime.presentationAckResult")]
    RuntimePresentationAckResult,
}
impl ::std::convert::From<&Self> for RuntimePresentationAckResultKind {
    fn from(value: &RuntimePresentationAckResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimePresentationAckResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimePresentationAckResult => {
                f.write_str("runtime.presentationAckResult")
            }
        }
    }
}
impl ::std::str::FromStr for RuntimePresentationAckResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.presentationAckResult" => Ok(Self::RuntimePresentationAckResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePresentationAckResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimePresentationAckResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimePresentationAckResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimePresentationAckResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckResultProtocolVersion> for i64 {
    fn from(value: RuntimePresentationAckResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckResultProtocolVersion>
for RuntimePresentationAckResultProtocolVersion {
    fn from(value: &RuntimePresentationAckResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimePresentationAckResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimePresentationAckResultRequestId`
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
pub struct RuntimePresentationAckResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimePresentationAckResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimePresentationAckResultRequestId>
for ::std::string::String {
    fn from(value: RuntimePresentationAckResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimePresentationAckResultRequestId>
for RuntimePresentationAckResultRequestId {
    fn from(value: &RuntimePresentationAckResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimePresentationAckResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimePresentationAckResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimePresentationAckResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimePresentationAckResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimePresentationAckResultRequestId {
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
///`RuntimeReady`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeReady",
///  "type": "object",
///  "required": [
///    "capabilities",
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "schemaDigest"
///  ],
///  "properties": {
///    "capabilities": {
///      "type": "array",
///      "items": {
///        "type": "string",
///        "enum": [
///          "diagnostic"
///        ]
///      },
///      "maxItems": 1,
///      "uniqueItems": true
///    },
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.ready"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "schemaDigest": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{64}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeReady {
    pub capabilities: Vec<RuntimeReadyCapabilitiesItem>,
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeReadyCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeReadyGenerationId,
    pub kind: RuntimeReadyKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeReadyProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeReadyRequestId,
    #[serde(rename = "schemaDigest")]
    pub schema_digest: RuntimeReadySchemaDigest,
}
impl ::std::convert::From<&RuntimeReady> for RuntimeReady {
    fn from(value: &RuntimeReady) -> Self {
        value.clone()
    }
}
///`RuntimeReadyCapabilitiesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "diagnostic"
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
pub enum RuntimeReadyCapabilitiesItem {
    #[serde(rename = "diagnostic")]
    Diagnostic,
}
impl ::std::convert::From<&Self> for RuntimeReadyCapabilitiesItem {
    fn from(value: &RuntimeReadyCapabilitiesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeReadyCapabilitiesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Diagnostic => f.write_str("diagnostic"),
        }
    }
}
impl ::std::str::FromStr for RuntimeReadyCapabilitiesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "diagnostic" => Ok(Self::Diagnostic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeReadyCapabilitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeReadyCapabilitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeReadyCapabilitiesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeReadyCorrelationId`
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
pub struct RuntimeReadyCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeReadyCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeReadyCorrelationId> for ::std::string::String {
    fn from(value: RuntimeReadyCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeReadyCorrelationId> for RuntimeReadyCorrelationId {
    fn from(value: &RuntimeReadyCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeReadyCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeReadyCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeReadyCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeReadyCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeReadyCorrelationId {
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
///`RuntimeReadyGenerationId`
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
pub struct RuntimeReadyGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeReadyGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeReadyGenerationId> for ::std::string::String {
    fn from(value: RuntimeReadyGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeReadyGenerationId> for RuntimeReadyGenerationId {
    fn from(value: &RuntimeReadyGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeReadyGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeReadyGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeReadyGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeReadyGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeReadyGenerationId {
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
///`RuntimeReadyKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.ready"
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
pub enum RuntimeReadyKind {
    #[serde(rename = "runtime.ready")]
    RuntimeReady,
}
impl ::std::convert::From<&Self> for RuntimeReadyKind {
    fn from(value: &RuntimeReadyKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeReadyKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeReady => f.write_str("runtime.ready"),
        }
    }
}
impl ::std::str::FromStr for RuntimeReadyKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.ready" => Ok(Self::RuntimeReady),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeReadyKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeReadyKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeReadyKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeReadyProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeReadyProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeReadyProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeReadyProtocolVersion> for i64 {
    fn from(value: RuntimeReadyProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeReadyProtocolVersion> for RuntimeReadyProtocolVersion {
    fn from(value: &RuntimeReadyProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeReadyProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeReadyProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeReadyRequestId`
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
pub struct RuntimeReadyRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeReadyRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeReadyRequestId> for ::std::string::String {
    fn from(value: RuntimeReadyRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeReadyRequestId> for RuntimeReadyRequestId {
    fn from(value: &RuntimeReadyRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeReadyRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeReadyRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeReadyRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeReadyRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeReadyRequestId {
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
///`RuntimeReadySchemaDigest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{64}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuntimeReadySchemaDigest(::std::string::String);
impl ::std::ops::Deref for RuntimeReadySchemaDigest {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeReadySchemaDigest> for ::std::string::String {
    fn from(value: RuntimeReadySchemaDigest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeReadySchemaDigest> for RuntimeReadySchemaDigest {
    fn from(value: &RuntimeReadySchemaDigest) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeReadySchemaDigest {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9a-f]{64}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{64}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeReadySchemaDigest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeReadySchemaDigest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeReadySchemaDigest {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeReadySchemaDigest {
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
///`RuntimeRefreshCue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeRefreshCue",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.refreshCue"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeRefreshCue {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeRefreshCueCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeRefreshCueGenerationId,
    pub kind: RuntimeRefreshCueKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeRefreshCueProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeRefreshCueRequestId,
}
impl ::std::convert::From<&RuntimeRefreshCue> for RuntimeRefreshCue {
    fn from(value: &RuntimeRefreshCue) -> Self {
        value.clone()
    }
}
///`RuntimeRefreshCueCorrelationId`
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
pub struct RuntimeRefreshCueCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeRefreshCueCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueCorrelationId> for ::std::string::String {
    fn from(value: RuntimeRefreshCueCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueCorrelationId>
for RuntimeRefreshCueCorrelationId {
    fn from(value: &RuntimeRefreshCueCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeRefreshCueCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeRefreshCueCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueCorrelationId {
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
///`RuntimeRefreshCueGenerationId`
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
pub struct RuntimeRefreshCueGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeRefreshCueGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueGenerationId> for ::std::string::String {
    fn from(value: RuntimeRefreshCueGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueGenerationId>
for RuntimeRefreshCueGenerationId {
    fn from(value: &RuntimeRefreshCueGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeRefreshCueGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeRefreshCueGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueGenerationId {
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
///`RuntimeRefreshCueKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.refreshCue"
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
pub enum RuntimeRefreshCueKind {
    #[serde(rename = "runtime.refreshCue")]
    RuntimeRefreshCue,
}
impl ::std::convert::From<&Self> for RuntimeRefreshCueKind {
    fn from(value: &RuntimeRefreshCueKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeRefreshCueKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeRefreshCue => f.write_str("runtime.refreshCue"),
        }
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.refreshCue" => Ok(Self::RuntimeRefreshCue),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeRefreshCueKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeRefreshCueKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeRefreshCueProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeRefreshCueProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeRefreshCueProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueProtocolVersion> for i64 {
    fn from(value: RuntimeRefreshCueProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueProtocolVersion>
for RuntimeRefreshCueProtocolVersion {
    fn from(value: &RuntimeRefreshCueProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeRefreshCueProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeRefreshCueRequestId`
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
pub struct RuntimeRefreshCueRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeRefreshCueRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueRequestId> for ::std::string::String {
    fn from(value: RuntimeRefreshCueRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueRequestId> for RuntimeRefreshCueRequestId {
    fn from(value: &RuntimeRefreshCueRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeRefreshCueRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeRefreshCueRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueRequestId {
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
///`RuntimeRefreshCueResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeRefreshCueResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.cueRefreshResult"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeRefreshCueResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeRefreshCueResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeRefreshCueResultGenerationId,
    pub kind: RuntimeRefreshCueResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeRefreshCueResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeRefreshCueResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeRefreshCueResult> for RuntimeRefreshCueResult {
    fn from(value: &RuntimeRefreshCueResult) -> Self {
        value.clone()
    }
}
///`RuntimeRefreshCueResultCorrelationId`
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
pub struct RuntimeRefreshCueResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeRefreshCueResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueResultCorrelationId>
for ::std::string::String {
    fn from(value: RuntimeRefreshCueResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueResultCorrelationId>
for RuntimeRefreshCueResultCorrelationId {
    fn from(value: &RuntimeRefreshCueResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeRefreshCueResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeRefreshCueResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueResultCorrelationId {
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
///`RuntimeRefreshCueResultGenerationId`
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
pub struct RuntimeRefreshCueResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeRefreshCueResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueResultGenerationId>
for ::std::string::String {
    fn from(value: RuntimeRefreshCueResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueResultGenerationId>
for RuntimeRefreshCueResultGenerationId {
    fn from(value: &RuntimeRefreshCueResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeRefreshCueResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeRefreshCueResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueResultGenerationId {
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
///`RuntimeRefreshCueResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.cueRefreshResult"
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
pub enum RuntimeRefreshCueResultKind {
    #[serde(rename = "runtime.cueRefreshResult")]
    RuntimeCueRefreshResult,
}
impl ::std::convert::From<&Self> for RuntimeRefreshCueResultKind {
    fn from(value: &RuntimeRefreshCueResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeRefreshCueResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeCueRefreshResult => f.write_str("runtime.cueRefreshResult"),
        }
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.cueRefreshResult" => Ok(Self::RuntimeCueRefreshResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeRefreshCueResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeRefreshCueResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeRefreshCueResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeRefreshCueResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeRefreshCueResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueResultProtocolVersion> for i64 {
    fn from(value: RuntimeRefreshCueResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueResultProtocolVersion>
for RuntimeRefreshCueResultProtocolVersion {
    fn from(value: &RuntimeRefreshCueResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeRefreshCueResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeRefreshCueResultRequestId`
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
pub struct RuntimeRefreshCueResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeRefreshCueResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeRefreshCueResultRequestId> for ::std::string::String {
    fn from(value: RuntimeRefreshCueResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeRefreshCueResultRequestId>
for RuntimeRefreshCueResultRequestId {
    fn from(value: &RuntimeRefreshCueResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeRefreshCueResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeRefreshCueResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeRefreshCueResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeRefreshCueResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeRefreshCueResultRequestId {
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
///`RuntimeSelectTarget`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeSelectTarget",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "pid",
///    "protocolVersion",
///    "requestId",
///    "windowId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.selectTarget"
///      ]
///    },
///    "pid": {
///      "type": "integer",
///      "minimum": 1.0
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "windowId": {
///      "type": "integer",
///      "minimum": 0.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeSelectTarget {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeSelectTargetCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeSelectTargetGenerationId,
    pub kind: RuntimeSelectTargetKind,
    pub pid: ::std::num::NonZeroU64,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeSelectTargetProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeSelectTargetRequestId,
    #[serde(rename = "windowId")]
    pub window_id: u64,
}
impl ::std::convert::From<&RuntimeSelectTarget> for RuntimeSelectTarget {
    fn from(value: &RuntimeSelectTarget) -> Self {
        value.clone()
    }
}
///`RuntimeSelectTargetCorrelationId`
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
pub struct RuntimeSelectTargetCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeSelectTargetCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetCorrelationId> for ::std::string::String {
    fn from(value: RuntimeSelectTargetCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetCorrelationId>
for RuntimeSelectTargetCorrelationId {
    fn from(value: &RuntimeSelectTargetCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeSelectTargetCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeSelectTargetCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetCorrelationId {
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
///`RuntimeSelectTargetGenerationId`
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
pub struct RuntimeSelectTargetGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeSelectTargetGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetGenerationId> for ::std::string::String {
    fn from(value: RuntimeSelectTargetGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetGenerationId>
for RuntimeSelectTargetGenerationId {
    fn from(value: &RuntimeSelectTargetGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeSelectTargetGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeSelectTargetGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetGenerationId {
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
///`RuntimeSelectTargetKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.selectTarget"
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
pub enum RuntimeSelectTargetKind {
    #[serde(rename = "runtime.selectTarget")]
    RuntimeSelectTarget,
}
impl ::std::convert::From<&Self> for RuntimeSelectTargetKind {
    fn from(value: &RuntimeSelectTargetKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeSelectTargetKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeSelectTarget => f.write_str("runtime.selectTarget"),
        }
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.selectTarget" => Ok(Self::RuntimeSelectTarget),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeSelectTargetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeSelectTargetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeSelectTargetProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeSelectTargetProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeSelectTargetProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetProtocolVersion> for i64 {
    fn from(value: RuntimeSelectTargetProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetProtocolVersion>
for RuntimeSelectTargetProtocolVersion {
    fn from(value: &RuntimeSelectTargetProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeSelectTargetProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeSelectTargetRequestId`
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
pub struct RuntimeSelectTargetRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeSelectTargetRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetRequestId> for ::std::string::String {
    fn from(value: RuntimeSelectTargetRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetRequestId>
for RuntimeSelectTargetRequestId {
    fn from(value: &RuntimeSelectTargetRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeSelectTargetRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeSelectTargetRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetRequestId {
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
///`RuntimeSelectTargetResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeSelectTargetResult",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "state"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.targetSelected"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "state": {
///      "$ref": "#/definitions/TeachingState"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeSelectTargetResult {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeSelectTargetResultCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeSelectTargetResultGenerationId,
    pub kind: RuntimeSelectTargetResultKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeSelectTargetResultProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeSelectTargetResultRequestId,
    pub state: TeachingState,
}
impl ::std::convert::From<&RuntimeSelectTargetResult> for RuntimeSelectTargetResult {
    fn from(value: &RuntimeSelectTargetResult) -> Self {
        value.clone()
    }
}
///`RuntimeSelectTargetResultCorrelationId`
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
pub struct RuntimeSelectTargetResultCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeSelectTargetResultCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetResultCorrelationId>
for ::std::string::String {
    fn from(value: RuntimeSelectTargetResultCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetResultCorrelationId>
for RuntimeSelectTargetResultCorrelationId {
    fn from(value: &RuntimeSelectTargetResultCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetResultCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeSelectTargetResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeSelectTargetResultCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetResultCorrelationId {
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
///`RuntimeSelectTargetResultGenerationId`
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
pub struct RuntimeSelectTargetResultGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeSelectTargetResultGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetResultGenerationId>
for ::std::string::String {
    fn from(value: RuntimeSelectTargetResultGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetResultGenerationId>
for RuntimeSelectTargetResultGenerationId {
    fn from(value: &RuntimeSelectTargetResultGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetResultGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeSelectTargetResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeSelectTargetResultGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetResultGenerationId {
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
///`RuntimeSelectTargetResultKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.targetSelected"
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
pub enum RuntimeSelectTargetResultKind {
    #[serde(rename = "runtime.targetSelected")]
    RuntimeTargetSelected,
}
impl ::std::convert::From<&Self> for RuntimeSelectTargetResultKind {
    fn from(value: &RuntimeSelectTargetResultKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeSelectTargetResultKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeTargetSelected => f.write_str("runtime.targetSelected"),
        }
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetResultKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.targetSelected" => Ok(Self::RuntimeTargetSelected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeSelectTargetResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeSelectTargetResultKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeSelectTargetResultProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeSelectTargetResultProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeSelectTargetResultProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetResultProtocolVersion> for i64 {
    fn from(value: RuntimeSelectTargetResultProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetResultProtocolVersion>
for RuntimeSelectTargetResultProtocolVersion {
    fn from(value: &RuntimeSelectTargetResultProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeSelectTargetResultProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetResultProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeSelectTargetResultRequestId`
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
pub struct RuntimeSelectTargetResultRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeSelectTargetResultRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeSelectTargetResultRequestId> for ::std::string::String {
    fn from(value: RuntimeSelectTargetResultRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeSelectTargetResultRequestId>
for RuntimeSelectTargetResultRequestId {
    fn from(value: &RuntimeSelectTargetResultRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeSelectTargetResultRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeSelectTargetResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeSelectTargetResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeSelectTargetResultRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeSelectTargetResultRequestId {
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
///`RuntimeShutdown`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeShutdown",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.shutdown"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeShutdown {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeShutdownCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeShutdownGenerationId,
    pub kind: RuntimeShutdownKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeShutdownProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeShutdownRequestId,
}
impl ::std::convert::From<&RuntimeShutdown> for RuntimeShutdown {
    fn from(value: &RuntimeShutdown) -> Self {
        value.clone()
    }
}
///`RuntimeShutdownComplete`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeShutdownComplete",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.shutdownComplete"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeShutdownComplete {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeShutdownCompleteCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeShutdownCompleteGenerationId,
    pub kind: RuntimeShutdownCompleteKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeShutdownCompleteProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeShutdownCompleteRequestId,
}
impl ::std::convert::From<&RuntimeShutdownComplete> for RuntimeShutdownComplete {
    fn from(value: &RuntimeShutdownComplete) -> Self {
        value.clone()
    }
}
///`RuntimeShutdownCompleteCorrelationId`
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
pub struct RuntimeShutdownCompleteCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeShutdownCompleteCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownCompleteCorrelationId>
for ::std::string::String {
    fn from(value: RuntimeShutdownCompleteCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownCompleteCorrelationId>
for RuntimeShutdownCompleteCorrelationId {
    fn from(value: &RuntimeShutdownCompleteCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeShutdownCompleteCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeShutdownCompleteCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeShutdownCompleteCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeShutdownCompleteCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownCompleteCorrelationId {
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
///`RuntimeShutdownCompleteGenerationId`
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
pub struct RuntimeShutdownCompleteGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeShutdownCompleteGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownCompleteGenerationId>
for ::std::string::String {
    fn from(value: RuntimeShutdownCompleteGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownCompleteGenerationId>
for RuntimeShutdownCompleteGenerationId {
    fn from(value: &RuntimeShutdownCompleteGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeShutdownCompleteGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeShutdownCompleteGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeShutdownCompleteGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeShutdownCompleteGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownCompleteGenerationId {
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
///`RuntimeShutdownCompleteKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.shutdownComplete"
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
pub enum RuntimeShutdownCompleteKind {
    #[serde(rename = "runtime.shutdownComplete")]
    RuntimeShutdownComplete,
}
impl ::std::convert::From<&Self> for RuntimeShutdownCompleteKind {
    fn from(value: &RuntimeShutdownCompleteKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeShutdownCompleteKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeShutdownComplete => f.write_str("runtime.shutdownComplete"),
        }
    }
}
impl ::std::str::FromStr for RuntimeShutdownCompleteKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.shutdownComplete" => Ok(Self::RuntimeShutdownComplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeShutdownCompleteKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeShutdownCompleteKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeShutdownCompleteKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeShutdownCompleteProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeShutdownCompleteProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeShutdownCompleteProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownCompleteProtocolVersion> for i64 {
    fn from(value: RuntimeShutdownCompleteProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownCompleteProtocolVersion>
for RuntimeShutdownCompleteProtocolVersion {
    fn from(value: &RuntimeShutdownCompleteProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeShutdownCompleteProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownCompleteProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeShutdownCompleteRequestId`
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
pub struct RuntimeShutdownCompleteRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeShutdownCompleteRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownCompleteRequestId> for ::std::string::String {
    fn from(value: RuntimeShutdownCompleteRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownCompleteRequestId>
for RuntimeShutdownCompleteRequestId {
    fn from(value: &RuntimeShutdownCompleteRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeShutdownCompleteRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeShutdownCompleteRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for RuntimeShutdownCompleteRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RuntimeShutdownCompleteRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownCompleteRequestId {
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
///`RuntimeShutdownCorrelationId`
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
pub struct RuntimeShutdownCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeShutdownCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownCorrelationId> for ::std::string::String {
    fn from(value: RuntimeShutdownCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownCorrelationId>
for RuntimeShutdownCorrelationId {
    fn from(value: &RuntimeShutdownCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeShutdownCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeShutdownCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeShutdownCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeShutdownCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownCorrelationId {
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
///`RuntimeShutdownGenerationId`
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
pub struct RuntimeShutdownGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeShutdownGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownGenerationId> for ::std::string::String {
    fn from(value: RuntimeShutdownGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownGenerationId> for RuntimeShutdownGenerationId {
    fn from(value: &RuntimeShutdownGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeShutdownGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeShutdownGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeShutdownGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeShutdownGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownGenerationId {
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
///`RuntimeShutdownKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.shutdown"
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
pub enum RuntimeShutdownKind {
    #[serde(rename = "runtime.shutdown")]
    RuntimeShutdown,
}
impl ::std::convert::From<&Self> for RuntimeShutdownKind {
    fn from(value: &RuntimeShutdownKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeShutdownKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeShutdown => f.write_str("runtime.shutdown"),
        }
    }
}
impl ::std::str::FromStr for RuntimeShutdownKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.shutdown" => Ok(Self::RuntimeShutdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeShutdownKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeShutdownKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeShutdownKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeShutdownProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeShutdownProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeShutdownProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownProtocolVersion> for i64 {
    fn from(value: RuntimeShutdownProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownProtocolVersion>
for RuntimeShutdownProtocolVersion {
    fn from(value: &RuntimeShutdownProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeShutdownProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeShutdownRequestId`
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
pub struct RuntimeShutdownRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeShutdownRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeShutdownRequestId> for ::std::string::String {
    fn from(value: RuntimeShutdownRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeShutdownRequestId> for RuntimeShutdownRequestId {
    fn from(value: &RuntimeShutdownRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeShutdownRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeShutdownRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeShutdownRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeShutdownRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeShutdownRequestId {
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
///`RuntimeStart`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeStart",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "sessionId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.start"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "sessionId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeStart {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeStartCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeStartGenerationId,
    pub kind: RuntimeStartKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeStartProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeStartRequestId,
    #[serde(rename = "sessionId")]
    pub session_id: RuntimeStartSessionId,
}
impl ::std::convert::From<&RuntimeStart> for RuntimeStart {
    fn from(value: &RuntimeStart) -> Self {
        value.clone()
    }
}
///`RuntimeStartCorrelationId`
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
pub struct RuntimeStartCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartCorrelationId> for ::std::string::String {
    fn from(value: RuntimeStartCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartCorrelationId> for RuntimeStartCorrelationId {
    fn from(value: &RuntimeStartCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartCorrelationId {
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
///`RuntimeStartGenerationId`
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
pub struct RuntimeStartGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartGenerationId> for ::std::string::String {
    fn from(value: RuntimeStartGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartGenerationId> for RuntimeStartGenerationId {
    fn from(value: &RuntimeStartGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartGenerationId {
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
///`RuntimeStartKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.start"
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
pub enum RuntimeStartKind {
    #[serde(rename = "runtime.start")]
    RuntimeStart,
}
impl ::std::convert::From<&Self> for RuntimeStartKind {
    fn from(value: &RuntimeStartKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeStartKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeStart => f.write_str("runtime.start"),
        }
    }
}
impl ::std::str::FromStr for RuntimeStartKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.start" => Ok(Self::RuntimeStart),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeStartKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeStartProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeStartProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeStartProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartProtocolVersion> for i64 {
    fn from(value: RuntimeStartProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartProtocolVersion> for RuntimeStartProtocolVersion {
    fn from(value: &RuntimeStartProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeStartProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeStartRequestId`
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
pub struct RuntimeStartRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartRequestId> for ::std::string::String {
    fn from(value: RuntimeStartRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartRequestId> for RuntimeStartRequestId {
    fn from(value: &RuntimeStartRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartRequestId {
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
///`RuntimeStartSessionId`
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
pub struct RuntimeStartSessionId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartSessionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartSessionId> for ::std::string::String {
    fn from(value: RuntimeStartSessionId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartSessionId> for RuntimeStartSessionId {
    fn from(value: &RuntimeStartSessionId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartSessionId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartSessionId {
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
///`RuntimeStarted`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeStarted",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId",
///    "sessionId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.started"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "sessionId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeStarted {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeStartedCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeStartedGenerationId,
    pub kind: RuntimeStartedKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeStartedProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeStartedRequestId,
    #[serde(rename = "sessionId")]
    pub session_id: RuntimeStartedSessionId,
}
impl ::std::convert::From<&RuntimeStarted> for RuntimeStarted {
    fn from(value: &RuntimeStarted) -> Self {
        value.clone()
    }
}
///`RuntimeStartedCorrelationId`
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
pub struct RuntimeStartedCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartedCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartedCorrelationId> for ::std::string::String {
    fn from(value: RuntimeStartedCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartedCorrelationId> for RuntimeStartedCorrelationId {
    fn from(value: &RuntimeStartedCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartedCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartedCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartedCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartedCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartedCorrelationId {
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
///`RuntimeStartedGenerationId`
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
pub struct RuntimeStartedGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartedGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartedGenerationId> for ::std::string::String {
    fn from(value: RuntimeStartedGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartedGenerationId> for RuntimeStartedGenerationId {
    fn from(value: &RuntimeStartedGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartedGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartedGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartedGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartedGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartedGenerationId {
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
///`RuntimeStartedKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.started"
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
pub enum RuntimeStartedKind {
    #[serde(rename = "runtime.started")]
    RuntimeStarted,
}
impl ::std::convert::From<&Self> for RuntimeStartedKind {
    fn from(value: &RuntimeStartedKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeStartedKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeStarted => f.write_str("runtime.started"),
        }
    }
}
impl ::std::str::FromStr for RuntimeStartedKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.started" => Ok(Self::RuntimeStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeStartedKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartedKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartedKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeStartedProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeStartedProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeStartedProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartedProtocolVersion> for i64 {
    fn from(value: RuntimeStartedProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartedProtocolVersion>
for RuntimeStartedProtocolVersion {
    fn from(value: &RuntimeStartedProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeStartedProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartedProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeStartedRequestId`
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
pub struct RuntimeStartedRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartedRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartedRequestId> for ::std::string::String {
    fn from(value: RuntimeStartedRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartedRequestId> for RuntimeStartedRequestId {
    fn from(value: &RuntimeStartedRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartedRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartedRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartedRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartedRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartedRequestId {
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
///`RuntimeStartedSessionId`
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
pub struct RuntimeStartedSessionId(::std::string::String);
impl ::std::ops::Deref for RuntimeStartedSessionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStartedSessionId> for ::std::string::String {
    fn from(value: RuntimeStartedSessionId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStartedSessionId> for RuntimeStartedSessionId {
    fn from(value: &RuntimeStartedSessionId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStartedSessionId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStartedSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStartedSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStartedSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStartedSessionId {
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
///`RuntimeStop`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeStop",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.stop"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeStop {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeStopCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeStopGenerationId,
    pub kind: RuntimeStopKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeStopProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeStopRequestId,
}
impl ::std::convert::From<&RuntimeStop> for RuntimeStop {
    fn from(value: &RuntimeStop) -> Self {
        value.clone()
    }
}
///`RuntimeStopCorrelationId`
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
pub struct RuntimeStopCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStopCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStopCorrelationId> for ::std::string::String {
    fn from(value: RuntimeStopCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStopCorrelationId> for RuntimeStopCorrelationId {
    fn from(value: &RuntimeStopCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStopCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStopCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStopCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStopCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStopCorrelationId {
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
///`RuntimeStopGenerationId`
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
pub struct RuntimeStopGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStopGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStopGenerationId> for ::std::string::String {
    fn from(value: RuntimeStopGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStopGenerationId> for RuntimeStopGenerationId {
    fn from(value: &RuntimeStopGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStopGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStopGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStopGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStopGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStopGenerationId {
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
///`RuntimeStopKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.stop"
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
pub enum RuntimeStopKind {
    #[serde(rename = "runtime.stop")]
    RuntimeStop,
}
impl ::std::convert::From<&Self> for RuntimeStopKind {
    fn from(value: &RuntimeStopKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeStopKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeStop => f.write_str("runtime.stop"),
        }
    }
}
impl ::std::str::FromStr for RuntimeStopKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.stop" => Ok(Self::RuntimeStop),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeStopKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStopKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStopKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeStopProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeStopProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeStopProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStopProtocolVersion> for i64 {
    fn from(value: RuntimeStopProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStopProtocolVersion> for RuntimeStopProtocolVersion {
    fn from(value: &RuntimeStopProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeStopProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStopProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeStopRequestId`
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
pub struct RuntimeStopRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeStopRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStopRequestId> for ::std::string::String {
    fn from(value: RuntimeStopRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStopRequestId> for RuntimeStopRequestId {
    fn from(value: &RuntimeStopRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStopRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStopRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStopRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStopRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStopRequestId {
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
///`RuntimeStopped`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RuntimeStopped",
///  "type": "object",
///  "required": [
///    "correlationId",
///    "generationId",
///    "kind",
///    "protocolVersion",
///    "requestId"
///  ],
///  "properties": {
///    "correlationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "generationId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "kind": {
///      "type": "string",
///      "enum": [
///        "runtime.stopped"
///      ]
///    },
///    "protocolVersion": {
///      "type": "integer",
///      "enum": [
///        2
///      ]
///    },
///    "requestId": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RuntimeStopped {
    #[serde(rename = "correlationId")]
    pub correlation_id: RuntimeStoppedCorrelationId,
    #[serde(rename = "generationId")]
    pub generation_id: RuntimeStoppedGenerationId,
    pub kind: RuntimeStoppedKind,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: RuntimeStoppedProtocolVersion,
    #[serde(rename = "requestId")]
    pub request_id: RuntimeStoppedRequestId,
}
impl ::std::convert::From<&RuntimeStopped> for RuntimeStopped {
    fn from(value: &RuntimeStopped) -> Self {
        value.clone()
    }
}
///`RuntimeStoppedCorrelationId`
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
pub struct RuntimeStoppedCorrelationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStoppedCorrelationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStoppedCorrelationId> for ::std::string::String {
    fn from(value: RuntimeStoppedCorrelationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStoppedCorrelationId> for RuntimeStoppedCorrelationId {
    fn from(value: &RuntimeStoppedCorrelationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStoppedCorrelationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStoppedCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStoppedCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStoppedCorrelationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStoppedCorrelationId {
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
///`RuntimeStoppedGenerationId`
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
pub struct RuntimeStoppedGenerationId(::std::string::String);
impl ::std::ops::Deref for RuntimeStoppedGenerationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStoppedGenerationId> for ::std::string::String {
    fn from(value: RuntimeStoppedGenerationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStoppedGenerationId> for RuntimeStoppedGenerationId {
    fn from(value: &RuntimeStoppedGenerationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStoppedGenerationId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStoppedGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStoppedGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStoppedGenerationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStoppedGenerationId {
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
///`RuntimeStoppedKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "runtime.stopped"
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
pub enum RuntimeStoppedKind {
    #[serde(rename = "runtime.stopped")]
    RuntimeStopped,
}
impl ::std::convert::From<&Self> for RuntimeStoppedKind {
    fn from(value: &RuntimeStoppedKind) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuntimeStoppedKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RuntimeStopped => f.write_str("runtime.stopped"),
        }
    }
}
impl ::std::str::FromStr for RuntimeStoppedKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "runtime.stopped" => Ok(Self::RuntimeStopped),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuntimeStoppedKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStoppedKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStoppedKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RuntimeStoppedProtocolVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "enum": [
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RuntimeStoppedProtocolVersion(i64);
impl ::std::ops::Deref for RuntimeStoppedProtocolVersion {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStoppedProtocolVersion> for i64 {
    fn from(value: RuntimeStoppedProtocolVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStoppedProtocolVersion>
for RuntimeStoppedProtocolVersion {
    fn from(value: &RuntimeStoppedProtocolVersion) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for RuntimeStoppedProtocolVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStoppedProtocolVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`RuntimeStoppedRequestId`
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
pub struct RuntimeStoppedRequestId(::std::string::String);
impl ::std::ops::Deref for RuntimeStoppedRequestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuntimeStoppedRequestId> for ::std::string::String {
    fn from(value: RuntimeStoppedRequestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RuntimeStoppedRequestId> for RuntimeStoppedRequestId {
    fn from(value: &RuntimeStoppedRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RuntimeStoppedRequestId {
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
impl ::std::convert::TryFrom<&str> for RuntimeStoppedRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuntimeStoppedRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuntimeStoppedRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuntimeStoppedRequestId {
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
///`Target`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Target",
///  "type": "object",
///  "required": [
///    "bounds",
///    "pid",
///    "title",
///    "window_id"
///  ],
///  "properties": {
///    "bounds": {
///      "$ref": "#/definitions/Rect"
///    },
///    "pid": {
///      "type": "integer",
///      "minimum": 1.0
///    },
///    "title": {
///      "type": "string",
///      "maxLength": 256
///    },
///    "window_id": {
///      "type": "integer",
///      "minimum": 0.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub bounds: Rect,
    pub pid: ::std::num::NonZeroU64,
    pub title: TargetTitle,
    pub window_id: u64,
}
impl ::std::convert::From<&Target> for Target {
    fn from(value: &Target) -> Self {
        value.clone()
    }
}
///`TargetTitle`
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
pub struct TargetTitle(::std::string::String);
impl ::std::ops::Deref for TargetTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TargetTitle> for ::std::string::String {
    fn from(value: TargetTitle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TargetTitle> for TargetTitle {
    fn from(value: &TargetTitle) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TargetTitle {
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
impl ::std::convert::TryFrom<&str> for TargetTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TargetTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TargetTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TargetTitle {
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
///`TeachingCue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TeachingCue",
///  "type": "object",
///  "required": [
///    "caption",
///    "destination",
///    "direction",
///    "element_id",
///    "expires_at",
///    "gesture",
///    "id",
///    "locale",
///    "observation_id",
///    "source"
///  ],
///  "properties": {
///    "caption": {
///      "type": "string",
///      "maxLength": 400,
///      "minLength": 1
///    },
///    "destination": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Rect"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "direction": {
///      "anyOf": [
///        {
///          "type": "string",
///          "enum": [
///            "up",
///            "down",
///            "left",
///            "right"
///          ]
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "element_id": {
///      "type": "string",
///      "maxLength": 80
///    },
///    "expires_at": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "gesture": {
///      "type": "string",
///      "enum": [
///        "point",
///        "click",
///        "drag",
///        "type",
///        "scroll"
///      ]
///    },
///    "grounding": {
///      "type": "string",
///      "enum": [
///        "accessibility",
///        "visual"
///      ]
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "locale": {
///      "type": "string",
///      "enum": [
///        "en",
///        "vi"
///      ]
///    },
///    "observation_id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "screenshot_id": {
///      "anyOf": [
///        {
///          "type": "string",
///          "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "source": {
///      "$ref": "#/definitions/Rect"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeachingCue {
    pub caption: TeachingCueCaption,
    pub destination: ::std::option::Option<Rect>,
    pub direction: ::std::option::Option<TeachingCueDirection>,
    pub element_id: TeachingCueElementId,
    pub expires_at: f64,
    pub gesture: TeachingCueGesture,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub grounding: ::std::option::Option<TeachingCueGrounding>,
    pub id: TeachingCueId,
    pub locale: TeachingCueLocale,
    pub observation_id: TeachingCueObservationId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub screenshot_id: ::std::option::Option<TeachingCueScreenshotId>,
    pub source: Rect,
}
impl ::std::convert::From<&TeachingCue> for TeachingCue {
    fn from(value: &TeachingCue) -> Self {
        value.clone()
    }
}
///`TeachingCueCaption`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 400,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TeachingCueCaption(::std::string::String);
impl ::std::ops::Deref for TeachingCueCaption {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeachingCueCaption> for ::std::string::String {
    fn from(value: TeachingCueCaption) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TeachingCueCaption> for TeachingCueCaption {
    fn from(value: &TeachingCueCaption) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TeachingCueCaption {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 400usize {
            return Err("longer than 400 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TeachingCueCaption {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueCaption {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueCaption {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeachingCueCaption {
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
///`TeachingCueDirection`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "up",
///    "down",
///    "left",
///    "right"
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
pub enum TeachingCueDirection {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}
impl ::std::convert::From<&Self> for TeachingCueDirection {
    fn from(value: &TeachingCueDirection) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingCueDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
        }
    }
}
impl ::std::str::FromStr for TeachingCueDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingCueDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingCueElementId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 80
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TeachingCueElementId(::std::string::String);
impl ::std::ops::Deref for TeachingCueElementId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeachingCueElementId> for ::std::string::String {
    fn from(value: TeachingCueElementId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TeachingCueElementId> for TeachingCueElementId {
    fn from(value: &TeachingCueElementId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TeachingCueElementId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 80usize {
            return Err("longer than 80 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TeachingCueElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueElementId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeachingCueElementId {
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
///`TeachingCueGesture`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "point",
///    "click",
///    "drag",
///    "type",
///    "scroll"
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
pub enum TeachingCueGesture {
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "drag")]
    Drag,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "scroll")]
    Scroll,
}
impl ::std::convert::From<&Self> for TeachingCueGesture {
    fn from(value: &TeachingCueGesture) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingCueGesture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Point => f.write_str("point"),
            Self::Click => f.write_str("click"),
            Self::Drag => f.write_str("drag"),
            Self::Type => f.write_str("type"),
            Self::Scroll => f.write_str("scroll"),
        }
    }
}
impl ::std::str::FromStr for TeachingCueGesture {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "point" => Ok(Self::Point),
            "click" => Ok(Self::Click),
            "drag" => Ok(Self::Drag),
            "type" => Ok(Self::Type),
            "scroll" => Ok(Self::Scroll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingCueGesture {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueGesture {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueGesture {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingCueGrounding`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "accessibility",
///    "visual"
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
pub enum TeachingCueGrounding {
    #[serde(rename = "accessibility")]
    Accessibility,
    #[serde(rename = "visual")]
    Visual,
}
impl ::std::convert::From<&Self> for TeachingCueGrounding {
    fn from(value: &TeachingCueGrounding) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingCueGrounding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accessibility => f.write_str("accessibility"),
            Self::Visual => f.write_str("visual"),
        }
    }
}
impl ::std::str::FromStr for TeachingCueGrounding {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accessibility" => Ok(Self::Accessibility),
            "visual" => Ok(Self::Visual),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingCueGrounding {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueGrounding {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueGrounding {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingCueId`
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
pub struct TeachingCueId(::std::string::String);
impl ::std::ops::Deref for TeachingCueId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeachingCueId> for ::std::string::String {
    fn from(value: TeachingCueId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TeachingCueId> for TeachingCueId {
    fn from(value: &TeachingCueId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TeachingCueId {
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
impl ::std::convert::TryFrom<&str> for TeachingCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeachingCueId {
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
///`TeachingCueLocale`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "en",
///    "vi"
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
pub enum TeachingCueLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "vi")]
    Vi,
}
impl ::std::convert::From<&Self> for TeachingCueLocale {
    fn from(value: &TeachingCueLocale) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingCueLocale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::En => f.write_str("en"),
            Self::Vi => f.write_str("vi"),
        }
    }
}
impl ::std::str::FromStr for TeachingCueLocale {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "en" => Ok(Self::En),
            "vi" => Ok(Self::Vi),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingCueLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueLocale {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingCueObservationId`
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
pub struct TeachingCueObservationId(::std::string::String);
impl ::std::ops::Deref for TeachingCueObservationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeachingCueObservationId> for ::std::string::String {
    fn from(value: TeachingCueObservationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TeachingCueObservationId> for TeachingCueObservationId {
    fn from(value: &TeachingCueObservationId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TeachingCueObservationId {
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
impl ::std::convert::TryFrom<&str> for TeachingCueObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueObservationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeachingCueObservationId {
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
///`TeachingCueScreenshotId`
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
pub struct TeachingCueScreenshotId(::std::string::String);
impl ::std::ops::Deref for TeachingCueScreenshotId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeachingCueScreenshotId> for ::std::string::String {
    fn from(value: TeachingCueScreenshotId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TeachingCueScreenshotId> for TeachingCueScreenshotId {
    fn from(value: &TeachingCueScreenshotId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TeachingCueScreenshotId {
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
impl ::std::convert::TryFrom<&str> for TeachingCueScreenshotId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingCueScreenshotId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingCueScreenshotId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeachingCueScreenshotId {
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
///`TeachingState`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "TeachingState",
///  "type": "object",
///  "required": [
///    "check",
///    "cue",
///    "observation",
///    "revision",
///    "session_id",
///    "target",
///    "targets"
///  ],
///  "properties": {
///    "check": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/CheckResult"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "cue": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/TeachingCue"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "journey": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Journey"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "observation": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Observation"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "readiness": {
///      "type": "object",
///      "required": [
///        "accessibility",
///        "model",
///        "observation",
///        "reason",
///        "screen"
///      ],
///      "properties": {
///        "accessibility": {
///          "enum": [
///            "unknown",
///            "available",
///            "unavailable"
///          ]
///        },
///        "model": {
///          "enum": [
///            "unconfigured",
///            "ready",
///            "unavailable"
///          ]
///        },
///        "observation": {
///          "enum": [
///            "unknown",
///            "available",
///            "unavailable"
///          ]
///        },
///        "reason": {
///          "enum": [
///            "none",
///            "connect_model",
///            "observe_again",
///            "retry_plan"
///          ]
///        },
///        "screen": {
///          "enum": [
///            "unknown",
///            "available",
///            "unavailable"
///          ]
///        }
///      },
///      "additionalProperties": false
///    },
///    "revision": {
///      "type": "integer",
///      "minimum": 0.0
///    },
///    "session_id": {
///      "anyOf": [
///        {
///          "type": "string",
///          "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "target": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Target"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "targets": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Target"
///      },
///      "maxItems": 100
///    },
///    "timings": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "elapsed_ms",
///          "phase"
///        ],
///        "properties": {
///          "elapsed_ms": {
///            "type": "number",
///            "maximum": 3600000.0,
///            "minimum": 0.0
///          },
///          "phase": {
///            "enum": [
///              "observation",
///              "model",
///              "grounding"
///            ]
///          }
///        },
///        "additionalProperties": false
///      },
///      "maxItems": 200
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeachingState {
    pub check: ::std::option::Option<CheckResult>,
    pub cue: ::std::option::Option<TeachingCue>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub journey: ::std::option::Option<Journey>,
    pub observation: ::std::option::Option<Observation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub readiness: ::std::option::Option<TeachingStateReadiness>,
    pub revision: u64,
    pub session_id: ::std::option::Option<TeachingStateSessionId>,
    pub target: ::std::option::Option<Target>,
    pub targets: ::std::vec::Vec<Target>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub timings: ::std::vec::Vec<TeachingStateTimingsItem>,
}
impl ::std::convert::From<&TeachingState> for TeachingState {
    fn from(value: &TeachingState) -> Self {
        value.clone()
    }
}
///`TeachingStateReadiness`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "accessibility",
///    "model",
///    "observation",
///    "reason",
///    "screen"
///  ],
///  "properties": {
///    "accessibility": {
///      "enum": [
///        "unknown",
///        "available",
///        "unavailable"
///      ]
///    },
///    "model": {
///      "enum": [
///        "unconfigured",
///        "ready",
///        "unavailable"
///      ]
///    },
///    "observation": {
///      "enum": [
///        "unknown",
///        "available",
///        "unavailable"
///      ]
///    },
///    "reason": {
///      "enum": [
///        "none",
///        "connect_model",
///        "observe_again",
///        "retry_plan"
///      ]
///    },
///    "screen": {
///      "enum": [
///        "unknown",
///        "available",
///        "unavailable"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeachingStateReadiness {
    pub accessibility: TeachingStateReadinessAccessibility,
    pub model: TeachingStateReadinessModel,
    pub observation: TeachingStateReadinessObservation,
    pub reason: TeachingStateReadinessReason,
    pub screen: TeachingStateReadinessScreen,
}
impl ::std::convert::From<&TeachingStateReadiness> for TeachingStateReadiness {
    fn from(value: &TeachingStateReadiness) -> Self {
        value.clone()
    }
}
///`TeachingStateReadinessAccessibility`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "unknown",
///    "available",
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
pub enum TeachingStateReadinessAccessibility {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::convert::From<&Self> for TeachingStateReadinessAccessibility {
    fn from(value: &TeachingStateReadinessAccessibility) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingStateReadinessAccessibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::Available => f.write_str("available"),
            Self::Unavailable => f.write_str("unavailable"),
        }
    }
}
impl ::std::str::FromStr for TeachingStateReadinessAccessibility {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "available" => Ok(Self::Available),
            "unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingStateReadinessAccessibility {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for TeachingStateReadinessAccessibility {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for TeachingStateReadinessAccessibility {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingStateReadinessModel`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "unconfigured",
///    "ready",
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
pub enum TeachingStateReadinessModel {
    #[serde(rename = "unconfigured")]
    Unconfigured,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::convert::From<&Self> for TeachingStateReadinessModel {
    fn from(value: &TeachingStateReadinessModel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingStateReadinessModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unconfigured => f.write_str("unconfigured"),
            Self::Ready => f.write_str("ready"),
            Self::Unavailable => f.write_str("unavailable"),
        }
    }
}
impl ::std::str::FromStr for TeachingStateReadinessModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unconfigured" => Ok(Self::Unconfigured),
            "ready" => Ok(Self::Ready),
            "unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingStateReadinessModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingStateReadinessModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingStateReadinessModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingStateReadinessObservation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "unknown",
///    "available",
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
pub enum TeachingStateReadinessObservation {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::convert::From<&Self> for TeachingStateReadinessObservation {
    fn from(value: &TeachingStateReadinessObservation) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingStateReadinessObservation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::Available => f.write_str("available"),
            Self::Unavailable => f.write_str("unavailable"),
        }
    }
}
impl ::std::str::FromStr for TeachingStateReadinessObservation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "available" => Ok(Self::Available),
            "unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingStateReadinessObservation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for TeachingStateReadinessObservation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for TeachingStateReadinessObservation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingStateReadinessReason`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "none",
///    "connect_model",
///    "observe_again",
///    "retry_plan"
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
pub enum TeachingStateReadinessReason {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "connect_model")]
    ConnectModel,
    #[serde(rename = "observe_again")]
    ObserveAgain,
    #[serde(rename = "retry_plan")]
    RetryPlan,
}
impl ::std::convert::From<&Self> for TeachingStateReadinessReason {
    fn from(value: &TeachingStateReadinessReason) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingStateReadinessReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::ConnectModel => f.write_str("connect_model"),
            Self::ObserveAgain => f.write_str("observe_again"),
            Self::RetryPlan => f.write_str("retry_plan"),
        }
    }
}
impl ::std::str::FromStr for TeachingStateReadinessReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "connect_model" => Ok(Self::ConnectModel),
            "observe_again" => Ok(Self::ObserveAgain),
            "retry_plan" => Ok(Self::RetryPlan),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingStateReadinessReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingStateReadinessReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingStateReadinessReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingStateReadinessScreen`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "unknown",
///    "available",
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
pub enum TeachingStateReadinessScreen {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::convert::From<&Self> for TeachingStateReadinessScreen {
    fn from(value: &TeachingStateReadinessScreen) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingStateReadinessScreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::Available => f.write_str("available"),
            Self::Unavailable => f.write_str("unavailable"),
        }
    }
}
impl ::std::str::FromStr for TeachingStateReadinessScreen {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "available" => Ok(Self::Available),
            "unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingStateReadinessScreen {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingStateReadinessScreen {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingStateReadinessScreen {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TeachingStateSessionId`
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
pub struct TeachingStateSessionId(::std::string::String);
impl ::std::ops::Deref for TeachingStateSessionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeachingStateSessionId> for ::std::string::String {
    fn from(value: TeachingStateSessionId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TeachingStateSessionId> for TeachingStateSessionId {
    fn from(value: &TeachingStateSessionId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TeachingStateSessionId {
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
impl ::std::convert::TryFrom<&str> for TeachingStateSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingStateSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingStateSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeachingStateSessionId {
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
///`TeachingStateTimingsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "elapsed_ms",
///    "phase"
///  ],
///  "properties": {
///    "elapsed_ms": {
///      "type": "number",
///      "maximum": 3600000.0,
///      "minimum": 0.0
///    },
///    "phase": {
///      "enum": [
///        "observation",
///        "model",
///        "grounding"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeachingStateTimingsItem {
    pub elapsed_ms: f64,
    pub phase: TeachingStateTimingsItemPhase,
}
impl ::std::convert::From<&TeachingStateTimingsItem> for TeachingStateTimingsItem {
    fn from(value: &TeachingStateTimingsItem) -> Self {
        value.clone()
    }
}
///`TeachingStateTimingsItemPhase`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "observation",
///    "model",
///    "grounding"
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
pub enum TeachingStateTimingsItemPhase {
    #[serde(rename = "observation")]
    Observation,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "grounding")]
    Grounding,
}
impl ::std::convert::From<&Self> for TeachingStateTimingsItemPhase {
    fn from(value: &TeachingStateTimingsItemPhase) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TeachingStateTimingsItemPhase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Observation => f.write_str("observation"),
            Self::Model => f.write_str("model"),
            Self::Grounding => f.write_str("grounding"),
        }
    }
}
impl ::std::str::FromStr for TeachingStateTimingsItemPhase {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "observation" => Ok(Self::Observation),
            "model" => Ok(Self::Model),
            "grounding" => Ok(Self::Grounding),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TeachingStateTimingsItemPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TeachingStateTimingsItemPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeachingStateTimingsItemPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
