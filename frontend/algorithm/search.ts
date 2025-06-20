import * as OpenCC from 'opencc-js'

import { trim_file_path } from './url'

import { BucketFiles, SearchItem } from '@/types'

// export const cn2tw = OpenCC.Converter({ from: 'cn', to: 'tw' })
// export const tw2cn = OpenCC.Converter({ from: 'tw', to: 'cn' })
export const cn2jp = OpenCC.Converter({ from: 'cn', to: 'jp' })

// function removeDuplicateCharacters(combinedQuery: string): string {
//   return Array.from(new Set(nodejieba.cut(combinedQuery, true))).join('')
// }

export function aggregate_builder(...b: Array<BucketFiles>) {
  return b.flat().map((item) => {
    return {
      id: trim_file_path(item.file_path),
      info: item,
    }
  })
}

export async function ai_search(q: string, n: number): Promise<SearchItem[]> {
  const queryjp = cn2jp(q)
  const serviceUrl = process.env.BACKEND_URL || 'http://localhost:2999'
  const queryai = await fetch(
    `${serviceUrl}/findname?name=${encodeURIComponent(q)}`,
  )
    .then((res) => res.json())
    .then((data) => data.ans[0] || '')
    .catch(() => '')

  const results: SearchItem[] = await fetch(
    `${serviceUrl}/conbinesearch?q1=${encodeURIComponent(
      q + ' ' + queryai,
    )}&q2=${encodeURIComponent(q + ' ' + queryjp)}&n=${n}`,
  )
    .then((res) => res.json())
    .catch(() => [])

  return results
}

export async function default_search(
  q: string,
  n: number,
): Promise<SearchItem[]> {
  const queryjp = cn2jp(q)
  const serviceUrl = process.env.BACKEND_URL || 'http://localhost:2999'

  const results: SearchItem[] = await fetch(
    `${serviceUrl}/search?q=${encodeURIComponent(q + ' ' + queryjp)}&n=${n}`,
  )
    .then((res) => res.json())
    .catch(() => [])

  return results
}
