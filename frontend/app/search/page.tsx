import { notFound } from 'next/navigation'

import Search from '@/components/search'
import { SearchAnswer } from '@/components/search/SearchAnswer'
import { SearchIntro } from '@/components/search/SearchIntro'
import { Separator } from '@/components/ui/separator'
import { ai_search } from '@/lib/search'

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
        <div className='flex flex-col gap-4 px-1 pt-10 md:px-10'>
          <div className='md:hidden'>
            <SearchIntro name={q} />
          </div>
          <div className='grid grid-cols-1 gap-4 md:grid-cols-[2fr_1px_1fr]'>
            <div className='md:pr-6'>
              <SearchAnswer answer={answer} />
            </div>
            <Separator className='hidden md:block' orientation='vertical' />
            <div className='hidden md:block md:pl-2'>
              <SearchIntro name={q} />
            </div>
          </div>
        </div>
      </div>
    )
  } else {
    notFound()
  }
}
