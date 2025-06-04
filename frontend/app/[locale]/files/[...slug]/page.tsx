import { notFound } from 'next/navigation'

import { checknodevariety, node2list } from '@/algorithm/tree'
import { Sidebar } from '@/components/sidebar'
import { FileList } from '@/components/fileList'
import { tree } from '@/config/root'
import { RoundArrowButton } from '@/components/returnButton'
import { GameIntro } from '@/components/gameIntro'

export default async function BrowserPage({
  params,
}: {
  params: Promise<{ slug: string[] }>
}) {
  const origin_slug = (await params).slug
  const slug = origin_slug.map(decodeURIComponent)

  origin_slug.pop()
  let node: any = tree

  try {
    for (const key of slug) {
      node = node[key]
    }
  } catch {
    notFound()
  }

  let variety = checknodevariety(node)

  if (variety === '404') {
    notFound()
  }

  const inode = node2list(node)

  return (
    <div className='mx-auto flex w-full max-w-3xl gap-2 sm:gap-4'>
      <RoundArrowButton />
      <Sidebar />
      <div className='w-full'>
        {variety === 'file' ? (
          <GameIntro info={node} />
        ) : (
          <FileList inode={inode} slug={slug} />
        )}
      </div>
    </div>
  )
}
