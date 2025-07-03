'use client'

import { Menu, X } from 'lucide-react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useEffect, useState } from 'react'

import { ThemeSwitch } from '@/components/primitives/ThemeSwitch'
import { siteConfig } from '@/config/site'
import { t } from '@/i18n'
import { cn } from '@/lib/utils'
import { Button } from '@ui/button'
import { GithubIcon, Logo } from '@ui/icons'

export const Navbar = () => {
  const pathname = usePathname()
  const [isMenuOpen, setIsMenuOpen] = useState(false)

  useEffect(() => {
    setIsMenuOpen(false)
  }, [pathname])

  return (
    <nav className='bg-background/80 fixed top-0 left-0 z-50 w-full border-b backdrop-blur-sm'>
      <div className='mx-auto flex h-14 max-w-[1280px] items-center justify-between px-6'>
        <div className='flex items-center'>
          <Link className='flex items-center gap-1' href='/'>
            <Logo />
            <p className='font-bold'>{t('websiteName')}</p>
          </Link>
          <ul className='ml-4 hidden gap-4 sm:flex'>
            {siteConfig.navItems.map((item) => (
              <li key={item.href}>
                <Link
                  className={cn(
                    'hover:text-foreground/80 text-sm transition-colors',
                    pathname === item.href && 'text-primary font-medium',
                  )}
                  href={item.href}
                >
                  {item.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>

        <div className='hidden items-center gap-2 sm:flex'>
          <Link
            aria-label='Github'
            className='text-default-500'
            href={siteConfig.links.github}
            target='_blank'
          >
            <GithubIcon className='text-default-500' />
          </Link>
          <ThemeSwitch />
        </div>

        <div className='flex items-center gap-2 sm:hidden'>
          <Link
            aria-label='Github'
            className='text-default-500'
            href={siteConfig.links.github}
            target='_blank'
          >
            <GithubIcon className='text-default-500' />
          </Link>
          <ThemeSwitch />
          <Button
            size='icon'
            variant='ghost'
            onClick={() => setIsMenuOpen((open) => !open)}
          >
            {isMenuOpen ? (
              <X className='size-5' />
            ) : (
              <Menu className='size-5' />
            )}
          </Button>
        </div>
      </div>

      {isMenuOpen && (
        <div className='sm:hidden'>
          <ul className='bg-background space-y-2 border-t p-4'>
            {siteConfig.navItems.map((item) => (
              <li key={item.href}>
                <Link
                  className='block py-2 text-sm font-medium'
                  href={item.href}
                >
                  {item.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      )}
    </nav>
  )
}
