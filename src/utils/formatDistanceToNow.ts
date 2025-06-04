import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'

dayjs.extend(relativeTime)

const replaceTimeUnits = (input: string) => {
  const replacements: Record<string, string> = {
    an: '1',
    a: '1',
    second: '秒',
    seconds: '秒',
    minute: '分钟',
    minutes: '分钟',
    hour: '小时',
    hours: '小时',
    day: '天',
    days: '天',
    week: '周',
    weeks: '周',
    month: '月',
    months: '月',
    year: '年',
    years: '年',
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
    return '数秒前'
  }

  const localizedTime = replaceTimeUnits(time()).replace(/s\b/g, '')

  return `${localizedTime}前`
}
