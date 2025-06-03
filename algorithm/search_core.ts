import Fuse, { IFuseOptions } from 'fuse.js'
import * as OpenCC from 'opencc-js'

import { search_index } from "@/config/root"
import { SearchItem, SearchList } from '@/types'

export const cn2jp = OpenCC.Converter({ from: 'cn', to: 'jp' })

export const fuseOptions: IFuseOptions<SearchItem> = {
  includeScore: true,
  ignoreLocation: true,
  ignoreFieldNorm: true,
  threshold: 0.78,
  keys: ['id'],
}

export function runsearch(query: string, files: SearchList): SearchList {
  const fuse = new Fuse(files, fuseOptions)
  return fuse.search(query).map((result) => result.item)
}

export async function aiSearchCore(
  q: string,
  n: number,
  index: SearchList = search_index,
): Promise<SearchItem[]> {
  const queryjp = cn2jp(q)
  const queryai = await fetch(
    `http://localhost:2998/findname?name=${encodeURIComponent(q)}`,
  )
    .then((res) => res.json())
    .then((data) => data.ans[0] || '')
    .catch(() => '')

  const fuse = new Fuse(index, fuseOptions)
  const aiRes = fuse
    .search(`${q} ${queryai}`)
    .map((result) => ({ item: result.item, score: result.score }))
  const traditionalResults = fuse
    .search(`${q} ${queryjp}`)
    .map((result) => ({ item: result.item, score: result.score }))

  const results: Array<{ item: SearchItem; score: number | undefined }> = []
  for (const res of aiRes) {
    if (res.score) {
      results.push(res)
    }
  }
  for (const res of traditionalResults) {
    if (res.score) {
      const existingIndex = results.findIndex(
        (r) => r.item.id === res.item.id,
      )
      if (existingIndex !== -1) {
        if (results[existingIndex].score && res.score) {
          results[existingIndex].score =
            (results[existingIndex].score ?? 0) / 2 + (res.score ?? 0) / 2
        }
      } else {
        results.push(res)
      }
    }
  }

  return results
    .sort((a, b) => (a.score ?? 0) - (b.score ?? 0))
    .slice(0, n)
    .map((result) => result.item)
}

export function defaultSearchCore(
  q: string,
  n: number,
  index: SearchList = search_index,
): SearchItem[] {
  const queryjp = cn2jp(q)
  return runsearch(`${q}${queryjp}`, index).slice(0, n)
}
