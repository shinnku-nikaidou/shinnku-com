export type SiteConfig = typeof siteConfig

export const siteConfig = {
  name: '真红小站',
  description:
    '这里是真红小站（失落小站）, 一个galgame资源站, (包括visual novel, 黄油, psp, krkr, ons gal资源 等), 收录了大部分的汉化galgame, 大部分的生肉galgame资源。',
  navItems: [
    {
      label: '文档',
      href: '/docs',
    },
    {
      label: '关于',
      href: '/about',
    },
    {
      label: '目录',
      href: '/files',
    },
  ],
  navMenuItems: [
    {
      label: '文档',
      href: '/docs',
    },
    {
      label: '关于',
      href: '/about',
    },
    {
      label: '目录',
      href: '/files',
    },
  ],
  links: {
    github: 'https://github.com/shinnku-nikaidou/shinnku-com',
    docs: '/docs',
    files: '/files',
  },
}
