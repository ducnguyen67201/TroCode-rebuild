import { useEffect, useRef, useState } from 'react';
import type { TranscriptionLanguage } from '@tro/contracts';
import type { VoiceClient } from '../../platform/voice-client';

export function VoiceLanguageSetting({ client }: { client: VoiceClient }) {
  const [language, setLanguage] = useState<TranscriptionLanguage | null>(null);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const mounted = useRef(false);

  useEffect(() => {
    mounted.current = true;
    void client
      .status()
      .then((status) => {
        if (mounted.current) setLanguage(status.transcriptionLanguage);
      })
      .catch(() => {
        if (mounted.current)
          setError('Transcription language is unavailable. Try again.');
      });
    return () => {
      mounted.current = false;
    };
  }, [client]);

  async function change(next: TranscriptionLanguage) {
    const previous = language;
    setLanguage(next);
    setSaving(true);
    setError('');
    try {
      const status = await client.setTranscriptionLanguage(next);
      if (mounted.current) setLanguage(status.transcriptionLanguage);
    } catch {
      if (mounted.current) {
        setLanguage(previous);
        setError('Transcription language could not be saved. Try again.');
      }
    } finally {
      if (mounted.current) setSaving(false);
    }
  }

  return (
    <article className="voice-language-setting" aria-busy={language === null}>
      <div>
        <p className="settings-label">Voice</p>
        <label htmlFor="transcription-language">Transcription language</label>
        <p>
          Choose the language you expect to speak. English and Vietnamese focus
          recognition; Auto detects the input language. Changes apply to your
          next voice instruction and do not translate it.
        </p>
      </div>
      <select
        id="transcription-language"
        value={language ?? 'vi'}
        disabled={language === null || saving}
        onChange={(event) =>
          void change(event.target.value as TranscriptionLanguage)
        }
      >
        <option value="vi">Vietnamese</option>
        <option value="en">English</option>
        <option value="auto">Auto</option>
      </select>
      {saving && <span className="voice-language-saving">Saving…</span>}
      {error && (
        <p className="voice-language-error" role="alert">
          {error}
        </p>
      )}
    </article>
  );
}
