'use client'

import { useRouter } from '@bprogress/next'
import { ChevronRight, FileText, FolderOpen } from 'lucide-react'
import Link from 'next/link'
import { useState } from 'react'

import { BlogTreeNode } from '@/lib/mdx/types'
import { cn } from '@/utils/cn'

interface TreeItemProps {
  node: BlogTreeNode
  level: number
}

export const TreeItem = ({ node, level }: TreeItemProps) => {
  const router = useRouter()
  const [isOpen, setIsOpen] = useState(true)

  const handleClick = (
    event: React.MouseEvent<HTMLAnchorElement, MouseEvent>,
  ) => {
    if (node.type === 'directory') {
      event.preventDefault()
      setIsOpen(!isOpen)
    } else {
      router.push(`/docs/${node.path}`)
    }
  }

  return (
    <nav className='select-none'>
      <Link
        className={cn(
          'flex w-full items-center cursor-pointer rounded-xl py-2',
          level === 0 ? 'mt-0' : 'mt-1',
          'hover:bg-default/40',
        )}
        href={node.type === 'directory' ? '#' : `/docs/${node.path}`}
        style={{ paddingLeft: `${level * 12 + 12}px` }}
        onClick={handleClick}
      >
        {node.type === 'directory' ? (
          <>
            <ChevronRight
              className={`transition-transform duration-200 ${
                isOpen ? 'rotate-90' : ''
              }`}
              size={16}
            />
            <FolderOpen className='text-warning' size={16} />
          </>
        ) : (
          <FileText className='ml-5 shrink-0 text-primary' size={16} />
        )}
        <span className='ml-2 text-wrap text-left text-sm'>{node.label}</span>
      </Link>
      {node.type === 'directory' && isOpen && (
        <div className='overflow-hidden'>
          {node.children?.map((child, index) => (
            <TreeItem key={index} level={level + 1} node={child} />
          ))}
        </div>
      )}
    </nav>
  )
}
