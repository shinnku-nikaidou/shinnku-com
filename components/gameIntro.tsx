'use client'

import {
  Card,
  CardHeader,
  CardBody,
  CardFooter,
  Divider,
  Link,
} from '@heroui/react'
import React from 'react'

import { FileInfo } from '@/types'
import { num2size } from '@/algorithm/util'
import { generate_download_url, trim_file_path } from '@/algorithm/url'

export const GameIntro: React.FC<{ info: FileInfo }> = ({ info }) => {
  const s = info.file_path.split('/')
  const name = s[s.length - 1]

  return (
    <Card className=''>
      <CardHeader className='flex gap-3'>
        {/* <Image
          alt='heroui logo'
          height={40}
          radius='sm'
          src='https://avatars.githubusercontent.com/u/86160567?s=200&v=4'
          width={40}
        /> */}
        <div className='flex flex-col'>
          <p className='text-lg'>{name}</p>
        </div>
      </CardHeader>
      <Divider />
      <CardBody>
        <p>路径：{trim_file_path(info.file_path)}</p>
        <p className='text-gray-300'>大小：{num2size(info.file_size)}</p>
      </CardBody>
      <Divider />
      <CardFooter>
        <Link isExternal showAnchorIcon href={generate_download_url(s)}>
          点击此处下载
        </Link>
      </CardFooter>
    </Card>
  )
}
