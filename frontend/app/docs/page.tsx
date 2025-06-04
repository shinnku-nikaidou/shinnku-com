import type { Metadata } from 'next'

import { kunMetadata } from './metadata'

import { getAllPosts } from '@/lib/mdx/getPosts'
import { KunAboutCard } from '@/components/docs/Card'
import { KunMasonryGrid } from '@/components/MasonryGrid'

export const metadata: Metadata = kunMetadata

export default function Kun() {
  const posts = getAllPosts()

  return (
    <div className='w-full px-6 pb-6'>
      <div className='grid gap-4'>
        <KunMasonryGrid columnWidth={256} gap={24}>
          {posts.map((post) => (
            <KunAboutCard key={post.slug} post={post} />
          ))}
        </KunMasonryGrid>
      </div>
    </div>
  )
}
