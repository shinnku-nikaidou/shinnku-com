import { subtitle, title } from '@/components/primitives'
import Search from '@/components/search'
import { Button } from '@/components/ui/button'
import { siteConfig } from '@/config/site'
import { t } from '@/i18n'
import Link from 'next/link'

export default function Home() {
  return (
    <section className='flex flex-col items-center justify-center h-[calc(100vh-128px)] gap-4'>
      <div className='inline-block max-w-xl text-center'>
        <span className={title({ color: 'pink' })}>
          {t('websiteName').slice(0, 2)}&nbsp;
        </span>
        <span className={title()}>{t('websiteName').slice(2)}&nbsp;</span>
        <div className={subtitle({ class: 'mt-6' })}>
          {t('pageWelcomeDescription')}
        </div>
      </div>

      <div className='mt-10 flex w-full items-center justify-center'>
        <div className='w-full max-w-3xl gap-4'>
          <Search />
        </div>
      </div>

      <div className='mt-8 flex gap-3'>
        <Button asChild className='rounded-full' variant='outline'>
          <Link href={siteConfig.links.files}>{t('browseAllGames')}</Link>
        </Button>
        <Button asChild className='rounded-full' variant='outline'>
          <Link
            href='https://congyu.moe/'
            rel='noopener noreferrer'
            target='_blank'
          >
            {t('advertisement')}
          </Link>
        </Button>
      </div>
    </section>
  )
}
