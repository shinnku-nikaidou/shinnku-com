import { BlogFrontmatterSchema } from '../validation'
import type { Blog, BlogPostMetadata } from './types'

import fs from 'fs'
import path from 'path'

import matter from 'gray-matter'

import { markdownToText } from '@/lib/utils/markdownToText'

const POSTS_PATH = path.join(process.cwd(), 'posts')

export const getAllPosts = () => {
  const posts: BlogPostMetadata[] = []

  const traverseDirectory = (currentPath: string, basePath: string = '') => {
    const files = fs.readdirSync(currentPath)

    files.forEach((file) => {
      const filePath = path.join(currentPath, file)
      const stat = fs.statSync(filePath)

      if (stat.isDirectory()) {
        traverseDirectory(filePath, path.join(basePath, file))
      } else if (file.endsWith('.mdx')) {
        const fileContents = fs.readFileSync(filePath, 'utf8')
        const { data } = matter(fileContents)
        const parsed = BlogFrontmatterSchema.safeParse(data)
        if (!parsed.success) {
          throw new Error(
            `Invalid frontmatter in ${filePath}: ${parsed.error.message}`,
          )
        }
        const frontmatter = parsed.data

        const slug = path
          .join(basePath, file.replace(/\.mdx$/, ''))
          .replace(/\\/g, '/')

        posts.push({
          title: frontmatter.title,
          banner: frontmatter.banner,
          date: frontmatter.date
            ? new Date(frontmatter.date).toISOString()
            : '',
          description: frontmatter.description,
          textCount: markdownToText(fileContents).length - 300,
          slug,
          path: slug,
        })
      }
    })
  }

  traverseDirectory(POSTS_PATH)

  return posts.sort((a, b) => (a.date > b.date ? -1 : 1))
}

export const getPostBySlug = (slug: string): Blog => {
  const realSlug = slug.replace(/\.mdx$/, '')
  const fullPath = path.join(POSTS_PATH, `${realSlug}.mdx`)
  const fileContents = fs.readFileSync(fullPath, 'utf8')
  const { data, content } = matter(fileContents)
  const parsed = BlogFrontmatterSchema.safeParse(data)
  if (!parsed.success) {
    throw new Error(
      `Invalid frontmatter in ${fullPath}: ${parsed.error.message}`,
    )
  }

  return {
    slug: realSlug,
    content,
    frontmatter: parsed.data,
  }
}

export const getAdjacentPosts = (currentSlug: string) => {
  const posts = getAllPosts()
  const currentIndex = posts.findIndex((post) => post.slug === currentSlug)

  return {
    prev: currentIndex > 0 ? posts[currentIndex - 1] : null,
    next: currentIndex < posts.length - 1 ? posts[currentIndex + 1] : null,
  }
}
