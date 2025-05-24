import { NextResponse, NextRequest } from 'next/server'
import { wikisearchpicture } from '@/algorithm/wiki'

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url)
  const name = searchParams.get('name') as string
  const bg = await wikisearchpicture(name)
  const intro = await fetch(`http://localhost:2998/intro?name=${name}`)
  if (intro.ok) {
    const text = await intro.text()
    console.log(text)
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
