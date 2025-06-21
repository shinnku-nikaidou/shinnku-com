import type { Metadata } from 'next'

import { generateBlogMetadataTemplate } from './metadata'

import { BlogHeader } from '@/components/blog/BlogHeader'
import { CustomMDX } from '@/components/blog/CustomMDX'
import { BlogBottomNavigation } from '@/components/blog/Navigation'
import { TableOfContents } from '@/components/blog/TableOfContents'
import {
  getAdjacentPosts,
  getAllPosts,
  getPostBySlug,
} from '@/lib/mdx/getPosts'

interface Props {
  params: Promise<{
    slug: string[]
  }>
}

export const generateStaticParams = async () => {
  const posts = getAllPosts()

  return posts.map((post) => ({
    slug: post.slug.split('/'),
  }))
}

export const generateMetadata = async ({
  params,
}: Props): Promise<Metadata> => {
  const { slug } = await params
  const url = slug.join('/')
  const blog = getPostBySlug(url)

  return generateBlogMetadataTemplate(blog)
}

export default async function BlogPostPage({ params }: Props) {
  const { slug } = await params
  const url = slug.join('/')
  const { content, frontmatter } = getPostBySlug(url)
  const { prev, next } = getAdjacentPosts(url)

  return (
    <div className='flex w-full'>
      <div className='w-full px-6 lg:w-[calc(100%-16rem)]'>
        <BlogHeader frontmatter={frontmatter} />
        <article className='blog-prose'>
          <CustomMDX source={content} />
        </article>
        <BlogBottomNavigation next={next} prev={prev} />
      </div>

      <div>
        <div className='fixed'>
          <TableOfContents />
        </div>
      </div>
    </div>
  )
}
