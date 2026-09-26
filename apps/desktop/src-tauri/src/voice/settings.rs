use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionLanguage {
    Auto,
    En,
    #[default]
    Vi,
}

impl TranscriptionLanguage {
    pub fn request_languages(self) -> Vec<String> {
        match self {
            Self::Auto => Vec::new(),
            Self::En => vec!["en".to_owned()],
            Self::Vi => vec!["vi".to_owned()],
        }
    }
}

impl FromStr for TranscriptionLanguage {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "en" => Ok(Self::En),
            "vi" => Ok(Self::Vi),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct StoredVoiceSettings {
    transcription_language: TranscriptionLanguage,
}

#[derive(Clone, Debug, Default)]
pub struct VoiceSettingsStore {
    path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SettingsWriteError;

impl VoiceSettingsStore {
    pub fn at(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }

    pub fn load(&self) -> TranscriptionLanguage {
        self.path
            .as_ref()
            .and_then(|path| std::fs::read(path).ok())
            .and_then(|bytes| serde_json::from_slice::<StoredVoiceSettings>(&bytes).ok())
            .map(|settings| settings.transcription_language)
            .unwrap_or_default()
    }

    pub fn save(
        &self,
        transcription_language: TranscriptionLanguage,
    ) -> Result<(), SettingsWriteError> {
        let Some(path) = self.path.as_ref() else {
            return Ok(());
        };
        let parent = path.parent().ok_or(SettingsWriteError)?;
        std::fs::create_dir_all(parent).map_err(|_| SettingsWriteError)?;
        let bytes = serde_json::to_vec(&StoredVoiceSettings {
            transcription_language,
        })
        .map_err(|_| SettingsWriteError)?;
        std::fs::write(path, bytes).map_err(|_| SettingsWriteError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_vietnamese_and_maps_closed_provider_hints() {
        assert_eq!(TranscriptionLanguage::default(), TranscriptionLanguage::Vi);
        assert!(TranscriptionLanguage::Auto.request_languages().is_empty());
        assert_eq!(TranscriptionLanguage::En.request_languages(), vec!["en"]);
        assert_eq!(TranscriptionLanguage::Vi.request_languages(), vec!["vi"]);
        assert!("fr".parse::<TranscriptionLanguage>().is_err());
    }

    #[test]
    fn missing_or_invalid_settings_fall_back_to_vietnamese() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("voice-settings.json");
        let store = VoiceSettingsStore::at(path.clone());

        assert_eq!(store.load(), TranscriptionLanguage::Vi);
        std::fs::write(&path, br#"{"transcriptionLanguage":"vi""#).unwrap();
        assert_eq!(store.load(), TranscriptionLanguage::Vi);
        std::fs::write(&path, br#"{"transcriptionLanguage":"fr"}"#).unwrap();
        assert_eq!(store.load(), TranscriptionLanguage::Vi);
        std::fs::write(
            &path,
            br#"{"transcriptionLanguage":"en","unexpected":true}"#,
        )
        .unwrap();
        assert_eq!(store.load(), TranscriptionLanguage::Vi);
    }

    #[test]
    fn every_language_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let store = VoiceSettingsStore::at(
            directory
                .path()
                .join("not-created-yet")
                .join("voice-settings.json"),
        );

        for language in [
            TranscriptionLanguage::Auto,
            TranscriptionLanguage::En,
            TranscriptionLanguage::Vi,
        ] {
            store.save(language).unwrap();
            assert_eq!(store.load(), language);
        }
    }

    #[test]
    fn failed_save_does_not_succeed_silently() {
        let directory = tempfile::tempdir().unwrap();
        let blocked_parent = directory.path().join("not-a-directory");
        std::fs::write(&blocked_parent, b"blocked").unwrap();
        let store = VoiceSettingsStore::at(blocked_parent.join("voice-settings.json"));

        assert_eq!(
            store.save(TranscriptionLanguage::En),
            Err(SettingsWriteError)
        );
    }
}
