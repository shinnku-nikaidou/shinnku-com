'use client'

import { ChevronRight } from 'lucide-react'
import Link from 'next/link'
import { useState } from 'react'

import { SidebarContent } from './SidebarContent'

import { Drawer, DrawerContent, DrawerHeader } from '@/components/ui/drawer'
import { t } from '@/i18n'
import { BlogTreeNode } from '@/lib/mdx/types'

interface Props {
  tree: BlogTreeNode
}

export const BlogSidebar = ({ tree }: Props) => {
  const [open, setOpen] = useState(false)

  return (
    <div className='blog-scroll-nav'>
      <aside className='bg-background fixed top-32 hidden h-[calc(100dvh-256px)] w-64 py-2 md:block'>
        <div className='border-default-200 bg-background scrollbar-hide flex h-full flex-col overflow-scroll border-r px-4'>
          <Link className='my-3 text-xl' href='/about'>
            {t('navMenuDirectory')}
          </Link>
          <SidebarContent tree={tree} />
        </div>
      </aside>

      <button
        className='text-default-500 fixed top-0 left-0 flex h-full cursor-pointer items-center md:hidden'
        onClick={() => setOpen(true)}
      >
        <ChevronRight size={24} />
      </button>

      <Drawer open={open} onOpenChange={setOpen}>
        <DrawerContent>
          <DrawerHeader className='flex flex-col gap-1'>
            {t('navMenuDirectory')}
          </DrawerHeader>
          <div className='px-4'>
            <SidebarContent tree={tree} />
          </div>
        </DrawerContent>
      </Drawer>
    </div>
  )
}
