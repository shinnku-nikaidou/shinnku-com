import {
  AndroidOutlined,
  CodeOutlined,
  GlobalOutlined,
  WindowsFilled,
  WindowsOutlined,
} from '@ant-design/icons'
import Image from 'next/image'

import { KRKROutlined, ONSOutlined } from '@/components/galgame-icons'
import { t } from '@/i18n/zh'

export const IndexList = [
  {
    title: t('indexForum'),
    link: 'https://galgame.dev/',
    body: <GlobalOutlined className='my-icon-large' />,
  },
  {
    title: t('indexOldWin'),
    link: '/files/shinnku/0/win',
    body: <WindowsOutlined className='my-icon-large' />,
  },
  {
    title: t('indexNewWin'),
    link: '/files/shinnku/zd',
    body: <WindowsFilled className='my-icon-large' />,
  },
  {
    title: t('indexApk'),
    link: '/files/shinnku/0/apk',
    body: <AndroidOutlined className='my-icon-large' />,
  },
  {
    title: t('indexOns'),
    link: '/files/shinnku/0/ons',
    body: <ONSOutlined height={60} width={60} />,
  },
  {
    title: t('indexKrkr'),
    link: '/files/shinnku/0/krkr',
    body: <KRKROutlined height={36} width={36} />,
  },
  {
    title: t('indexTools'),
    link: '/files/shinnku/0/tools',
    body: <CodeOutlined className='my-icon-large' />,
  },
  {
    title: t('indexRaw'),
    link: '/files/galgame0',
    body: <Image alt={'japan'} height={40} src={'/japan.svg'} width={40} />,
  },
]

export const IndexListForSlog = [
  {
    title: t('indexForum'),
    link: 'https://galgame.dev/',
    body: <GlobalOutlined />,
  },
  {
    title: t('indexOldWinShort'),
    link: '/files/shinnku/0/win',
    body: <WindowsOutlined />,
  },
  {
    title: t('indexNewWinShort'),
    link: '/files/shinnku/zd',
    body: <WindowsFilled />,
  },
  {
    title: t('indexApkShort'),
    link: '/files/shinnku/0/apk',
    body: <AndroidOutlined />,
  },
  {
    title: t('indexOnsShort'),
    link: '/files/shinnku/0/ons',
    body: <ONSOutlined height={24} width={24} />,
  },
  {
    title: t('indexKrkrShort'),
    link: '/files/shinnku/0/krkr',
    body: <KRKROutlined height={16} width={16} />,
  },
  {
    title: t('indexToolsShort'),
    link: '/files/shinnku/0/tools',
    body: <CodeOutlined />,
  },
  {
    title: t('indexRawShort'),
    link: '/files/galgame0',
    body: <Image alt={'japan'} height={20} src={'/japan.svg'} width={20} />,
  },
]
