import * as React from 'react'

import { cn } from '@/lib/utils'

export interface AvatarProps extends React.ImgHTMLAttributes<HTMLImageElement> {
  isBordered?: boolean
}

const Avatar = React.forwardRef<HTMLImageElement, AvatarProps>(
  ({ className, isBordered, alt = '', ...props }, ref) => {
    return (
      <img
        ref={ref}
        alt={alt}
        className={cn(
          'h-10 w-10 rounded-full object-cover',
          isBordered && 'ring-2 ring-ring ring-offset-2',
          className,
        )}
        {...props}
      />
    )
  },
)

Avatar.displayName = 'Avatar'

export { Avatar }
