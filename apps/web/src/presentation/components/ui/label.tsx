import * as React from 'react'

import { cn } from '@/lib/utils'

export interface LabelProps extends React.LabelHTMLAttributes<HTMLLabelElement> {
  required?: boolean
}

export const Label = React.forwardRef<HTMLLabelElement, LabelProps>(
  ({ className, required, children, ...props }, ref) => (
    <label
      ref={ref}
      className={cn(
        'text-xs font-mono font-medium text-[var(--muted)] uppercase tracking-widest',
        'flex items-center gap-1',
        className,
      )}
      {...props}
    >
      <span className="text-[var(--accent)] select-none">{'>'}</span>
      {children}
      {required && <span className="text-[var(--destructive)]">*</span>}
    </label>
  ),
)
Label.displayName = 'Label'
