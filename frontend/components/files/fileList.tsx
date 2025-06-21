'use client'

import type { Inode } from '@/types'

import { FileZipOutlined, FolderOpenOutlined } from '@ant-design/icons'
import Link from 'next/link'
import { useCallback, useState } from 'react'

import { generateHref } from '@/algorithm/url'
import { num2size } from '@/algorithm/util'
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination'
import { t } from '@/i18n'

interface ListboxWrapperProps {
  children: React.ReactNode
}

export const ListboxWrapper: React.FC<ListboxWrapperProps> = ({ children }) => (
  <div className='rounded-md border border-border px-1 py-2'>{children}</div>
)

export const FileList: React.FC<{
  inode: Inode
  slug: string[]
}> = ({ inode, slug }) => {
  const [page, setPage] = useState(1)
  const onPaginationChange = useCallback((e: number) => setPage(e), [setPage])
  const iconClasses =
    'text-2xl text-muted-foreground pointer-events-none flex-shrink-0'

  const totalPages = Math.ceil(inode.length / 10)
  const items = inode.slice((page - 1) * 10, page * 10)

  return (
    <ListboxWrapper>
      <ul className='divide-y divide-border'>
        {items.map((item, index) => (
          <li key={index} className='max-w-[800px] py-3'>
            <Link
              className='flex items-center gap-2'
              href={generateHref(item, slug)}
            >
              {item.type === 'file' ? (
                <FileZipOutlined className={iconClasses} />
              ) : item.type === 'folder' ? (
                <FolderOpenOutlined className={iconClasses} />
              ) : null}
              <span className='flex-1'>{item.name}</span>
              <span className='text-sm text-muted-foreground'>
                {item.type === 'file'
                  ? `size: ${num2size(item.info.file_size)}`
                  : t('fileFolder')}
              </span>
            </Link>
          </li>
        ))}
      </ul>

      <div className='mt-4 flex justify-center'>
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                href='#'
                onClick={() => page > 1 && onPaginationChange(page - 1)}
              />
            </PaginationItem>
            {Array.from({ length: totalPages }, (_, i) => i + 1).map((p) => (
              <PaginationItem key={p}>
                <PaginationLink
                  href='#'
                  isActive={p === page}
                  onClick={() => onPaginationChange(p)}
                >
                  {p}
                </PaginationLink>
              </PaginationItem>
            ))}
            <PaginationItem>
              <PaginationNext
                href='#'
                onClick={() =>
                  page < totalPages && onPaginationChange(page + 1)
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      </div>
    </ListboxWrapper>
  )
}
