import Redis from 'ioredis'

import { config } from '@/config/root'
import { WikipediaAnswer } from '@/types/wiki'

export type Lang = 'ja' | 'zh' | 'en'

const redisClient = new Redis({
  host: config.redis.host,
  port: config.redis.port,
  password: config.redis.password,
  db: config.redis.database,
})

const emptyanswer = {
  title: '',
  text: '',
}

function assertLang(lang: string): asserts lang is Lang {
  const validLangs: Lang[] = ['ja', 'zh', 'en']

  if (!validLangs.includes(lang as Lang)) {
    throw new Error(
      `Invalid Lang type: ${lang}. Must be one of ${validLangs.join(', ')}`,
    )
  }
}

export async function wikifullsearch(
  query: string,
  lang: Lang = 'zh',
): Promise<WikipediaAnswer> {
  const queurl = `https://${lang}.wikipedia.org/w/api.php?action=query&format=json&list=search&srsearch=${query}&srlimit=1`

  const res = await (await fetch(queurl)).json()

  try {
    assertLang(lang)
    const pageid: number = res['query']['search'][0]['pageid']

    const quer = `https://${lang}.wikipedia.org/w/api.php?action=query&format=json&pageids=${pageid}&prop=info|categories|langlinks|extracts&explaintext=&`
    const ans = await (await fetch(quer)).json()

    const context = ans['query']['pages'][`${pageid}`]

    return {
      title: context['title'],
      text: context['extract'],
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (e) {
    return emptyanswer
  }
}

export async function wikisearchpicture(query: string): Promise<string | null> {
  let pageid: number
  const ans = await redisClient.get(`cache:search:wiki:zh:${query}`)

  if (ans) {
    pageid = parseInt(ans, 10)
    const bg = await redisClient.get(`img:wiki:zh:${pageid}`)

    if (bg) {
      return bg
    }
  }

  return null
}

export async function wikiredissearch(
  query: string,
  lang: Lang = 'zh',
): Promise<WikipediaAnswer> {
  let pageid: number

  try {
    assertLang(lang)
    const ans = await redisClient.get(`cache:search:wiki:${lang}:${query}`)

    if (ans) {
      pageid = parseInt(ans, 10)
    } else {
      const queurl = `https://${lang}.wikipedia.org/w/api.php?action=query&format=json&list=search&srsearch=${query}&srlimit=1`
      const res = await (await fetch(queurl)).json()

      pageid = res['query']['search'][0]['pageid']
    }
  } catch (_e) {
    return emptyanswer
  }

  try {
    const ans = await redisClient.hmget(
      `wikipedia:${lang}:${pageid}`,
      'title',
      'text',
    )

    if (ans[0] && ans[1]) {
      const bg = await redisClient.get(`img:wiki:${lang}:${pageid}`)

      if (bg) {
        return {
          title: ans[0],
          text: ans[1],
          bg: bg,
        }
      } else
        return {
          title: ans[0],
          text: ans[1],
        }
    } else if (lang == 'ja') {
      return emptyanswer
    } else {
      return wikiredissearch(query, 'ja')
    }
  } catch (_e) {
    return emptyanswer
  }
}
