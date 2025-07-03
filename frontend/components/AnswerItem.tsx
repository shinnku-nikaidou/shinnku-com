'use client'

import Link from 'next/link'

import { get_game_type } from '@/lib/url'
import { num2size } from '@/lib/utils'
import { FileInfo } from '@/types'
import { Card, CardContent, CardHeader } from '@ui/card'

interface AnswerItemProps {
  info: FileInfo
}

export const AnswerItem: React.FC<AnswerItemProps> = ({ info }) => {
  let parts = info.file_path.split('/')
  const fileName = parts[parts.length - 1]
  // Fix the href by adding the appropriate routing prefix
  const prefix = info.file_path.startsWith('合集系列') ? 'galgame0' : 'shinnku'
  if (info.file_path.startsWith('合集系列/')) {
    parts = parts.slice(2) // Remove the first part if it starts with '合集系列/浮士德galgame游戏合集/'
  }
  const href = `/files/${prefix}/${parts.map(encodeURIComponent).join('/')}`

  return (
    <Card className='transition-shadow hover:shadow-md'>
      <CardHeader className='pb-0'>
        <Link className='text-lg text-blue-600 hover:underline' href={href}>
          {fileName}
        </Link>
        <p className='text-muted-foreground text-sm break-all'>
          文件路径：{info.file_path}
        </p>
      </CardHeader>
      <CardContent className='text-muted-foreground pt-0 text-sm'>
        <span className='pr-2'>{get_game_type(info.file_path)}</span>
        {num2size(info.file_size)}
      </CardContent>
    </Card>
  )
}
