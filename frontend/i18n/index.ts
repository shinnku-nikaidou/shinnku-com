import { cookies } from 'next/headers'

import zhCn from './zh-cn'
import zhTw from './zh-tw'
import enUs from './en-us'

const locales = {
  'zh-cn': zhCn,
  'zh-tw': zhTw,
  'en-us': enUs,
} as const

type Locale = keyof typeof locales

export const t = (key: keyof typeof zhCn) => {
  const cookieLocale = cookies().get('NEXT_LOCALE')?.value as Locale | undefined
  const messages = locales[cookieLocale ?? 'zh-cn']

  return messages[key] ?? zhCn[key]
}
