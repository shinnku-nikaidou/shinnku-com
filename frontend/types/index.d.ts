import { FileInfo } from '@/lib/validation'

export type FileInfo = FileInfo
export type FileOrFolder =
  | { type: 'file'; name: string; info: FileInfo }
  | { type: 'folder'; name: string }
export type GameType = '熟肉' | '生肉' | '手机'
export type WikipediaAnswer = {
  title: string
  text: string
  bg?: string
}
