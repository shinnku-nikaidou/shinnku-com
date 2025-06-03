import { MDXRemoteProps, compileMDX } from 'next-mdx-remote/rsc'
import rehypeKatex from 'rehype-katex'
import remarkMath from 'remark-math'

import { KunLink } from './element/KunLink'
import { KunTable } from './element/KunTable'
import { KunCode } from './element/KunCode'
import { createKunHeading } from './element/kunHeading'

const components = {
  h1: createKunHeading(1),
  h2: createKunHeading(2),
  h3: createKunHeading(3),
  h4: createKunHeading(4),
  h5: createKunHeading(5),
  h6: createKunHeading(6),
  a: KunLink,
  code: KunCode,
  Table: KunTable,
}

export const CustomMDX = async (props: MDXRemoteProps) => {
  const { content } = await compileMDX({
    source: props.source,
    options: {
      mdxOptions: {
        rehypePlugins: [[rehypeKatex, { output: 'mathml' }], remarkMath],
      },
    },
    components: { ...components, ...(props.components || {}) } as any,
  })

  return content
}
