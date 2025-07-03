'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'

import { IndexListForSlog } from '@/config/indexList'
import { Button } from '@ui/button'

export const Sidebar = () => {
  const pathname = usePathname()

  return (
    <div className='flex flex-col gap-4'>
      {IndexListForSlog.map((item, index) => (
        <div key={index}>
          <Button
            asChild
            className='flex md:hidden'
            size='icon'
            variant={pathname === item.link ? 'default' : 'outline'}
          >
            <Link href={item.link}>{item.body}</Link>
          </Button>
          <Button
            asChild
            className='hidden w-full justify-start md:flex'
            variant={pathname === item.link ? 'default' : 'outline'}
          >
            <Link className='flex items-center gap-2' href={item.link}>
              {item.body}
              <span>{item.title}</span>
            </Link>
          </Button>
        </div>
      ))}
    </div>
  )
}
