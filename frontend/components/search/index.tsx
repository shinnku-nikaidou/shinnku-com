'use client'

import { useRouter } from '@bprogress/next'
import React, { useState } from 'react'

import { SearchIcon } from '@/components/ui/icons'
import { Input } from '@/components/ui/input'
import { Kbd } from '@/components/ui/kbd'
import { t } from '@/i18n'

interface SearchProps {
  initialSearchTerm?: string
}

export const Search: React.FC<SearchProps> = ({ initialSearchTerm = '' }) => {
  const [searchTerm, setSearchTerm] = useState<string>(initialSearchTerm)
  const router = useRouter()

  const handleSearch = () => {
    if (searchTerm.trim() !== '') {
      router.push(`/search?q=${encodeURIComponent(searchTerm)}`)
    }
  }

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(event.target.value)
  }

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      handleSearch()
    }
  }

  return (
    <div className='relative mx-auto w-full max-w-2xl'>
      <SearchIcon className='text-muted-foreground absolute top-1/2 left-4 h-5 w-5 -translate-y-1/2' />
      <Input
        aria-label='Search'
        className='pr-14 pl-10 shadow-sm'
        placeholder={t('searchPlaceholder')}
        type='search'
        value={searchTerm}
        onChange={handleInputChange}
        onKeyDown={handleKeyDown}
      />
      <Kbd
        className='absolute top-1/2 right-4 hidden -translate-y-1/2 lg:block'
        onClick={handleSearch}
      >
        Enter
      </Kbd>
    </div>
  )
}

export default Search
