import js from '@eslint/js';
import ts from 'typescript-eslint';
export default ts.config(
  {
    ignores: [
      '**/generated*',
      '**/gen/**',
      '**/dist/**',
      '**/target/**',
      '**/.venv/**',
      '**/.local/**',
    ],
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  {
    files: ['**/*.{mjs,ts,tsx}'],
    languageOptions: {
      globals: {
        process: 'readonly',
        console: 'readonly',
        Buffer: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
        URL: 'readonly',
        fetch: 'readonly',
        performance: 'readonly',
        window: 'readonly',
        document: 'readonly',
        AbortSignal: 'readonly',
      },
    },
  },
);
