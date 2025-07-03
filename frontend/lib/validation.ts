import { z } from 'zod'

export const FileInfoSchema = z.object({
  file_path: z.string(),
  upload_timestamp: z.number(),
  file_size: z.number(),
})
export type FileInfo = z.infer<typeof FileInfoSchema>
export const SearchItemSchema = z.object({
  id: z.string(),
  info: FileInfoSchema,
})
export type SearchItem = z.infer<typeof SearchItemSchema>
export const SearchListSchema = z.array(SearchItemSchema)
export type SearchList = z.infer<typeof SearchListSchema>
export const BlogFrontmatterSchema = z.object({
  title: z.string(),
  banner: z.string(),
  description: z.string(),
  date: z
    .union([z.string(), z.date()])
    .transform((v) => (v instanceof Date ? v.toISOString() : v)),
  authorUid: z.number(),
  authorName: z.string(),
  authorAvatar: z.string(),
  authorHomepage: z.string(),
})
export type BlogFrontmatter = z.infer<typeof BlogFrontmatterSchema>
