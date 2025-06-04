'use client'

import { useCallback, useEffect, useState } from 'react'

interface Size {
  width: number | undefined
  height: number | undefined
}

export const useResizeObserver = (ref: React.RefObject<HTMLElement | null>) => {
  const [size, setSize] = useState<Size>({
    width: undefined,
    height: undefined,
  })

  const handleResize = useCallback((entries: ResizeObserverEntry[]) => {
    const entry = entries[0]

    if (entry) {
      const { width, height } = entry.contentRect

      setSize({ width, height })
    }
  }, [])

  useEffect(() => {
    if (!ref.current) {
      return
    }

    const observer = new ResizeObserver(handleResize)

    observer.observe(ref.current)

    return () => {
      observer.disconnect()
    }
  }, [ref, handleResize])

  return size
}
