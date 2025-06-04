import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'

import { t } from '@/i18n/zh'

dayjs.extend(relativeTime)

const replaceTimeUnits = (input: string) => {
  const replacements: Record<string, string> = {
    an: '1',
    a: '1',
    second: t('seconds'),
    seconds: t('seconds'),
    minute: t('minutes'),
    minutes: t('minutes'),
    hour: t('hours'),
    hours: t('hours'),
    day: t('days'),
    days: t('days'),
    week: t('weeks'),
    weeks: t('weeks'),
    month: t('months'),
    months: t('months'),
    year: t('years'),
    years: t('years'),
  }

  const regex = new RegExp(Object.keys(replacements).join('|'), 'g')

  return input.replace(regex, (matched) => replacements[matched])
}

export const formatDistanceToNow = (pastTime: number | Date | string) => {
  const now = dayjs()
  const diffInSeconds = now.diff(pastTime, 'second')

  const time = () => {
    if (diffInSeconds < 60) {
      return now.to(pastTime, true)
    } else if (diffInSeconds < 3600) {
      return now.to(pastTime, true)
    } else if (diffInSeconds < 86400) {
      return now.to(pastTime, true)
    } else if (diffInSeconds < 2592000) {
      return now.to(pastTime, true)
    } else if (diffInSeconds < 31536000) {
      return now.to(pastTime, true)
    } else {
      return now.to(pastTime, true)
    }
  }

  if (time() === 'a few seconds') {
    return t('fewSeconds')
  }

  const localizedTime = replaceTimeUnits(time()).replace(/s\b/g, '')

  return `${localizedTime}${t('agoSuffix')}`
}
