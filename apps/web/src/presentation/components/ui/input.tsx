import * as React from 'react'

import { cn } from '@/lib/utils'

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  error?: boolean
}

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, error, type, ...props }, ref) => (
    <input
      ref={ref}
      type={type}
      className={cn(
        'flex h-9 w-full',
        'bg-[var(--surface)] text-[var(--fg)] placeholder:text-[var(--muted)]',
        'border border-[var(--border)] rounded-sm',
        'px-3 py-1 text-sm font-mono',
        'transition-colors duration-100',
        'focus:outline-none focus:border-[var(--accent)] focus:ring-1 focus:ring-[var(--accent)]',
        'disabled:opacity-40 disabled:cursor-not-allowed',
        error && 'border-[var(--destructive)] focus:ring-[var(--destructive)]',
        className,
      )}
      {...props}
    />
  ),
)
Input.displayName = 'Input'
