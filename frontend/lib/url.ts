import { FileOrFolder, GameType } from '@/types'

export function generateHref(item: FileOrFolder, slug: string[]) {
  const a = ['', 'files', ...slug, item.name]

  return a.map(encodeURIComponent).join('/')
}

export function generate_download_url(file_path: string[]): string {
  if (file_path[0] == '合集系列') {
    // Convert 合集系列/浮士德galgame游戏合集/... to galgame0/...
    // This matches the backend tree structure mapping
    const relativePath = file_path.slice(2) // Remove '合集系列' and '浮士德galgame游戏合集'
    const apiPath = ['galgame0', ...relativePath]
    const encodedPath = apiPath.map(encodeURIComponent).join('/')
    return `/api/r2/download-url/${encodedPath}`
  } else {
    const url = 'https://zd.shinnku.top/file/shinnku/'

    return `${url}${file_path.map(encodeURIComponent).join('/')}`
  }
}

export function trim_file_path(file_path: string) {
  const prefix = '合集系列/'

  if (file_path.startsWith(prefix)) {
    return file_path.slice(prefix.length)
  }

  return file_path
}

export function get_game_type(file_path: string): GameType {
  if (file_path.startsWith('合集系列')) {
    return '生肉'
  } else if (file_path.startsWith('zd')) {
    return '熟肉'
  } else if (file_path.startsWith('0/win')) {
    return '熟肉'
  } else return '手机'
}

export function trim_wikipedia_ans(text: string) {
  const p1 = text.indexOf('== 參考')
  const t1 = p1 != -1 ? text.substring(0, p1) : text

  const p2 = t1.indexOf('== 参考')
  const t2 = p2 != -1 ? t1.substring(0, p2) : t1

  return t2
}

export function wikipediaToMarkdown(wikipediaText: string): string {
  const regex = /^(={2,})\s*(.*?)\s*(={2,})$/gm

  return wikipediaText.replace(regex, (match, startEquals, title) => {
    const level = startEquals.length
    const markdownHashes = '#'.repeat(level)

    return `${markdownHashes} ${title}`
  })
}
