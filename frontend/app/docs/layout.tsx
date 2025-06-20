import { ReactNode } from 'react'

import { BlogSidebar } from '@/components/blog/Sidebar'
import { getDirectoryTree } from '@/lib/mdx/directoryTree'

interface LayoutProps {
  children: ReactNode
}

export default function Layout({ children }: LayoutProps) {
  const tree = getDirectoryTree()

  return (
    <div className='container mx-auto flex max-w-[1280px]'>
      <BlogSidebar tree={tree} />
      <main className='flex-1 overflow-y-auto py-4 pl-0 md:pl-64'>
        {children}
      </main>
    </div>
  )
}
