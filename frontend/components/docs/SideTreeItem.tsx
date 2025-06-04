'use client'

import { useState } from 'react'
import { Link } from '@heroui/link'
import { ChevronRight, FileText, FolderOpen } from 'lucide-react'
import { useRouter } from 'next/navigation'

import { KunTreeNode } from '@/lib/mdx/types'
import { cn } from '@/utils/cn'

interface TreeItemProps {
  node: KunTreeNode
  level: number
}

export const TreeItem = ({ node, level }: TreeItemProps) => {
  const router = useRouter()
  const [isOpen, setIsOpen] = useState(true)

  const handleClick = () => {
    if (node.type === 'directory') {
      setIsOpen(!isOpen)
    } else {
      router.push(`/docs/${node.path}`)
    }
  }

  return (
    <nav className='select-none'>
      <Link
        className={cn(
          'w-full cursor-pointer justify-start gap-2 rounded-xl px-3 py-2',
          level === 0 ? 'mt-0' : 'mt-1',
          'hover:bg-default/40',
        )}
        color='foreground'
        style={{ paddingLeft: `${level * 12 + 12}px` }}
        onPress={handleClick}
      >
        <div className='flex items-center gap-2'>
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
          <span className='text-wrap text-left text-sm'>{node.label}</span>
        </div>
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
