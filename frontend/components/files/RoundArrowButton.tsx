'use client'

import { useRouter } from '@bprogress/next'
import { Button } from '@ui/button'
import { ArrowLeft } from 'lucide-react'

interface RoundArrowButtonProps {
  ariaLabel?: string
}

export const RoundArrowButton: React.FC<RoundArrowButtonProps> = ({
  ariaLabel,
}) => {
  const router = useRouter()

  return (
    <Button
      aria-label={ariaLabel}
      className='absolute bottom-12 left-6 rounded-full'
      size='icon'
      variant='secondary'
      onClick={() => router.back()}
    >
      <ArrowLeft aria-hidden='true' className='size-5' />
    </Button>
  )
}
