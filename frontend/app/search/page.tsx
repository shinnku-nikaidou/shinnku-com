import { notFound } from 'next/navigation'

import Search from '@/components/search'
import { SearchAnswer } from '@/components/search/answer'
import { ai_search } from '@/algorithm/search'
import { SearchIntro } from '@/components/search/intro'
import { Separator } from '@/components/ui/separator'

export default async function SearchPage({
  searchParams,
}: {
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {
  const q = (await searchParams).q as string
  const answer = await ai_search(q, 200)

  if (q) {
    return (
      <div>
        <div className='max-w-[720px] px-4 md:px-10'>
          <Search initialSearchTerm={q} />
        </div>
        <div className='grid grid-cols-1 gap-4 px-1 pt-10 md:grid-cols-[2fr_1px_1fr] md:px-10'>
          <div className='md:pr-10'>
            <SearchAnswer answer={answer} />
          </div>
          <Separator className='hidden md:block' orientation='vertical' />
          <div className='md:pl-2'>
            <SearchIntro name={q} />
          </div>
        </div>
      </div>
    )
  } else {
    notFound()
  }
}
