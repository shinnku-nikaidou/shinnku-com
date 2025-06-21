import enUs from './en-us'
import zhCn from './zh-cn'
import zhTw from './zh-tw'

const locales = {
  'zh-cn': zhCn,
  'zh-tw': zhTw,
  'en-us': enUs,
} as const

export const t = (key: keyof typeof zhCn) => {
  const messages = locales['zh-cn']

  return messages[key] ?? zhCn[key]
}
