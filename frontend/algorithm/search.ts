import { SearchItem } from '@/types'

export async function ai_search(q: string, n: number): Promise<SearchItem[]> {
  const serviceUrl = process.env.BACKEND_URL || 'http://localhost:2999'
  const queryai = await fetch(
    `${serviceUrl}/findname?name=${encodeURIComponent(q)}`,
  )
    .then((res) => res.json())
    .then((data) => data.ans[0] || '')
    .catch(() => '')

  let url = `${serviceUrl}/combinesearch?q1=${encodeURIComponent(queryai)}&q2=${encodeURIComponent(q)}&n=${n}`

  const results: SearchItem[] = await fetch(url)
    .then((res) => res.json())
    .catch(() => [])

  return results
}

export async function default_search(
  q: string,
  n: number,
): Promise<SearchItem[]> {
  const serviceUrl = process.env.BACKEND_URL || 'http://localhost:2999'

  const results: SearchItem[] = await fetch(
    `${serviceUrl}/search?q=${encodeURIComponent(q)}&n=${n}`,
  )
    .then((res) => res.json())
    .catch(() => [])

  return results
}
