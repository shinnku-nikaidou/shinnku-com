import {
  AndroidOutlined,
  CodeOutlined,
  GlobalOutlined,
  WindowsFilled,
  WindowsOutlined,
} from '@ant-design/icons'
import Image from 'next/image'

import { KRKROutlined, ONSOutlined } from '@/components/galgame-icons'
import { t } from '@/i18n'

const indexItems = [
  {
    titleKey: 'indexForum',
    shortKey: 'indexForum',
    link: 'https://galgame.dev/',
    iconLarge: <GlobalOutlined className='my-icon-large' />,
    iconSmall: <GlobalOutlined />,
  },
  {
    titleKey: 'indexOldWin',
    shortKey: 'indexOldWinShort',
    link: '/files/shinnku/0/win',
    iconLarge: <WindowsOutlined className='my-icon-large' />,
    iconSmall: <WindowsOutlined />,
  },
  {
    titleKey: 'indexNewWin',
    shortKey: 'indexNewWinShort',
    link: '/files/shinnku/zd',
    iconLarge: <WindowsFilled className='my-icon-large' />,
    iconSmall: <WindowsFilled />,
  },
  {
    titleKey: 'indexApk',
    shortKey: 'indexApkShort',
    link: '/files/shinnku/0/apk',
    iconLarge: <AndroidOutlined className='my-icon-large' />,
    iconSmall: <AndroidOutlined />,
  },
  {
    titleKey: 'indexOns',
    shortKey: 'indexOnsShort',
    link: '/files/shinnku/0/ons',
    iconLarge: <ONSOutlined height={60} width={60} />,
    iconSmall: <ONSOutlined height={24} width={24} />,
  },
  {
    titleKey: 'indexKrkr',
    shortKey: 'indexKrkrShort',
    link: '/files/shinnku/0/krkr',
    iconLarge: <KRKROutlined height={36} width={36} />,
    iconSmall: <KRKROutlined height={16} width={16} />,
  },
  {
    titleKey: 'indexTools',
    shortKey: 'indexToolsShort',
    link: '/files/shinnku/0/tools',
    iconLarge: <CodeOutlined className='my-icon-large' />,
    iconSmall: <CodeOutlined />,
  },
  {
    titleKey: 'indexRaw',
    shortKey: 'indexRawShort',
    link: '/files/galgame0',
    iconLarge: (
      <Image alt={'japan'} height={40} src={'/japan.svg'} width={40} />
    ),
    iconSmall: (
      <Image alt={'japan'} height={20} src={'/japan.svg'} width={20} />
    ),
  },
] as const

export const IndexList = indexItems.map((item) => ({
  title: t(item.titleKey),
  link: item.link,
  body: item.iconLarge,
}))

export const IndexListForSlog = indexItems.map((item) => ({
  title: t(item.shortKey),
  link: item.link,
  body: item.iconSmall,
}))
