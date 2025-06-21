'use client'

import type { ThemeProviderProps } from 'next-themes'

import { AppProgressProvider as ProgressProvider } from '@bprogress/next'
import { HeroUIProvider } from '@heroui/system'
import { ThemeProvider as NextThemesProvider } from 'next-themes'
import { useRouter } from 'next/navigation'
import * as React from 'react'

export interface ProvidersProps {
  children: React.ReactNode
  themeProps?: ThemeProviderProps
}

export function Providers({ children, themeProps }: ProvidersProps) {
  const router = useRouter()

  return (
    <HeroUIProvider navigate={router.push}>
      <NextThemesProvider attribute='class' {...themeProps}>
        <ProgressProvider
          shallowRouting
          color='#006FEE'
          height='4px'
          options={{ showSpinner: false }}
        >
          {children}
        </ProgressProvider>
      </NextThemesProvider>
    </HeroUIProvider>
  )
}
