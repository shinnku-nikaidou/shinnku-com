'use client'

import { title as titleStyle } from '@/components/primitives'
import { Card, CardContent } from '@/components/ui/card'
import { IndexList } from '@/config/indexList'
import { t } from '@/i18n'
import Link from 'next/link'

export default function FilesPage() {
  return (
    <div className='container mx-auto mt-10 flex flex-col items-center text-center'>
      <h1 className={titleStyle()}>{t('allGames')}</h1>
      <div className='mt-8 grid grid-cols-2 gap-4 pt-10 sm:grid-cols-4'>
        {IndexList.map((item, index) => (
          <Link key={index} className='w-full' href={item.link}>
            <Card className='w-full shadow-xs'>
              <CardContent className='mt-1 flex size-36 flex-col items-center justify-center gap-2 overflow-visible'>
                <span>{item.body}</span>
                <b className='max-w-full overflow-hidden whitespace-nowrap'>
                  {item.title}
                </b>
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  )
}
