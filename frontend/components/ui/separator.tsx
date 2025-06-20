import * as React from 'react'

import { cn } from '@/lib/utils'

export interface SeparatorProps extends React.HTMLAttributes<HTMLDivElement> {
  orientation?: 'horizontal' | 'vertical'
  decorative?: boolean
}

const Separator = React.forwardRef<HTMLDivElement, SeparatorProps>(
  (
    { className, orientation = 'horizontal', decorative = true, ...props },
    ref,
  ) => (
    <div
      ref={ref}
      aria-orientation={orientation}
      role='separator'
      {...(decorative ? {} : { 'aria-orientation': orientation })}
      className={cn(
        'shrink-0 bg-divider',
        orientation === 'horizontal' ? 'h-px w-full' : 'h-full w-px',
        className,
      )}
      {...props}
    />
  ),
)

Separator.displayName = 'Separator'

export { Separator }
