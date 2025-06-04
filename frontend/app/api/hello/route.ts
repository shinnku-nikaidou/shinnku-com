import { NextResponse, NextRequest } from 'next/server'

type Data = {
  message: string
}

export async function GET(_req: NextRequest) {
  const data: Data = { message: 'Hello by shinnku' }

  return NextResponse.json(data)
}
