'use client'

import Link from 'next/link'
import { Card, CardBody } from '@heroui/react'

import { title } from '@/components/primitives'
import { IndexList } from '@/config/indexList'

export default function FilesPage() {
  return (
    <div className='items-center text-center'>
      <h1 className={title()}>全部游戏</h1>
      <div className='mt-8 grid grid-cols-2 gap-4 pt-10 sm:grid-cols-4'>
        {IndexList.map((item, index) => (
          <Card
            key={index}
            isPressable
            as={Link}
            className='w-full'
            href={item.link}
            shadow='sm'
          >
            <CardBody className='flex size-36 items-center justify-center gap-2 overflow-visible'>
              <span>{item.body}</span>
              <b>{item.title}</b>
            </CardBody>
          </Card>
        ))}
      </div>
    </div>
  )
}
