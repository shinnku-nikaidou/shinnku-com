import { NextRequest, NextResponse } from 'next/server'

import { Lang, wikifullsearch } from '@/lib/wiki'
import { WikipediaAnswer } from '@/types'

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url)
  const name = searchParams.get('name') as string
  const lang = searchParams.get('lang') as Lang
  let ans: WikipediaAnswer

  if (lang) {
    ans = await wikifullsearch(name.substring(0, 40), lang)
  } else {
    ans = await wikifullsearch(name.substring(0, 40))
  }

  return NextResponse.json(ans)
}
