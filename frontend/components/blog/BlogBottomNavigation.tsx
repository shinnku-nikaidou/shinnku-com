'use client'

import { BlogPostMetadata } from '@/lib/mdx/types'
import { Button } from '@ui/button'
import { ChevronLeft, ChevronRight } from 'lucide-react'

import Link from 'next/link'

interface NavigationProps {
  prev: BlogPostMetadata | null
  next: BlogPostMetadata | null
}

export const BlogBottomNavigation = ({ prev, next }: NavigationProps) => {
  return (
    <div className='mt-8 flex flex-wrap justify-between gap-4 border-t border-default-200 pt-8'>
      {prev ? (
        <Button asChild variant='ghost'>
          <Link className='flex items-center gap-2' href={`/docs/${prev.slug}`}>
            <ChevronLeft className='size-4' />
            {prev.title}
          </Link>
        </Button>
      ) : (
        <div />
      )}
      {next ? (
        <Button asChild variant='ghost'>
          <Link className='flex items-center gap-2' href={`/docs/${next.slug}`}>
            {next.title}
            <ChevronRight className='size-4' />
          </Link>
        </Button>
      ) : (
        <div />
      )}
    </div>
  )
}
