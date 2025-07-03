'use client'

import { ScrollArea } from '@/components/ui/scroll-area'
import { FileInfo } from '@/types'
import { AnswerItem } from './AnswerItem'

export type SearchList = {
  id: string
  info: FileInfo
}[]

interface SearchAnswerProps {
  answer: SearchList
}

export const SearchAnswer: React.FC<SearchAnswerProps> = ({ answer }) => {
  return (
    <ScrollArea className='h-[1600px]'>
      <div className='flex flex-col'>
        {answer.map((v) => (
          <div key={v.id} className='p-2'>
            <AnswerItem info={v.info} />
          </div>
        ))}
      </div>
    </ScrollArea>
  )
}
