import type { Metadata } from 'next'

import { generateKunMetadataTemplate } from './metadata'

import {
  getAdjacentPosts,
  getAllPosts,
  getPostBySlug,
} from '@/lib/mdx/getPosts'
import { CustomMDX } from '@/lib/mdx/CustomMDX'
import { TableOfContents } from '@/components/docs/TableOfContents'
import { KunBottomNavigation } from '@/components/docs/Navigation'
import { BlogHeader } from '@/components/docs/BlogHeader'

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

  return generateKunMetadataTemplate(blog)
}

export default async function Kun({ params }: Props) {
  const { slug } = await params
  const url = slug.join('/')
  const { content, frontmatter } = getPostBySlug(url)
  const { prev, next } = getAdjacentPosts(url)

  return (
    <div className='flex w-full'>
      <div className='w-full px-6 lg:w-[calc(100%-16rem)]'>
        <BlogHeader frontmatter={frontmatter} />
        <article className='kun-prose'>
          <CustomMDX source={content} />
        </article>
        <KunBottomNavigation next={next} prev={prev} />
      </div>

      <div>
        <div className='fixed'>
          <TableOfContents />
        </div>
      </div>
    </div>
  )
}
