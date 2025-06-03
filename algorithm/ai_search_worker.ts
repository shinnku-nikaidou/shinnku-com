import { parentPort } from 'node:worker_threads'
import { search_index } from '../config/root'
import { aiSearchCore } from './search_core'

parentPort?.on('message', async ({ q, n }: { q: string; n: number }) => {
  const res = await aiSearchCore(q, n, search_index)
  parentPort?.postMessage(res)
})
