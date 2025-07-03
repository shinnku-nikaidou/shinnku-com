import { NextResponse } from 'next/server'

type Data = {
  message: string
}

export async function GET() {
  const data: Data = { message: 'Hello by shinnku' }

  return NextResponse.json(data)
}
