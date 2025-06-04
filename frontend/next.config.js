/** @type {import('next').NextConfig} */
const nextConfig = {
  transpilePackages: ['next-mdx-remote'],
  i18n: {
    locales: ['zh-cn', 'zh-tw', 'en-us'],
    defaultLocale: 'zh-cn',
  },
}

module.exports = nextConfig
