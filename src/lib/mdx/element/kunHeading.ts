import React, { ReactNode } from 'react'

const slugify = (str: string): string => {
  return str
    .toString()
    .toLowerCase()
    .trim()
    .replace(/\s+/g, '-')
    .replace(/&/g, '-and-')
    .replace(/[^\p{L}\p{N}]+/gu, '')
    .replace(/--+/g, '-')
    .replace(/^-+|-+$/g, '')
}

export const createKunHeading = (level: number) => {
  const Heading = ({ children }: { children: ReactNode }) => {
    const slug = slugify(children?.toString() || '')

    return React.createElement(
      `h${level}`,
      { id: slug },
      [
        React.createElement('a', {
          href: `#${slug}`,
          key: `kun-link-${slug}`,
          className: 'kun-anchor',
          'aria-label': slug,
        }),
      ],
      children,
    )
  }

  Heading.displayName = `KunHeading${level}`

  return Heading
}
