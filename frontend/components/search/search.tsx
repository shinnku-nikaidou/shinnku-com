'use client'

import React, { useState } from 'react'
import { useRouter } from 'next/navigation'
import { Input } from '@heroui/input'
import { Kbd } from '@heroui/kbd'

import { SearchIcon } from '../icons'

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
    <Input
      aria-label='Search'
      classNames={{
        inputWrapper:
          'rounded-full bg-white dark:bg-black shadow-md border border-gray-200 focus-within:border-blue-500',
        input: 'text-base text-gray-700 placeholder-gray-500 dark:text-white',
      }}
      color='primary'
      endContent={<Kbd className='hidden lg:inline-block' keys={['enter']} />}
      labelPlacement='outside'
      placeholder={t('searchPlaceholder')}
      radius='full'
      size='lg'
      startContent={
        <SearchIcon className='pointer-events-none flex-shrink-0 text-base text-default-400' />
      }
      type='search'
      value={searchTerm}
      onChange={handleInputChange}
      onKeyDown={handleKeyDown}
    />
  )
}

export default Search // You can still have a default export if needed.
