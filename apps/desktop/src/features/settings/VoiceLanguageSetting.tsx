import { useEffect, useRef, useState } from 'react';
import {
  DEFAULT_TRANSCRIPTION_LANGUAGE,
  TRANSCRIPTION_LANGUAGES,
  type TranscriptionLanguage,
} from '@tro/contracts';
import type { VoiceClient } from '../../platform/voice-client';
import { useLanguage, type TranslationKey } from '../../i18n';

const LANGUAGE_KEYS = {
  vi: 'voiceLanguage.vietnamese',
  en: 'voiceLanguage.english',
  auto: 'voiceLanguage.auto',
} satisfies Record<TranscriptionLanguage, TranslationKey>;

export function VoiceLanguageSetting({ client }: { client: VoiceClient }) {
  const { t } = useLanguage();
  const [language, setLanguage] = useState<TranscriptionLanguage | null>(null);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<TranslationKey | null>(null);
  const mounted = useRef(false);

  useEffect(() => {
    mounted.current = true;
    void client
      .status()
      .then((status) => {
        if (mounted.current) setLanguage(status.transcriptionLanguage);
      })
      .catch(() => {
        if (mounted.current) setError('voiceLanguage.unavailable');
      });
    return () => {
      mounted.current = false;
    };
  }, [client]);

  async function change(next: TranscriptionLanguage) {
    const previous = language;
    setLanguage(next);
    setSaving(true);
    setError(null);
    try {
      const status = await client.setTranscriptionLanguage(next);
      if (mounted.current) setLanguage(status.transcriptionLanguage);
    } catch {
      if (mounted.current) {
        setLanguage(previous);
        setError('voiceLanguage.saveError');
      }
    } finally {
      if (mounted.current) setSaving(false);
    }
  }

  return (
    <article className="voice-language-setting" aria-busy={language === null}>
      <div>
        <p className="settings-label">{t('voiceLanguage.category')}</p>
        <label htmlFor="transcription-language">
          {t('voiceLanguage.label')}
        </label>
        <p>{t('voiceLanguage.description')}</p>
      </div>
      <select
        id="transcription-language"
        value={language ?? DEFAULT_TRANSCRIPTION_LANGUAGE}
        disabled={language === null || saving}
        onChange={(event) =>
          void change(event.target.value as TranscriptionLanguage)
        }
      >
        {TRANSCRIPTION_LANGUAGES.map((value) => (
          <option key={value} value={value}>
            {t(LANGUAGE_KEYS[value])}
          </option>
        ))}
      </select>
      {saving && (
        <span className="voice-language-saving">
          {t('voiceLanguage.saving')}
        </span>
      )}
      {error && (
        <p className="voice-language-error" role="alert">
          {t(error)}
        </p>
      )}
    </article>
  );
}
