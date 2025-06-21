import type { BlogFrontmatter } from '@/lib/mdx/types'

import { Avatar } from '@/components/ui/avatar'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { CalendarDays } from 'lucide-react'

import Image from 'next/image'

import { formatDate } from '@/utils/time'

interface BlogHeaderProps {
  frontmatter: BlogFrontmatter
}

export const BlogHeader = ({ frontmatter }: BlogHeaderProps) => {
  return (
    <Card className='w-full border-none bg-transparent shadow-none'>
      <CardHeader className='flex flex-col items-start px-0 pb-0'>
        <div className='relative w-full overflow-hidden rounded-xl'>
          <Image
            alt={frontmatter.title}
            className='size-full object-cover'
            height={540}
            src={frontmatter.banner}
            width={960}
          />
        </div>
      </CardHeader>

      <CardContent>
        <div className='flex flex-col space-y-4'>
          <h1 className='text-3xl font-bold tracking-tight sm:text-4xl'>
            {frontmatter.title}
          </h1>

          <div className='flex items-center gap-3'>
            <Avatar
              isBordered
              alt={frontmatter.authorName}
              className='shrink-0'
              src={frontmatter.authorAvatar}
            />
            <div className='flex flex-col gap-1'>
              <h2 className='text-small font-semibold leading-none'>
                {frontmatter.authorName}
              </h2>
              <div className='flex items-center gap-2'>
                <CalendarDays className='h-4 w-4 text-default-400' />
                <p className='text-small text-default-400'>
                  {formatDate(frontmatter.date, {
                    isPrecise: true,
                    isShowYear: true,
                  })}
                </p>
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
