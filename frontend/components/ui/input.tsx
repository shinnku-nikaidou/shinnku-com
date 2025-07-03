import * as React from 'react'

import { cn } from '@/lib/utils'

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type = 'text', ...props }, ref) => {
    return (
      <input
        ref={ref}
        className={cn(
          'border-input bg-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-12 w-full rounded-full border px-4 text-base focus-visible:ring-1 focus-visible:outline-hidden disabled:cursor-not-allowed disabled:opacity-50',
          className,
        )}
        type={type}
        {...props}
      />
    )
  },
)

Input.displayName = 'Input'

export { Input }
