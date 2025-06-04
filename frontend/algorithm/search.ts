import Fuse, { IFuseOptions } from 'fuse.js'
import * as OpenCC from 'opencc-js'

import { trim_file_path } from './url'

import { BucketFiles, SearchItem, SearchList } from '@/types'
import { search_index } from '@/config/root'

// export const cn2tw = OpenCC.Converter({ from: 'cn', to: 'tw' })
// export const tw2cn = OpenCC.Converter({ from: 'tw', to: 'cn' })
export const cn2jp = OpenCC.Converter({ from: 'cn', to: 'jp' })

const options: IFuseOptions<SearchItem> = {
  includeScore: true,
  ignoreLocation: true,
  ignoreFieldNorm: true,
  threshold: 0.78,
  keys: ['id'],
}

export function runsearch(query: string, files: SearchList): SearchList {
  const fuse = new Fuse(files, options)
  const tmp = fuse.search(query)

  return tmp.map((result) => result.item)
}

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
  const serviceUrl = process.env.AI_SERVICE_URL || 'http://localhost:2999'
  const queryai = await fetch(
    `${serviceUrl}/findname?name=${encodeURIComponent(q)}`,
  )
    .then((res) => res.json())
    .then((data) => data.ans[0] || '')
    .catch((_error) => {
      return ''
    })

  const fuse = new Fuse(search_index, options)
  const ai_res = fuse
    .search(q + ' ' + queryai)
    .map((result) => ({ item: result.item, score: result.score }))
  const traditional_results = fuse
    .search(q + ' ' + queryjp)
    .map((result) => ({ item: result.item, score: result.score }))

  const results: Array<{ item: SearchItem; score: number | undefined }> = []

  for (const res of ai_res) {
    if (res.score) {
      results.push(res)
    }
  }
  for (const res of traditional_results) {
    if (res.score) {
      const existing_result_index = results.findIndex(
        (r) => r.item.id === res.item.id,
      )

      if (existing_result_index !== -1) {
        if (results[existing_result_index].score && res.score) {
          results[existing_result_index].score =
            (results[existing_result_index].score ?? 0) / 2 +
            (res.score ?? 0) / 2
        }
      } else {
        results.push(res)
      }
    }
  }

  return results
    .sort((a, b) => {
      return (a.score ?? 0) - (b.score ?? 0)
    })
    .slice(0, n)
    .map((result) => result.item)
}

export function default_search(q: string, n: number): SearchItem[] {
  const queryjp = cn2jp(q)
  // const query = removeDuplicateCharacters(q + queryjp)

  const results = runsearch(q + queryjp, search_index).slice(0, n)

  return results
}
