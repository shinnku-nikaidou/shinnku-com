/** @type {import('eslint').Linter.FlatConfig[]} */
const tsParser = require('@typescript-eslint/parser')
const tsPlugin = require('@typescript-eslint/eslint-plugin')
const reactPlugin = require('eslint-plugin-react')
const hooksPlugin = require('eslint-plugin-react-hooks')
const jsxA11yPlugin = require('eslint-plugin-jsx-a11y')
const unusedImportsPlugin = require('eslint-plugin-unused-imports')
const importPlugin = require('eslint-plugin-import')
const prettierPlugin = require('eslint-plugin-prettier')
const nextPlugin = require('@next/eslint-plugin-next')

module.exports = [
  {
    ignores: [
      '.now/*',
      '*.css',
      '.changeset',
      'dist',
      'esm/*',
      'public/*',
      'tests/*',
      'scripts/*',
      '*.config.js',
      '.DS_Store',
      'node_modules',
      'coverage',
      '.next',
      'build',
      '!.commitlintrc.cjs',
      '!.lintstagedrc.cjs',
      '!jest.config.js',
      '!plopfile.js',
      '!react-shim.js',
      '!tsup.config.ts',
    ],
  },
  {
    files: ['**/*.{ts,tsx,js}'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2021,
        sourceType: 'module',
        ecmaFeatures: {
          jsx: true,
        },
      },
    },
    plugins: {
      '@typescript-eslint': tsPlugin,
      react: reactPlugin,
      'react-hooks': hooksPlugin,
      'jsx-a11y': jsxA11yPlugin,
      'unused-imports': unusedImportsPlugin,
      import: importPlugin,
      prettier: prettierPlugin,
      '@next/next': nextPlugin,
    },
    rules: {
      'no-console': 'warn',
      'react/prop-types': 'off',
      'react/jsx-uses-react': 'off',
      'react/react-in-jsx-scope': 'off',
      'react-hooks/exhaustive-deps': 'off',
      'jsx-a11y/click-events-have-key-events': 'warn',
      'jsx-a11y/interactive-supports-focus': 'warn',
      'prettier/prettier': 'warn',
      'no-unused-vars': 'off',
      'unused-imports/no-unused-vars': 'off',
      'unused-imports/no-unused-imports': 'warn',
      '@typescript-eslint/no-unused-vars': [
        'warn',
        {
          args: 'after-used',
          ignoreRestSiblings: false,
          argsIgnorePattern: '^_.*?$',
        },
      ],
      'import/order': [
        'warn',
        {
          groups: [
            'type',
            'builtin',
            'object',
            'external',
            'internal',
            'parent',
            'sibling',
            'index',
          ],
          pathGroups: [
            {
              pattern: '~/**',
              group: 'external',
              position: 'after',
            },
          ],
          'newlines-between': 'always',
        },
      ],
      'react/self-closing-comp': 'warn',
      'react/jsx-sort-props': [
        'warn',
        {
          callbacksLast: true,
          shorthandFirst: true,
          noSortAlphabetically: false,
          reservedFirst: true,
        },
      ],
      'padding-line-between-statements': [
        'warn',
        { blankLine: 'always', prev: '*', next: 'return' },
        { blankLine: 'always', prev: ['const', 'let', 'var'], next: '*' },
        {
          blankLine: 'any',
          prev: ['const', 'let', 'var'],
          next: ['const', 'let', 'var'],
        },
      ],
    },
  },
]
