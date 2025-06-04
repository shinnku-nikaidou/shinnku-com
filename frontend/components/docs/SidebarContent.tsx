'use client'

import { TreeItem } from './SideTreeItem'

import { KunTreeNode } from '@/lib/mdx/types'

interface Props {
  tree: KunTreeNode
}

export const SidebarContent = ({ tree }: Props) => {
  return (
    <div>
      {tree.type === 'directory' &&
        tree.children?.map((child, index) => (
          <TreeItem key={index} level={0} node={child} />
        ))}
    </div>
  )
}
