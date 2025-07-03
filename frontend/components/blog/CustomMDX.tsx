import { MDXRemoteProps, compileMDX } from 'next-mdx-remote/rsc'
import rehypeKatex from 'rehype-katex'
import remarkMath from 'remark-math'

import { BlogCode } from './BlogCode'
import { BlogLink } from './BlogLink'
import { BlogTable } from './BlogTable'
import { createBlogHeading } from './createBlogHeading'

const components = {
  h1: createBlogHeading(1),
  h2: createBlogHeading(2),
  h3: createBlogHeading(3),
  h4: createBlogHeading(4),
  h5: createBlogHeading(5),
  h6: createBlogHeading(6),
  a: BlogLink,
  code: BlogCode,
  Table: BlogTable,
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
