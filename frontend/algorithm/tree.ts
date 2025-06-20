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
