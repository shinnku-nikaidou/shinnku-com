import zhCn from './zh-cn'
import zhTw from './zh-tw'
import enUs from './en-us'

const locales = {
  'zh-cn': zhCn,
  'zh-tw': zhTw,
  'en-us': enUs,
} as const

export type Locale = keyof typeof locales

const getLocale = (): Locale => {
  if (typeof window === 'undefined') {
    // Server environment; rely on the NEXT_LOCALE cookie if present
    const { cookies } = require('next/headers') as typeof import('next/headers')

    return (
      (cookies().get('NEXT_LOCALE')?.value as Locale | undefined) ?? 'zh-cn'
    )
  }

  // Client environment; fall back to <html lang="..."> attribute
  const lang = document.documentElement.lang

  return (lang as Locale) || 'zh-cn'
}

export const t = (key: keyof typeof zhCn) => {
  const locale = getLocale()
  const messages = locales[locale]

  return messages[key] ?? zhCn[key]
}
