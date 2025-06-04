'use client'

import { Button } from '@heroui/react'
import { ChevronLeft, ChevronRight } from 'lucide-react'
import Link from 'next/link'

import { KunPostMetadata } from '@/lib/mdx/types'

interface NavigationProps {
  prev: KunPostMetadata | null
  next: KunPostMetadata | null
}

export const KunBottomNavigation = ({ prev, next }: NavigationProps) => {
  return (
    <div className='mt-8 flex flex-wrap justify-between gap-4 border-t border-default-200 pt-8'>
      {prev ? (
        <Button
          as={Link}
          href={`/docs/${prev.slug}`}
          startContent={<ChevronLeft className='size-4' />}
          variant='light'
        >
          {prev.title}
        </Button>
      ) : (
        <div />
      )}
      {next ? (
        <Button
          as={Link}
          endContent={<ChevronRight className='size-4' />}
          href={`/docs/${next.slug}`}
          variant='light'
        >
          {next.title}
        </Button>
      ) : (
        <div />
      )}
    </div>
  )
}
