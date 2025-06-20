import type { Config } from '@/types'

import { promises as fs } from 'fs'

import toml from 'toml'

export const config: Config = toml.parse(
  await fs.readFile('config.toml', {
    encoding: 'utf8',
  }),
)
