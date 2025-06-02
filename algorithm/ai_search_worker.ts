import { parentPort } from 'worker_threads'
import Fuse, { IFuseOptions } from 'fuse.js'
import * as OpenCC from 'opencc-js'

import { search_index } from '@/config/root'
import { SearchItem } from '@/types'

const cn2jp = OpenCC.Converter({ from: 'cn', to: 'jp' })

const options: IFuseOptions<SearchItem> = {
  includeScore: true,
  ignoreLocation: true,
  ignoreFieldNorm: true,
  threshold: 0.78,
  keys: ['id'],
}

async function runSearch(q: string, n: number): Promise<SearchItem[]> {
  const queryjp = cn2jp(q)
  const queryai = await fetch(
    `http://localhost:2998/findname?name=${encodeURIComponent(q)}`,
  )
    .then((res) => res.json())
    .then((data) => data.ans[0] || '')
    .catch(() => '')

  console.log(`search: ${q}, AI answer: ${queryai}`)
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

parentPort?.on('message', async ({ q, n }: { q: string; n: number }) => {
  const res = await runSearch(q, n)
  parentPort?.postMessage(res)
})
