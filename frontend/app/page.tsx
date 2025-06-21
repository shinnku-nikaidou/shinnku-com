import { Link } from '@heroui/link'
import { button as buttonStyles } from '@heroui/theme'

import { subtitle, title } from '@/components/primitives'
import Search from '@/components/search'
import { siteConfig } from '@/config/site'
import { t } from '@/i18n'

export default function Home() {
  return (
    <section className='flex flex-col items-center justify-center gap-4 py-8 md:py-10'>
      <div className='inline-block max-w-xl justify-center text-center'>
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
        <Link
          className={buttonStyles({ variant: 'bordered', radius: 'full' })}
          href={siteConfig.links.files}
        >
          {t('browseAllGames')}
        </Link>
        <Link
          isExternal
          className={buttonStyles({ variant: 'bordered', radius: 'full' })}
          href={'https://congyu.moe/'}
        >
          {t('advertisement')}
        </Link>
      </div>
    </section>
  )
}
