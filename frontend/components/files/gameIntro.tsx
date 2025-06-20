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
import {
  generate_download_url,
  get_game_type,
  trim_file_path,
} from '@/algorithm/url'
import { t } from '@/i18n'

export const GameIntro: React.FC<{ info: FileInfo }> = ({ info }) => {
  const s = info.file_path.split('/')
  const name = s[s.length - 1]
  const download_url = generate_download_url(s)
  const accelerate_dl = (() => {
    if (download_url.startsWith('https://dl.oo0o.ooo/file/shinnku/')) {
      return download_url.replace(
        'https://dl.oo0o.ooo/',
        'https://download.shinnku.com/',
      )
    } else return null
  })()

  return (
    <Card>
      <CardHeader className='flex gap-3'>
        <div className='flex items-baseline gap-2'>
          <span className='text-lg'>{name}</span>
        </div>
      </CardHeader>
      <Divider />
      <CardBody>
        <p>
          {t('path')}
          {trim_file_path(info.file_path)}
        </p>
        <p className='text-gray-300'>
          <span className='pr-2 text-sm'>{get_game_type(info.file_path)}</span>
          {t('size')}
          {num2size(info.file_size)}
        </p>
      </CardBody>
      <Divider />
      <CardFooter>
        <Link isExternal showAnchorIcon href={download_url}>
          {t('clickToDownload')}
        </Link>
        {accelerate_dl && (
          <Link
            isExternal
            showAnchorIcon
            className='ml-auto'
            href={accelerate_dl}
          >
            {t('fastDownload')}
          </Link>
        )}
      </CardFooter>
    </Card>
  )
}
