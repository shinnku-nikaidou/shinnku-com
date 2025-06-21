'use client'

import { useTheme } from 'next-themes'
import { FC, useEffect, useState } from 'react'

import { MoonFilledIcon, SunFilledIcon } from '@/components/ui/icons'
import { Switch } from '@/components/ui/switch'
import { cn } from '@/lib/utils'

export interface ThemeSwitchProps {
  className?: string
}

export const ThemeSwitch: FC<ThemeSwitchProps> = ({ className }) => {
  const { theme, setTheme } = useTheme()
  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    setMounted(true)
    if (!localStorage.getItem('theme')) {
      setTheme('light')
    }
  }, [setTheme])

  const isDark = mounted ? theme === 'dark' : false

  const toggleTheme = () => {
    setTheme(isDark ? 'light' : 'dark')
  }

  return (
    <Switch
      aria-label={`Switch to ${isDark ? 'light' : 'dark'} mode`}
      checked={isDark}
      className={cn(
        'px-px transition-opacity hover:opacity-80 cursor-pointer',
        className,
      )}
      onCheckedChange={toggleTheme}
    >
      {isDark ? <MoonFilledIcon size={16} /> : <SunFilledIcon size={16} />}
    </Switch>
  )
}
