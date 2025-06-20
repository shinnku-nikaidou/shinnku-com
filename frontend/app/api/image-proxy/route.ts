import { NextRequest } from 'next/server'

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url)
  const path = searchParams.get('path')

  if (!path) {
    return new Response('Missing path', { status: 400 })
  }

  const targetUrl = `https://www.shinnku.com/image/${encodeURIComponent(path)}`

  const resp = await fetch(targetUrl, {
    headers: {
      Referer: 'https://www.shinnku.com',
      Origin: 'https://www.shinnku.com',
    },
  })

  if (!resp.ok) {
    return new Response('Failed to fetch image', { status: resp.status })
  }

  const headers = new Headers(resp.headers)
  const contentType = resp.headers.get('content-type')

  if (contentType) {
    headers.set('content-type', contentType)
  }

  return new Response(resp.body, {
    status: resp.status,
    headers,
  })
}
