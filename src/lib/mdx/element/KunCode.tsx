import { highlight } from 'sugar-high'
import React, { FC } from 'react'

interface CodeProps extends React.HTMLAttributes<HTMLElement> {
  children: string
}

export const KunCode: FC<CodeProps> = ({ children, ...props }) => {
  const codeHTML = highlight(children)

  return <code dangerouslySetInnerHTML={{ __html: codeHTML }} {...props} />
}
