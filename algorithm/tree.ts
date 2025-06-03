import { BucketFiles, FileInfo, Inode, TreeNode, Variety } from '@/types'

export function generateTree(fileList: BucketFiles): TreeNode {
  const root: TreeNode = {}

  for (const file of fileList) {
    const filePath = file.file_path
    const pathParts = filePath.split('/')
    let pointer: TreeNode = root
    const l = pathParts.length - 1

    for (let i = 0; i < l; i++) {
      const part = pathParts[i]

      if (pointer[part] === undefined) {
        pointer[part] = {}
      }
      pointer = pointer[part] as TreeNode
    }
    pointer[pathParts[l]] = file
  }

  return root
}

export function node2list(node: TreeNode): Inode {
  const inode: Inode = []

  for (const child in node) {
    const value = node[child]
    if ((value as FileInfo).file_path) {
      inode.push({
        type: 'file',
        name: child,
        info: value as FileInfo,
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

export function checknodevariety(node: TreeNode | FileInfo | undefined): Variety {
  if (!node) {
    return '404'
  }

  if ((node as FileInfo).file_path) {
    return 'file'
  }

  return 'folder'
}
