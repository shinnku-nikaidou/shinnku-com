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

  const isLight = mounted ? theme === 'light' : true

  const toggleTheme = () => {
    setTheme(isLight ? 'dark' : 'light')
  }

  return (
    <Switch
      checked={isLight}
      onCheckedChange={toggleTheme}
      aria-label={`Switch to ${isLight ? 'dark' : 'light'} mode`}
      className={cn('px-px transition-opacity hover:opacity-80 cursor-pointer', className)}
    >
      {isLight ? <SunFilledIcon size={22} /> : <MoonFilledIcon size={22} />}
    </Switch>
  )
}
