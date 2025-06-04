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
  let lang
  if (typeof document !== 'undefined') {
    lang = document.documentElement.lang
  } else {
    lang = 'zh-cn'
  }
  return (lang as Locale) || 'zh-cn'
}

export const t = (key: keyof typeof zhCn) => {
  const locale = getLocale()
  const messages = locales[locale]

  return messages[key] ?? zhCn[key]
}
