import type { KunFrontmatter } from '@/lib/mdx/types'

import { Card, CardBody, CardHeader } from '@heroui/card'
import { Avatar } from '@heroui/avatar'
import { Image } from '@heroui/image'
import { CalendarDays } from 'lucide-react'

import { formatDate } from '@/utils/time'

interface BlogHeaderProps {
  frontmatter: KunFrontmatter
}

export const BlogHeader = ({ frontmatter }: BlogHeaderProps) => {
  return (
    <Card className='w-full border-none bg-transparent shadow-none'>
      <CardHeader className='flex flex-col items-start px-0 pb-0'>
        <div className='relative w-full overflow-hidden rounded-xl'>
          <Image
            isZoomed
            alt={frontmatter.title}
            className='object-cover'
            height='100%'
            src={frontmatter.banner}
            width='100%'
          />
        </div>
      </CardHeader>

      <CardBody>
        <div className='flex flex-col space-y-4'>
          <h1 className='text-3xl font-bold tracking-tight sm:text-4xl'>
            {frontmatter.title}
          </h1>

          <div className='flex items-center gap-3'>
            <Avatar
              className='shrink-0'
              isBordered={true}
              name={frontmatter.authorName}
              radius='full'
              size='md'
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
      </CardBody>
    </Card>
  )
}
