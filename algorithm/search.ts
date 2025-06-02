import Fuse, { IFuseOptions } from 'fuse.js'
import * as OpenCC from 'opencc-js'
import { Worker } from 'worker_threads'
import { fileURLToPath } from 'url'
import { dirname, join } from 'path'

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
  return new Promise((resolve, reject) => {
    const __filename = fileURLToPath(import.meta.url)
    const __dirname = dirname(__filename)
    const workerPath = join(__dirname, 'ai_search_worker.ts')

    const worker = new Worker(workerPath, { workerData: { q, n } })

    worker.on('message', (data) => resolve(data as SearchItem[]))
    worker.on('error', reject)
    worker.on('exit', (code) => {
      if (code !== 0) {
        reject(new Error(`Worker stopped with exit code ${code}`))
      }
    })
  })
}

export function default_search(q: string, n: number): SearchItem[] {
  const queryjp = cn2jp(q)
  // const query = removeDuplicateCharacters(q + queryjp)

  const results = runsearch(q + queryjp, search_index).slice(0, n)

  return results
}
