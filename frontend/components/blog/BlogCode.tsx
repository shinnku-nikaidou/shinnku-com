import React, { FC } from 'react'
import { highlight } from 'sugar-high'

interface CodeProps extends React.HTMLAttributes<HTMLElement> {
  children: string
}

export const BlogCode: FC<CodeProps> = ({ children, ...props }) => {
  const codeHTML = highlight(children)

  return <code dangerouslySetInnerHTML={{ __html: codeHTML }} {...props} />
}
