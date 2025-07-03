'use client'

import type { Inode } from '@/types'

import { FileZipOutlined, FolderOpenOutlined } from '@ant-design/icons'
import Link from 'next/link'
import { useCallback, useState } from 'react'

import { generateHref } from '@/algorithm/url'
import { num2size } from '@/algorithm/util'
import { t } from '@/i18n'
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@ui/pagination'

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
    'text-2xl text-muted-foreground pointer-events-none shrink-0'

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

            {/* First page */}
            {totalPages > 0 && (
              <PaginationItem>
                <PaginationLink
                  href='#'
                  isActive={1 === page}
                  onClick={() => onPaginationChange(1)}
                >
                  1
                </PaginationLink>
              </PaginationItem>
            )}

            {/* Left ellipsis */}
            {page > 3 && (
              <PaginationItem>
                <PaginationEllipsis />
              </PaginationItem>
            )}

            {/* Pages around current */}
            {Array.from({ length: totalPages }, (_, i) => i + 1)
              .filter(
                (p) =>
                  p !== 1 && p !== totalPages && p >= page - 1 && p <= page + 1,
              )
              .map((p) => (
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

            {/* Right ellipsis */}
            {page < totalPages - 2 && (
              <PaginationItem>
                <PaginationEllipsis />
              </PaginationItem>
            )}

            {/* Last page */}
            {totalPages > 1 && (
              <PaginationItem>
                <PaginationLink
                  href='#'
                  isActive={totalPages === page}
                  onClick={() => onPaginationChange(totalPages)}
                >
                  {totalPages}
                </PaginationLink>
              </PaginationItem>
            )}

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
