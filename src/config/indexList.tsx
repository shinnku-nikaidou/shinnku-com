import {
  AndroidOutlined,
  CodeOutlined,
  GlobalOutlined,
  WindowsFilled,
  WindowsOutlined,
} from '@ant-design/icons'
import Image from 'next/image'

import { KRKROutlined, ONSOutlined } from '@/components/galgame-icons'

export const IndexList = [
  {
    title: '真红论坛',
    link: 'https://galgame.dev/',
    body: <GlobalOutlined className='my-icon-large' />,
  },
  {
    title: '前win汉化集合',
    link: '/files/shinnku/0/win',
    body: <WindowsOutlined className='my-icon-large' />,
  },
  {
    title: '新win汉化集合',
    link: '/files/shinnku/zd',
    body: <WindowsFilled className='my-icon-large' />,
  },
  {
    title: 'apk集合',
    link: '/files/shinnku/0/apk',
    body: <AndroidOutlined className='my-icon-large' />,
  },
  {
    title: 'ons集合',
    link: '/files/shinnku/0/ons',
    body: <ONSOutlined height={60} width={60} />,
  },
  {
    title: 'krkr集合',
    link: '/files/shinnku/0/krkr',
    body: <KRKROutlined height={36} width={36} />,
  },
  {
    title: 'galgame工具',
    link: '/files/shinnku/0/tools',
    body: <CodeOutlined className='my-icon-large' />,
  },
  {
    title: '生肉硬盘集合',
    link: '/files/galgame0',
    body: <Image alt={'japan'} height={40} src={'/japan.svg'} width={40} />,
  },
]

export const IndexListForSlog = [
  {
    title: '真红论坛',
    link: 'https://galgame.dev/',
    body: <GlobalOutlined />,
  },
  {
    title: '前win汉化',
    link: '/files/shinnku/0/win',
    body: <WindowsOutlined />,
  },
  {
    title: '新win汉化',
    link: '/files/shinnku/zd',
    body: <WindowsFilled />,
  },
  {
    title: 'apk集合',
    link: '/files/shinnku/0/apk',
    body: <AndroidOutlined />,
  },
  {
    title: 'ons集合',
    link: '/files/shinnku/0/ons',
    body: <ONSOutlined height={24} width={24} />,
  },
  {
    title: 'krkr集合',
    link: '/files/shinnku/0/krkr',
    body: <KRKROutlined height={16} width={16} />,
  },
  {
    title: 'gal工具',
    link: '/files/shinnku/0/tools',
    body: <CodeOutlined />,
  },
  {
    title: '生肉集合',
    link: '/files/galgame0',
    body: <Image alt={'japan'} height={20} src={'/japan.svg'} width={20} />,
  },
]
