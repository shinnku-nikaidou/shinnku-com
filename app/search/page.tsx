import { notFound } from 'next/navigation'

import Search from '@/components/search/search'
import { SearchAnswer } from '@/components/search/search-answer'
import { ai_search } from '@/algorithm/search'
import { SearchIntro } from '@/components/search/search-intro'

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
        <div className='max-w-[720px] pl-10'>
          <Search initialSearchTerm={q} />
        </div>
        <div className='grid grid-cols-1 pl-10 pt-10 md:grid-cols-2'>
          <div className='pr-20'>
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
