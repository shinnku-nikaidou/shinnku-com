import { headers } from 'next/headers'
import { redirect } from 'next/navigation'

const locales = ['zh-cn', 'zh-tw', 'en-us'] as const
const defaultLocale = 'zh-cn'

function detectLocale(acceptLang?: string): string {
  if (!acceptLang) return defaultLocale
  const accepted = acceptLang.split(',').map((l) => l.trim().toLowerCase())
  for (const lang of accepted) {
    if (locales.includes(lang as any)) return lang
    const short = lang.split('-')[0]
    const match = locales.find((l) => l.startsWith(short))
    if (match) return match
  }
  return defaultLocale
}

export default function RootPage() {
  const locale = detectLocale(headers().get('accept-language') || undefined)
  redirect(`/${locale}`)
}
