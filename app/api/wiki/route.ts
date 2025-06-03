import { NextResponse, NextRequest } from 'next/server'

import { Lang, wikiredissearch } from '@/algorithm/wiki'
import { WikipediaAnswer } from '@/types/wiki'

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url)
  const name = searchParams.get('name') as string
  const lang = searchParams.get('lang') as Lang
  let ans: WikipediaAnswer

  if (lang) {
    ans = await wikiredissearch(name.substring(0, 40), lang)
  } else {
    ans = await wikiredissearch(name.substring(0, 40))
  }

  return NextResponse.json(ans)
}
