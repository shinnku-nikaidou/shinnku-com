'use client'

import { title as titleStyle } from '@/components/primitives'
import { Card, CardContent } from '@/components/ui/card'
import { IndexList } from '@/config/indexList'
import { t } from '@/i18n'
import Link from 'next/link'

export default function FilesPage() {
  return (
    <div className='container mx-auto flex flex-col items-center text-center'>
      <h1 className={titleStyle()}>{t('allGames')}</h1>
      <div className='mt-8 grid grid-cols-2 gap-4 pt-10 sm:grid-cols-4'>
        {IndexList.map((item, index) => (
          <Link key={index} className='w-full' href={item.link}>
            <Card className='w-full shadow-sm'>
              <CardContent className='flex flex-col size-36 items-center justify-center gap-2 overflow-visible'>
                <span>{item.body}</span>
                <b className='whitespace-nowrap overflow-hidden max-w-full'>
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
