'use client'

import { t } from '@/i18n'
import { generate_download_url, get_game_type, trim_file_path } from '@/lib/url'
import { num2size } from '@/lib/utils'
import { FileInfo } from '@/types'
import { Card, CardContent, CardFooter, CardHeader } from '@ui/card'
import { Separator } from '@ui/separator'
import { ExternalLink } from 'lucide-react'

import Link from 'next/link'
import React from 'react'

export const GameIntro: React.FC<{ info: FileInfo }> = ({ info }) => {
  const s = info.file_path.split('/')
  const name = s[s.length - 1]
  const download_url = generate_download_url(s)
  // const accelerate_dl = (() => {
  //   if (download_url.startsWith('https://zd.shinnku.top/file/shinnku/')) {
  //     return download_url.replace(
  //       'https://zd.shinnku.top/',
  //       'https://download.shinnku.com/',
  //     )
  //   } else return null
  // })()

  return (
    <Card>
      <CardHeader className='flex gap-3'>
        <div className='flex items-baseline gap-2'>
          <span className='text-lg'>{name}</span>
        </div>
      </CardHeader>
      <Separator />
      <CardContent>
        <p>
          {t('path')}
          {trim_file_path(info.file_path)}
        </p>
        <p className='text-gray-300'>
          <span className='pr-2 text-sm'>{get_game_type(info.file_path)}</span>
          {t('size')}
          {num2size(info.file_size)}
        </p>
      </CardContent>
      <Separator />
      <CardFooter className='flex gap-2'>
        <Link
          className='text-primary flex items-center gap-1 hover:underline'
          href={download_url}
          rel='noopener noreferrer'
          target='_blank'
        >
          {t('clickToDownload')}
          <ExternalLink className='size-4' />
        </Link>
        {/* {accelerate_dl && (
          <Link
            className='text-primary ml-auto flex items-center gap-1 hover:underline'
            href={accelerate_dl}
            rel='noopener noreferrer'
            target='_blank'
          >
            {t('fastDownload')}
            <ExternalLink className='size-4' />
          </Link>
        )} */}
      </CardFooter>
    </Card>
  )
}
