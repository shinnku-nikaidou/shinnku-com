'use client'

import Link from 'next/link'

import { get_game_type } from '@/algorithm/url'
import { num2size } from '@/algorithm/util'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { FileInfo } from '@/types'

interface AnswerItemProps {
  info: FileInfo
}

export const AnswerItem: React.FC<AnswerItemProps> = ({ info }) => {
  const parts = info.file_path.split('/')
  const fileName = parts[parts.length - 1]

  // Fix the href by adding the appropriate routing prefix
  const prefix = info.file_path.startsWith('合集系列') ? 'galgame0' : 'shinnku'
  const href = `/files/${prefix}/${parts.map(encodeURIComponent).join('/')}`

  return (
    <Card className='transition-shadow hover:shadow-md'>
      <CardHeader className='pb-0'>
        <Link className='text-lg text-blue-600 hover:underline' href={href}>
          {fileName}
        </Link>
        <p className='break-all text-sm text-muted-foreground'>
          文件路径：{info.file_path}
        </p>
      </CardHeader>
      <CardContent className='pt-0 text-sm text-muted-foreground'>
        <span className='pr-2'>{get_game_type(info.file_path)}</span>
        {num2size(info.file_size)}
      </CardContent>
    </Card>
  )
}
