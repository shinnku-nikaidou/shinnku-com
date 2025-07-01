'use client'

import {
  generate_download_url,
  get_game_type,
  trim_file_path,
} from '@/algorithm/url'
import { num2size } from '@/algorithm/util'
import { t } from '@/i18n'
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
          className='flex items-center gap-1 text-primary hover:underline'
          href={download_url}
          rel='noopener noreferrer'
          target='_blank'
        >
          {t('clickToDownload')}
          <ExternalLink className='size-4' />
        </Link>
        {accelerate_dl && (
          <Link
            className='ml-auto flex items-center gap-1 text-primary hover:underline'
            href={accelerate_dl}
            rel='noopener noreferrer'
            target='_blank'
          >
            {t('fastDownload')}
            <ExternalLink className='size-4' />
          </Link>
        )}
      </CardFooter>
    </Card>
  )
}
