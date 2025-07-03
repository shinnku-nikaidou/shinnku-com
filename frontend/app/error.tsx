'use client'

import { useEffect } from 'react'

import { t } from '@/i18n'

export default function Error({
  error,
  reset,
}: {
  error: Error
  reset: () => void
}) {
  useEffect(() => {
    // Log the error to an error reporting service
     
    console.error(error)
  }, [error])

  return (
    <div>
      <h2>{t('errorSomethingWrong')}</h2>
      <button
        onClick={
          // Attempt to recover by trying to re-render the segment
          () => reset()
        }
      >
        {t('errorTryAgain')}
      </button>
    </div>
  )
}
