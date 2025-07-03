import * as React from 'react'

import { cn } from '@/lib/utils'

export interface KbdProps extends React.HTMLAttributes<HTMLElement> {}

const Kbd = React.forwardRef<HTMLElement, KbdProps>(
  ({ className, ...props }, ref) => (
    <kbd
      ref={ref}
      className={cn(
        'bg-muted text-muted-foreground rounded border px-1.5 py-0.5 font-sans text-xs',
        className,
      )}
      {...props}
    />
  ),
)

Kbd.displayName = 'Kbd'

export { Kbd }
