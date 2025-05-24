'use client'

import { useEffect, useState } from 'react'
import ReactMarkdown from 'react-markdown'
import { ScrollShadow } from '@heroui/react'
import remarkBreaks from 'remark-breaks'

import { WikipediaAnswer } from '@/types/wiki'
import { subtitle, title } from '@/components/primitives'
import { trim_wikipedia_ans, wikipediaToMarkdown } from '@/algorithm/url'

interface SearchIntroProps {
  name: string
}

export const SearchIntro: React.FC<SearchIntroProps> = ({ name }) => {
  const [intro, setIntro] = useState<WikipediaAnswer>({
    title: name,
    text: '',
  })
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    if (intro.bg) {
      const boxMain = document.getElementById('box-main')!
      console.log(`set bg image https://www.shinnku.com/image/${intro.bg}`)
      boxMain.style.backgroundImage = `url('https://www.shinnku.com/image/${intro.bg}')`
    }
  }, [intro.bg])

  useEffect(() => {
    if (loading) {
      setLoading(false)
      fetch(`/api/aiintro?name=${encodeURIComponent(name)}`)
        .then(async (res) => res.json())
        .then((data) => setIntro(data))
    }
  })

  return (
    <ScrollShadow
      hideScrollBar
      className='inline-block h-[800px] max-w-xl justify-center text-center'
      size={100}
    >
      <div className={title({ color: 'violet' })}>{intro.title}</div>
      <div className={subtitle()}>简介来自gemini 2.5 pro的支持</div>
      <div className='prose dark:prose-invert'>
        <ReactMarkdown remarkPlugins={[remarkBreaks]}>
          {wikipediaToMarkdown(trim_wikipedia_ans(intro.text))}
        </ReactMarkdown>
      </div>
    </ScrollShadow>
  )
}
