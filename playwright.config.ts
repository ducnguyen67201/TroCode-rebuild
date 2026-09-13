import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests/acceptance',
  testMatch: '*.spec.ts',
  use: { baseURL: 'http://127.0.0.1:1421', browserName: 'chromium' },
  outputDir: '.local/playwright-results',
  webServer: {
    command:
      'node ../../node_modules/vite/bin/vite.js --host 127.0.0.1 --port 1421',
    cwd: 'apps/desktop',
    env: { VITE_TRO_PREVIEW: '1' },
    url: 'http://127.0.0.1:1421',
    reuseExistingServer: false,
  },
});
