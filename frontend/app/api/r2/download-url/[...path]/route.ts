import { NextRequest, NextResponse } from 'next/server'
import { S3Client, GetObjectCommand } from '@aws-sdk/client-s3'
import { getSignedUrl } from '@aws-sdk/s3-request-presigner'

// Initialize S3 client for Cloudflare R2
const s3Client = new S3Client({
  region: process.env.S3_REGION || 'auto',
  endpoint: process.env.S3_ENDPOINT,
  credentials: {
    accessKeyId: process.env.S3_ACCESS_KEY_ID!,
    secretAccessKey: process.env.S3_SECRET_ACCESS_KEY!,
  },
  forcePathStyle: false, // Use virtual-hosted style for R2 custom domain
})

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ path: string[] }> },
) {
  try {
    const { path } = await params

    if (!path || path.length === 0) {
      return new Response('Missing file path', { status: 400 })
    }

    // Decode URL path parameters
    const decodedPath = path.map(decodeURIComponent)
    let fullPath = decodedPath.join('/')

    // Handle path mapping for galgame0 (合集系列) files
    // Frontend receives: galgame0/1/1995年1月/...
    // But R2 storage has: `【合集系列】/浮士德galgame游戏合集/1/1995年1月/...
    if (decodedPath[0] === 'galgame0') {
      const relativePath = decodedPath.slice(1).join('/')
      fullPath = `\`【合集系列】/浮士德galgame游戏合集/${relativePath}`
    }

    console.log('Original path:', decodedPath.join('/'))
    console.log('Mapped R2 path:', fullPath)

    // Generate presigned URL for Cloudflare R2
    const command = new GetObjectCommand({
      Bucket: process.env.S3_BUCKET_NAME!,
      Key: fullPath,
      ResponseContentDisposition: `attachment; filename="${encodeURIComponent(decodedPath[decodedPath.length - 1])}"`,
    })

    const presignedUrl = await getSignedUrl(s3Client, command, {
      expiresIn: 14400, // 4 hours expiration
    })

    console.log('Generated presigned URL:', presignedUrl)

    // 302 redirect to presigned URL
    return NextResponse.redirect(presignedUrl, 302)
  } catch (error) {
    console.error('Error generating download URL:', error)

    if (error instanceof Error) {
      if (error.name === 'NoSuchKey') {
        return new Response('File not found', { status: 404 })
      }
      if (error.name === 'AccessDenied') {
        return new Response('Access denied', { status: 403 })
      }
    }

    return new Response('Internal server error', { status: 500 })
  }
}
