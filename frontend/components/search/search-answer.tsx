'use client'

import type { SearchList } from '@/types'

import { ScrollShadow } from '@heroui/react'

import { GameIntro } from '../gameIntro'

interface SearchAnswerProps {
  answer: SearchList
}

export const SearchAnswer: React.FC<SearchAnswerProps> = ({ answer }) => {
  return (
    <ScrollShadow hideScrollBar className='h-[800px]' size={100}>
      <div className='flex flex-col'>
        {answer.map((v, i) => {
          return (
            <div key={v.id} className='p-4'>
              <GameIntro key={i} info={v.info} />
            </div>
          )
        })}
      </div>
    </ScrollShadow>
  )
}
