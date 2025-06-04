import type { KunTreeNode } from './types'

import fs from 'fs'
import path from 'path'

import matter from 'gray-matter'

import { docDirectoryLabelMap } from '@/constants/doc'

const POSTS_PATH = path.join(process.cwd(), 'posts')

export const getDirectoryTree = (): KunTreeNode => {
  const buildTree = (
    currentPath: string,
    baseName: string,
  ): KunTreeNode | null => {
    const stats = fs.statSync(currentPath)

    if (stats.isFile() && currentPath.endsWith('.mdx')) {
      const fileContents = fs.readFileSync(currentPath, 'utf8')
      const { data } = matter(fileContents)

      return {
        name: baseName.replace(/\.mdx$/, ''),
        label: data.title,
        path: path
          .relative(POSTS_PATH, currentPath)
          .replace(/\.mdx$/, '')
          .replace(/\\/g, '/'),
        type: 'file',
      }
    }

    if (stats.isDirectory()) {
      const children = fs
        .readdirSync(currentPath)
        .map((child) => buildTree(path.join(currentPath, child), child))
        .filter((child): child is KunTreeNode => child !== null)

      return {
        name: baseName,
        label: docDirectoryLabelMap[baseName],
        path: path.relative(POSTS_PATH, currentPath).replace(/\\/g, '/'),
        children,
        type: 'directory',
      }
    }

    return null
  }

  return buildTree(POSTS_PATH, 'doc') as KunTreeNode
}
