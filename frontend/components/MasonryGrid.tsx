'use client'

import { useEffect, useRef, useState } from 'react'

import { useResizeObserver } from '@/hooks/useResizeObserver'
import { cn } from '@/utils/cn'

interface KunMasonryGridProps {
  children: React.ReactNode[]
  columnWidth?: number
  gap?: number
  className?: string
}

export const KunMasonryGrid = ({
  children,
  columnWidth = 256,
  gap = 24,
  className,
}: KunMasonryGridProps) => {
  const containerRef = useRef<HTMLDivElement>(null)
  const [columns, setColumns] = useState(1)
  const [isLoaded, setIsLoaded] = useState(false)

  const { width: containerWidth } = useResizeObserver(containerRef)

  useEffect(() => {
    const calculateColumns = () => {
      if (!containerWidth) {
        return
      }

      const newColumns = Math.max(
        1,
        Math.floor((containerWidth + gap) / (columnWidth + gap)),
      )

      setColumns(newColumns)
      if (!isLoaded) setIsLoaded(true)
    }

    calculateColumns()
  }, [containerWidth, columnWidth, gap, isLoaded])

  const distributeItems = () => {
    if (!Array.isArray(children)) {
      return []
    }

    const columnHeights = Array(columns).fill(0)
    const columnItems: React.ReactNode[][] = Array(columns)
      .fill(null)
      .map(() => [])

    children.forEach((child) => {
      if (!child) {
        return
      }
      const shortestColumn = columnHeights.indexOf(Math.min(...columnHeights))

      columnItems[shortestColumn].push(child)
      columnHeights[shortestColumn]++
    })

    return columnItems
  }

  return (
    <div
      ref={containerRef}
      className={cn(
        'grid w-full transition-opacity duration-300',
        isLoaded ? 'opacity-100' : 'opacity-0',
        className,
      )}
      style={{
        gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
        gap: `${gap}px`,
        maxWidth: '100%',
      }}
    >
      {distributeItems().map((column, columnIndex) => (
        <div
          key={columnIndex}
          className='flex flex-col'
          style={{ gap: `${gap}px` }}
        >
          {column.map((item, itemIndex) => (
            <div key={itemIndex} className='w-full'>
              {item}
            </div>
          ))}
        </div>
      ))}
    </div>
  )
}
