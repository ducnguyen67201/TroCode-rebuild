import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { parsePermissionSettingsGuide } from '@tro/contracts';
import type {
  PermissionSettingsGuide,
  PermissionSettingsTarget,
} from '@tro/contracts';

interface GuideCopy {
  title: string;
  path: string;
  instruction: string;
}

export function permissionSettingsCopy(
  platform: PermissionSettingsGuide['platform'],
  target: PermissionSettingsTarget,
): GuideCopy {
  if (platform === 'windows') {
    return {
      title: 'Microphone access',
      path: 'Privacy & security → Microphone',
      instruction:
        'Turn on Microphone access, then turn on “Let desktop apps access your microphone.” Windows uses this shared switch for Tro.',
    };
  }
  if (target === 'accessibility') {
    return {
      title: 'Accessibility',
      path: 'Privacy & Security → Accessibility',
      instruction:
        'Find Tro and turn it on. If Tro is missing, click +, then choose Applications → Tro.app → Open. Drag Tro.app from Applications into the app list also works.',
    };
  }
  if (target === 'microphone') {
    return {
      title: 'Microphone',
      path: 'Privacy & Security → Microphone',
      instruction:
        'Find Tro and turn it on. If Tro is missing, click +, then choose Applications → Tro.app → Open.',
    };
  }
  return {
    title: 'Screen Recording',
    path: 'Privacy & Security → Screen & System Audio Recording',
    instruction:
      'Find Tro and turn it on. If Tro is missing, click +, then choose Applications → Tro.app → Open. Drag Tro.app from Applications into the app list also works.',
  };
}

export function PermissionSettingsGuideView({
  guide,
}: {
  guide: PermissionSettingsGuide;
}) {
  const copy = permissionSettingsCopy(guide.platform, guide.target);
  return (
    <aside className="permission-guide" aria-label="Tro permission guide">
      <div className="permission-guide-kicker">Look in System Settings</div>
      <div className="permission-guide-target">Find Tro in this list</div>
      <h1>{copy.title}</h1>
      <p className="permission-guide-path">{copy.path}</p>
      <p>{copy.instruction}</p>
      <div className="permission-guide-boundary">
        You make every change. Tro only points out where.
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
