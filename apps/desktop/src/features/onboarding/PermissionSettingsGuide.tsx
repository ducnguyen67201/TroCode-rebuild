import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { parsePermissionSettingsGuide } from '@tro/contracts';
import type {
  PermissionSettingsGuide,
  PermissionSettingsTarget,
} from '@tro/contracts';
import {
  translate,
  useLanguage,
  type AppLocale,
  type TranslationKey,
} from '../../i18n';

interface GuideCopy {
  title: string;
  path: string;
  instruction: string;
}

export function permissionSettingsCopy(
  platform: PermissionSettingsGuide['platform'],
  target: PermissionSettingsTarget,
  locale: AppLocale = 'en',
): GuideCopy {
  if (platform === 'windows') {
    return guideCopy(locale, 'permissionGuide.windowsMicrophone');
  }
  if (target === 'accessibility') {
    return guideCopy(locale, 'permissionGuide.accessibility');
  }
  if (target === 'microphone') {
    return guideCopy(locale, 'permissionGuide.microphone');
  }
  return guideCopy(locale, 'permissionGuide.screenCapture');
}

function guideCopy(
  locale: AppLocale,
  prefix:
    | 'permissionGuide.windowsMicrophone'
    | 'permissionGuide.accessibility'
    | 'permissionGuide.microphone'
    | 'permissionGuide.screenCapture',
): GuideCopy {
  return {
    title: translate(locale, `${prefix}.title` as TranslationKey),
    path: translate(locale, `${prefix}.path` as TranslationKey),
    instruction: translate(locale, `${prefix}.instruction` as TranslationKey),
  };
}

export function PermissionSettingsGuideView({
  guide,
}: {
  guide: PermissionSettingsGuide;
}) {
  const { locale, t } = useLanguage();
  const copy = permissionSettingsCopy(guide.platform, guide.target, locale);
  return (
    <aside className="permission-guide" aria-label={t('permissionGuide.aria')}>
      <div className="permission-guide-kicker">
        {t('permissionGuide.kicker')}
      </div>
      <div className="permission-guide-target">
        {t('permissionGuide.target')}
      </div>
      <h1>{copy.title}</h1>
      <p className="permission-guide-path">{copy.path}</p>
      <p>{copy.instruction}</p>
      <div className="permission-guide-boundary">
        {t('permissionGuide.boundary')}
      </div>
      <svg
        className="permission-guide-pointer"
        aria-hidden="true"
        focusable="false"
        viewBox="0 0 52 64"
      >
        <path d="M58 26H10m0 0 13-13M10 26l13 13" />
      </svg>
    </aside>
  );
}

export function PermissionSettingsGuideWindow() {
  const [guide, setGuide] = useState<PermissionSettingsGuide | null>(null);

  useEffect(() => {
    document.body.classList.add('permission-guide-page');
    let disposed = false;
    let cleanup: (() => void) | undefined;
    function update(value: unknown) {
      if (disposed) return;
      try {
        setGuide(parsePermissionSettingsGuide(value));
      } catch {
        setGuide(null);
      }
    }
    void listen('permission-settings-guide', (event) =>
      update(event.payload),
    ).then((unlisten) => {
      if (disposed) unlisten();
      else {
        cleanup = unlisten;
        void invoke('permission_settings_guide_current').then(update);
      }
    });
    return () => {
      disposed = true;
      cleanup?.();
      document.body.classList.remove('permission-guide-page');
    };
  }, []);

  return guide ? <PermissionSettingsGuideView guide={guide} /> : null;
}
