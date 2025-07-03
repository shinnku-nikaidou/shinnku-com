export type SiteConfig = typeof siteConfig

import { t } from '@/i18n'

export const siteConfig = {
  name: t('websiteName'),
  description: t('siteDescription'),
  navItems: [
    {
      label: t('navDocs'),
      href: '/docs',
    },
    {
      label: t('navAbout'),
      href: '/about',
    },
    {
      label: t('navFiles'),
      href: '/files',
    },
  ],
  links: {
    github: 'https://github.com/shinnku-nikaidou/shinnku-com',
    docs: '/docs',
    files: '/files',
  },
}
