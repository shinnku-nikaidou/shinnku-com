'use client'

import {
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerHeader,
  Link,
  useDisclosure,
} from '@heroui/react'
import { ChevronRight } from 'lucide-react'

import { SidebarContent } from './SidebarContent'

import { t } from '@/i18n'
import { BlogTreeNode } from '@/lib/mdx/types'

interface Props {
  tree: BlogTreeNode
}

export const BlogSidebar = ({ tree }: Props) => {
  const { isOpen, onOpen, onOpenChange } = useDisclosure()

  return (
    <div className='blog-scroll-nav'>
      <aside className='fixed top-32 hidden h-[calc(100dvh-256px)] w-64 bg-background py-2 md:block'>
        <div className='flex h-full flex-col overflow-scroll border-r border-default-200 bg-background px-4 scrollbar-hide'>
          <Link className='my-3 text-xl' color='foreground' href='/about'>
            {t('navMenuDirectory')}
          </Link>
          <SidebarContent tree={tree} />
        </div>
      </aside>

      <button
        className='fixed left-0 top-0 flex h-full cursor-pointer items-center text-default-500 md:hidden'
        onClick={() => onOpen()}
      >
        <ChevronRight size={24} />
      </button>

      <Drawer
        isOpen={isOpen}
        placement='left'
        size='xs'
        onOpenChange={onOpenChange}
      >
        <DrawerContent>
          <DrawerHeader className='flex flex-col gap-1'>
            {t('navMenuDirectory')}
          </DrawerHeader>
          <DrawerBody>
            <SidebarContent tree={tree} />
          </DrawerBody>
        </DrawerContent>
      </Drawer>
    </div>
  )
}
