'use client'

import { usePathname } from 'next/navigation'

export const BackgroundBox = () => {
  const pathname = usePathname()
  const isHome = pathname === '/'

  return (
    <div
      className="box bg-[url('/assets/GT5Bjdba4AAbCkU.jpeg')] md:bg-[url('/assets/81320307_p0.jpg')]"
      id='box-main'
      style={{ opacity: isHome ? 0.24 : 0.08 }}
    />
  )
}
