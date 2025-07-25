import type { Blog } from '@/lib/mdx/types'
import type { Metadata } from 'next'

export const generateBlogMetadataTemplate = (blog: Blog): Metadata => {
  const { frontmatter } = blog

  return {
    title: `${frontmatter.title}`,
    description: frontmatter.description,
    openGraph: {
      title: frontmatter.title,
      description: frontmatter.description,
      type: 'article',
      publishedTime: frontmatter.date,
      modifiedTime: frontmatter.date,
      images: [
        {
          url: frontmatter.banner,
          width: 1920,
          height: 1080,
          alt: frontmatter.title,
        },
      ],
    },
    twitter: {
      card: 'summary_large_image',
      title: frontmatter.title,
      description: frontmatter.description,
      images: [frontmatter.banner],
    },
  }
}
