import js from '@eslint/js';
import globals from 'globals';
import svelteConfig from 'eslint-plugin-svelte';
import prettier from 'eslint-config-prettier';
import tseslint from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';

export default [
  js.configs.recommended,
  ...svelteConfig.configs['flat/recommended'],
  prettier,
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node
      }
    }
  },
  {
    files: ['**/*.ts', '**/*.tsx'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: 'module'
      }
    },
    plugins: {
      '@typescript-eslint': tseslint
    },
    rules: {
      ...tseslint.configs.recommended.rules,
      '@typescript-eslint/no-unused-vars': 'warn',
      '@typescript-eslint/no-explicit-any': 'warn'
    }
  },
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parserOptions: {
        parser: tsParser
      }
    }
  },
  {
    files: ['src/routes/docs/**/*.svelte', 'src/lib/components/FileBrowser.svelte'],
    rules: {
      'svelte/no-at-html-tags': 'off' // Allow {@html} in documentation pages
    }
  },
  {
    ignores: ['build/', '.svelte-kit/', 'dist/', 'node_modules/', '../backend/**/*']
  }
];