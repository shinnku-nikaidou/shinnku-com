import { Link } from '@heroui/link'
import { button as buttonStyles } from '@heroui/theme'

import { siteConfig } from '@/config/site'
import { title, subtitle } from '@/components/primitives'
import Search from '@/components/search/search'

export default function Home() {
  return (
    <section className='flex flex-col items-center justify-center gap-4 py-8 md:py-10'>
      <div className='inline-block max-w-xl justify-center text-center'>
        <span className={title({ color: 'pink' })}>真红&nbsp;</span>
        <span className={title()}>小站&nbsp;</span>
        <div className={subtitle({ class: 'mt-6' })}>
          真红小站（原 失落小站）一个galgame资源站, 收录了大部分的汉化galgame,
          大部分的生肉galgame资源。拥有最先进的galgame中文ai搜索引擎。
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
          浏览全部游戏
        </Link>
        <Link
          isExternal
          className={buttonStyles({ variant: 'bordered', radius: 'full' })}
          href={'https://congyu.moe/'}
        >
          {`丛雨vpn(这是广告，但保证质量)`}
        </Link>
      </div>
    </section>
  )
}
