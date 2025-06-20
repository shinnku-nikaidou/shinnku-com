'use client'

import { useEffect, useState } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkBreaks from 'remark-breaks'

import { WikipediaAnswer } from '@/types/wiki'
import { subtitle, title } from '@/components/primitives'
import { trim_wikipedia_ans, wikipediaToMarkdown } from '@/algorithm/url'
import { t } from '@/i18n'

interface SearchIntroProps {
  name: string
}

export const SearchIntro: React.FC<SearchIntroProps> = ({ name }) => {
  const [intro, setIntro] = useState<WikipediaAnswer>({
    title: name,
    text: '',
  })

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

  return (
    <>
      <div className={title({ color: 'violet' })}>{intro.title}</div>
      <div className={subtitle()}>{t('searchIntroFromGemini')}</div>
      <div className='prose dark:prose-invert'>
        <ReactMarkdown remarkPlugins={[remarkBreaks]}>
          {wikipediaToMarkdown(trim_wikipedia_ans(intro.text))}
        </ReactMarkdown>
      </div>
    </>
  )
}
