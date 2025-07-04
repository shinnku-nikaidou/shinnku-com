import '@/styles/index.css'
import { Metadata, Viewport } from 'next'
import Link from 'next/link'
import Script from 'next/script'

import { Providers } from './providers'

import { BackgroundBox } from '@/components/BackgroundBox'
import { Navbar } from '@/components/primitives/Navbar'
import { fontSans } from '@/config/fonts'
import { siteConfig } from '@/config/site'
import { cn } from '@/lib/utils'

export const metadata: Metadata = {
  metadataBase: new URL('https://www.shinnku.com'),
  title: {
    default: siteConfig.name,
    template: `%s - ${siteConfig.name}`,
  },
  description: siteConfig.description,
  icons: {
    icon: '/favicon.ico',
  },
}

export const viewport: Viewport = {
  themeColor: [
    { media: '(prefers-color-scheme: light)', color: 'white' },
    { media: '(prefers-color-scheme: dark)', color: 'black' },
  ],
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html suppressHydrationWarning lang='en'>
      <head>
        <Script
          async
          src='https://www.googletagmanager.com/gtag/js?id=G-61P3NL510C'
        />
        <Script id='google-analytics'>
          {`
          window.dataLayer = window.dataLayer || [];
          function gtag(){dataLayer.push(arguments);}
          gtag('js', new Date());
          gtag('config', 'G-61P3NL510C');
        `}
        </Script>
        <Script
          defer
          data-cf-beacon='{"token": "65325546c71740a78ecc6e8fa7815010"}'
          src='https://static.cloudflareinsights.com/beacon.min.js'
        />
      </head>
      <body
        className={cn(
          'bg-background min-h-screen font-sans antialiased',
          fontSans.variable,
        )}
      >
        <BackgroundBox />
        <Providers themeProps={{ attribute: 'class', defaultTheme: 'dark' }}>
          <div className='relative flex h-screen flex-col'>
            <Navbar />
            <main className='max-w-8xl container mx-auto grow px-6 pt-16'>
              {children}
            </main>
            <footer className='flex w-full items-center justify-center py-3'>
              <Link
                className='flex items-center gap-1 text-current'
                href='https://t.me/upsetgal'
                rel='noopener noreferrer'
                target='_blank'
                title='join telegram'
              >
                <span className='text-default-600'>Powered by</span>
                <p className='text-primary'>Shinnku; </p>
                <span className='text-default-600'>join our telegram</span>
              </Link>
            </footer>
          </div>
        </Providers>
      </body>
    </html>
  )
}
