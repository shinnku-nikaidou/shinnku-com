import type { BucketFiles, Config, SearchList, TreeNode } from '@/types'

import { promises as fs } from 'fs'

import toml from 'toml'

import { generateTree } from '@/algorithm/tree'
import { aggregate_builder } from '@/algorithm/search'

export const shinnku_bucket_files_json: BucketFiles = JSON.parse(
  await fs.readFile('data/shinnku_bucket_files.json', { encoding: 'utf8' }),
)

export const galgame0_bucket_files_json: BucketFiles = JSON.parse(
  await fs.readFile('data/galgame0_bucket_files.json', { encoding: 'utf8' }),
)

export const shinnku_tree = generateTree(shinnku_bucket_files_json)
export const galgame0_tree = generateTree(galgame0_bucket_files_json)

export const search_index: SearchList = aggregate_builder(
  shinnku_bucket_files_json,
  galgame0_bucket_files_json.filter((v) =>
    v.file_path.startsWith('合集系列/浮士德galgame游戏合集'),
  ),
)

export const tree = {
  shinnku: shinnku_tree,
  galgame0: (galgame0_tree['合集系列'] as TreeNode)[
    '浮士德galgame游戏合集'
  ] as TreeNode,
}

export const config: Config = toml.parse(
  await fs.readFile('config.toml', {
    encoding: 'utf8',
  }),
)
