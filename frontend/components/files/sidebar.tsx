'use client'

import { Button } from '@heroui/button'
import { Link } from '@heroui/link'
import { usePathname } from 'next/navigation'

import { IndexListForSlog } from '@/config/indexList'

export const Sidebar = () => {
  const pathname = usePathname()

  return (
    <div className='flex flex-col gap-4'>
      {IndexListForSlog.map((item, index) => (
        <div key={index}>
          <Button
            isIconOnly
            as={Link}
            className='flex md:hidden'
            color={pathname === item.link ? 'primary' : 'default'}
            href={item.link}
            variant={pathname === item.link ? 'solid' : 'bordered'}
          >
            {item.body}
          </Button>
          <Button
            fullWidth
            as={Link}
            className='hidden justify-start md:flex'
            color={pathname === item.link ? 'primary' : 'default'}
            href={item.link}
            variant={pathname === item.link ? 'solid' : 'bordered'}
          >
            {item.body}
            <span>{item.title}</span>
          </Button>
        </div>
      ))}
    </div>
  )
}
