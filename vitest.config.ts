import { defineConfig } from 'vitest/config';
export default defineConfig({
  test: {
    include: [
      'apps/desktop/src/**/*.test.{ts,tsx}',
      'packages/contracts/tests/*.test.ts',
    ],
    environment: 'jsdom',
  },
});
