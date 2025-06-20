import type { FileInfo, Node } from '@/types'

import { notFound } from 'next/navigation'

import { Sidebar } from '@/components/sidebar'
import { FileList } from '@/components/fileList'
import { RoundArrowButton } from '@/components/returnButton'
import { GameIntro } from '@/components/gameIntro'

interface FolderInode {
  type: 'folder'
  data: Node[]
}

interface FileInode {
  type: 'file'
  name: string
  info: FileInfo
}

type InodeResponse = FolderInode | FileInode

export default async function BrowserPage({
  params,
}: {
  params: Promise<{ slug: string[] }>
}) {
  const slug = (await params).slug.map(decodeURIComponent)
  const encoded = slug.map(encodeURIComponent).join('/')
  const path = encoded ? `/files/${encoded}` : '/files'

  const serviceUrl = process.env.BACKEND_URL || 'http://localhost:2999'
  const res = await fetch(`${serviceUrl}${path}`)

  if (!res.ok) {
    notFound()
  }

  const data = (await res.json()) as InodeResponse

  return (
    <div className='mx-auto flex w-full max-w-3xl gap-2 sm:gap-4'>
      <RoundArrowButton />
      <Sidebar />
      <div className='w-full'>
        {data.type === 'file' ? (
          <GameIntro info={data.info} />
        ) : (
          <FileList inode={data.data} slug={slug} />
        )}
      </div>
    </div>
  )
}
