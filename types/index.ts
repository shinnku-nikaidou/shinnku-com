import { SVGProps } from 'react'

export type IconSvgProps = SVGProps<SVGSVGElement> & {
  size?: number
}

export type FileInfo = {
  file_path: string
  upload_timestamp: number
  file_size: number
}

export type BucketFiles = FileInfo[]

export type SearchList = SearchItem[]

export type SearchItem = {
  id: string
  info: FileInfo
}

export type Node =
  | { type: 'file'; name: string; info: FileInfo }
  | { type: 'folder'; name: string }

export type Inode = Node[]

export type Variety = '404' | 'file' | 'folder'

export type RedisConfig = {
  host: string
  port: number
  password?: string
  database: number
}

export type Config = {
  redis: RedisConfig
}

export type GameType = '熟肉' | '生肉' | '手机'
