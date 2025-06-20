export interface BlogPostMetadata {
  title: string
  banner: string
  date: string
  description: string
  textCount: number
  slug: string
  path: string
}

export interface BlogTreeNode {
  name: string
  label: string
  path: string
  children?: BlogTreeNode[]
  type: 'file' | 'directory'
}

export interface BlogFrontmatter {
  title: string
  banner: string
  description: string
  date: string
  authorUid: number
  authorName: string
  authorAvatar: string
  authorHomepage: string
}

export interface Blog {
  slug: string
  content: string
  frontmatter: BlogFrontmatter
}
