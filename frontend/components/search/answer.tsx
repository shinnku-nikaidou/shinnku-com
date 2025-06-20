'use client'

import type { SearchList } from '@/types'

import { ScrollShadow } from '@heroui/react'

import { AnswerItem } from '../answerItem'

interface SearchAnswerProps {
  answer: SearchList
}

export const SearchAnswer: React.FC<SearchAnswerProps> = ({ answer }) => {
  return (
    <ScrollShadow hideScrollBar className='h-[1600px]' size={100}>
      <div className='flex flex-col'>
        {answer.map((v) => (
          <div key={v.id} className='p-2'>
            <AnswerItem info={v.info} />
          </div>
        ))}
      </div>
    </ScrollShadow>
  )
}
