'use client'

import { useEffect, useState } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkBreaks from 'remark-breaks'

import { title } from '@/components/primitives'
import { trim_wikipedia_ans, wikipediaToMarkdown } from '@/lib/url'
import { cn } from '@/lib/utils'
import { WikipediaAnswer } from '@/types'

interface SearchIntroProps {
  name: string
}

export const SearchIntro: React.FC<SearchIntroProps> = ({ name }) => {
  const [intro, setIntro] = useState<WikipediaAnswer>({
    title: name,
    text: '',
  })
  const [expanded, setExpanded] = useState(false)

  useEffect(() => {
    if (intro.bg) {
      const boxMain = document.getElementById('box-main')!

      boxMain.style.backgroundImage = `url('/api/image-proxy?path=${intro.bg}')`
    }
  }, [intro.bg])

  useEffect(() => {
    fetch(`/api/aiintro?name=${encodeURIComponent(name)}`)
      .then(async (res) => res.json())
      .then((data) => setIntro(data))
  }, [name])

  const toggleExpand = () => setExpanded((v) => !v)

  return (
    <>
      <div className={title({ color: 'pink', size: 'sm' })}>{intro.title}</div>
      <div
        className={cn(
          'prose prose-sm overflow-hidden transition-[max-height] duration-300 dark:prose-invert md:max-h-none',
          expanded ? 'max-h-none' : 'max-h-[30vh]',
        )}
      >
        <ReactMarkdown remarkPlugins={[remarkBreaks]}>
          {wikipediaToMarkdown(trim_wikipedia_ans(intro.text))}
        </ReactMarkdown>
      </div>
      <button
        className='mt-2 text-sm text-blue-500 md:hidden'
        onClick={toggleExpand}
      >
        {expanded ? '收起' : '展开'}
      </button>
    </>
  )
}
