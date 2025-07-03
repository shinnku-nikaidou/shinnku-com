'use client'

import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { t } from '@/i18n'
import { BlogPostMetadata } from '@/lib/mdx/types'
import { formatDistanceToNow } from '@/lib/utils/formatDistanceToNow'
import { Calendar as CalendarIcon, Type as TypeIcon } from 'lucide-react'

import Image from 'next/image'
import Link from 'next/link'
import { useState } from 'react'

interface Props {
  post: BlogPostMetadata
}

export const BlogCard = ({ post }: Props) => {
  const [imageLoaded, setImageLoaded] = useState(false)

  return (
    <Link
      className='block w-full transition-transform duration-200 hover:scale-[1.02]'
      href={`/docs/${post.slug}`}
    >
      <Card>
        <CardContent className='space-y-3 p-4'>
          <h2 className='mb-2 text-xl font-bold'>{post.title}</h2>
          <div className='relative mx-auto w-full overflow-hidden rounded-t-lg text-center opacity-90'>
            <div
              className={`bg-default-100 absolute inset-0 animate-pulse ${
                imageLoaded ? 'opacity-0' : 'opacity-90'
              } transition-opacity duration-300`}
              style={{ aspectRatio: '16/9' }}
            />
            <Image
              alt={post.title}
              className={`size-full object-cover transition-all duration-300 ${
                imageLoaded ? 'scale-100 opacity-90' : 'scale-105 opacity-0'
              }`}
              height={540}
              src={post.banner}
              style={{ aspectRatio: '16/9' }}
              width={960}
              onLoad={() => setImageLoaded(true)}
            />
          </div>
          <div className='text-default-500 flex items-center gap-4 text-sm'>
            <div className='flex items-center gap-1'>
              <CalendarIcon size={16} />
              <time>{formatDistanceToNow(post.date)}</time>
            </div>
            <div className='flex items-center gap-1'>
              <TypeIcon size={16} />
              <span>
                {post.textCount} {t('characters')}
              </span>
            </div>
          </div>
        </CardContent>
        <CardFooter className='border-default-200 bg-default-50 border-t px-5 py-3'>
          <span className='text-default-600 text-sm'>{t('readMore')}</span>
        </CardFooter>
      </Card>
    </Link>
  )
}
