import { BucketFiles, Inode, Variety } from '../types'

export function generateTree(fileList: BucketFiles): any {
  const root: any = {}

  for (const file of fileList) {
    const filePath = file.file_path
    const pathParts = filePath.split('/')
    let pointer = root
    const l = pathParts.length - 1

    for (let i = 0; i < l; i++) {
      const part = pathParts[i]

      if (pointer[part] === undefined) {
        pointer[part] = {}
      }
      pointer = pointer[part]
    }
    pointer[pathParts[l]] = file
  }

  return root
}

export function node2list(node: { [key: string]: any }): Inode {
  const inode: Inode = []

  for (const child in node) {
    const file_path: string | undefined = node[child]['file_path']

    if (file_path) {
      inode.push({
        type: 'file',
        name: child,
        info: node[child],
      })
    } else {
      inode.push({
        type: 'folder',
        name: child,
      })
    }
  }

  return inode
}

export function checknodevariety(node: any) {
  let variety: Variety

  if (node) {
    if (node['file_path']) {
      variety = 'file'
    } else {
      variety = 'folder'
    }
  } else {
    variety = '404'
  }

  return variety
}
