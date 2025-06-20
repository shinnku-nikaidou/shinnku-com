import { NextResponse, NextRequest } from 'next/server'

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url)
  const name = searchParams.get('name') as string
  const serviceUrl = process.env.BACKEND_URL || 'http://localhost:2999'
  const bg = await fetch(`${serviceUrl}/wikisearchpicture?name=${name}`)
    .then((res) => res.json())
    .then((data) => data.bg as string | null)
    .catch(() => null)
  const intro = await fetch(`${serviceUrl}/intro?name=${name}`)

  if (intro.ok) {
    const text = await intro.text()

    if (text.includes('No results found.')) {
      return NextResponse.json({ bg: bg || null })
    } else {
      let title = text.split('\n')[0]

      return NextResponse.json({
        bg: bg || null,
        title: title,
        text: text,
      })
    }
  }

  return NextResponse.json({ bg: bg || null })
}
