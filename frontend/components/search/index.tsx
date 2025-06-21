'use client'

import React, { useState } from 'react'
import { useRouter } from '@bprogress/next'
import { Input } from '@heroui/input'
import { Kbd } from '@heroui/kbd'

import { SearchIcon } from '../ui/icons'

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
      color='primary'
      endContent={<Kbd className='hidden lg:inline-block' keys={['enter']} />}
      labelPlacement='outside'
      placeholder={t('searchPlaceholder')}
      radius='full'
      size='lg'
      startContent={<SearchIcon className='' />}
      type='search'
      value={searchTerm}
      onChange={handleInputChange}
      onKeyDown={handleKeyDown}
    />
  )
}

export default Search // You can still have a default export if needed.
