import { notFound } from 'next/navigation'

import Search from '@/components/search'
import { SearchAnswer } from '@/components/search/answer'
import { ai_search } from '@/algorithm/search'
import { SearchIntro } from '@/components/search/intro'

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
        <div className='grid grid-cols-1 gap-4 px-4 pt-10 md:grid-cols-2 md:px-10'>
          <div className='md:pr-20'>
            <SearchAnswer answer={answer} />
          </div>
          <div>
            <SearchIntro name={q} />
          </div>
        </div>
      </div>
    )
  } else {
    notFound()
  }
}
