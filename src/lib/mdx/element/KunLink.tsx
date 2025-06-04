import Link from 'next/link'
import React, { FC } from 'react'

interface CustomLinkProps
  extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  href: string
}

export const KunLink: FC<CustomLinkProps> = ({ href, children, ...props }) => {
  if (href.startsWith('/')) {
    return (
      <Link href={href} {...props}>
        {children}
      </Link>
    )
  }

  if (href.startsWith('#')) {
    return (
      <a href={href} {...props}>
        {children}
      </a>
    )
  }

  return (
    <a href={href} rel='noopener noreferrer' target='_blank' {...props}>
      {children}
    </a>
  )
}
