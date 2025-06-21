'use client'

import type { ThemeProviderProps } from 'next-themes'

import { AppProgressProvider as ProgressProvider } from '@bprogress/next'
import { ThemeProvider as NextThemesProvider } from 'next-themes'
import * as React from 'react'

export interface ProvidersProps {
  children: React.ReactNode
  themeProps?: ThemeProviderProps
}

export function Providers({ children, themeProps }: ProvidersProps) {
  return (
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
  )
}
