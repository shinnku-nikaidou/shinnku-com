export type FileInfo = {
  file_path: string
  upload_timestamp: number
  file_size: number
}

export type FileOrFolder =
  | { type: 'file'; name: string; info: FileInfo }
  | { type: 'folder'; name: string }

export type GameType = '熟肉' | '生肉' | '手机'

export type WikipediaAnswer = {
  title: string
  text: string
  bg?: string
}
