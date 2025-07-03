import type { Metadata } from 'next'

import { blogMetadata } from './metadata'

import { BlogCard } from '@/components/blog/BlogCard'
import { BlogMasonryGrid } from '@/components/blog/BlogMasonryGrid'
import { getAllPosts } from '@/lib/mdx/getPosts'

export const metadata: Metadata = blogMetadata

export default function BlogListPage() {
  const posts = getAllPosts()

  return (
    <div className='w-full px-6 pb-6'>
      <div className='grid gap-4'>
        <BlogMasonryGrid columnWidth={256} gap={24}>
          {posts.map((post) => (
            <BlogCard key={post.slug} post={post} />
          ))}
        </BlogMasonryGrid>
      </div>
    </div>
  )
}
