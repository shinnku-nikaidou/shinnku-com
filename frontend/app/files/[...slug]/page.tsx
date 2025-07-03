import type { FileInfo, Node } from '@/types'

import { notFound } from 'next/navigation'

import { FileList } from '@/components/files/FileList'
import { GameIntro } from '@/components/files/GameIntro'
import { RoundArrowButton } from '@/components/files/RoundArrowButton'
import { Sidebar } from '@/components/files/Sidebar'

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
  const url = `${serviceUrl}${path}`
  const res = await fetch(url)

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
