import localFont from 'next/font/local'

export const fontSans = localFont({
  src: [
    {
      path: '../app/fonts/inter-latin-wght-normal.woff2',
      weight: '100 900',
      style: 'normal',
    },
    {
      path: '../app/fonts/inter-latin-ext-wght-normal.woff2',
      weight: '100 900',
      style: 'normal',
    },
  ],
  variable: '--font-inter',
  display: 'swap',
  fallback: ['system-ui', 'sans-serif'],
})

export const fontMono = localFont({
  src: [
    {
      path: '../app/fonts/fira-code-latin-wght-normal.woff2',
      weight: '300 700',
      style: 'normal',
    },
    {
      path: '../app/fonts/fira-code-latin-ext-wght-normal.woff2',
      weight: '300 700',
      style: 'normal',
    },
  ],
  variable: '--font-fira-code',
  display: 'swap',
  fallback: ['SFMono-Regular', 'ui-monospace', 'monospace'],
})
