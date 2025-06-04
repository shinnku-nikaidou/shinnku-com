'use client'

import { useEffect, useState } from 'react'

interface TOCItem {
  id: string
  text: string
  level: number
}

const scrollToHeading = (id: string) => {
  const headingElement = document.getElementById(id)

  if (headingElement) {
    headingElement.scrollIntoView({
      behavior: 'smooth',
      block: 'center',
    })
  }
}

export const TableOfContents = () => {
  const [headings, setHeadings] = useState<TOCItem[]>([])
  const [activeId, setActiveId] = useState('')

  useEffect(() => {
    const elements = Array.from(
      document.querySelectorAll('article h1, article h2, article h3'),
    ).map((element) => ({
      id: element.id,
      text: element.textContent || '',
      level: Number(element.tagName.charAt(1)),
    }))

    setHeadings(elements)

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setActiveId(entry.target.id)
          }
        })
      },
      { rootMargin: '0px 0px -80% 0px' },
    )

    document
      .querySelectorAll('article h1, article h2, article h3')
      .forEach((heading) => {
        observer.observe(heading)
      })

    return () => observer.disconnect()
  }, [])

  return (
    <nav className='hidden w-64 lg:block'>
      <h2 className='mb-4 text-lg font-semibold'>本页面索引</h2>
      <ul className='space-y-2'>
        {headings.map((heading) => (
          <li
            key={heading.id}
            style={{ paddingLeft: `${(heading.level - 1) * 1}rem` }}
          >
            <a
              className={`block py-1 text-sm hover:text-primary-500 ${
                activeId === heading.id
                  ? 'font-medium text-primary-500'
                  : 'text-default-600 dark:text-default-400'
              }`}
              href={`#${heading.id}`}
              onClick={(e) => {
                e.preventDefault()
                scrollToHeading(heading.id)
              }}
            >
              {heading.text}
            </a>
          </li>
        ))}
      </ul>
    </nav>
  )
}
