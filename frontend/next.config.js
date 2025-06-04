/** @type {import('next').NextConfig} */
const nextConfig = {
  transpilePackages: ['next-mdx-remote'],
  i18n: {
    locales: ['zh-cn'],
    defaultLocale: 'zh-cn',
  },
}

module.exports = nextConfig
