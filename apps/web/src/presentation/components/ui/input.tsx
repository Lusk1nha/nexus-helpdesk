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
        'rounded-sm border border-[var(--border)]',
        'px-3 py-1 font-mono text-sm',
        'transition-colors duration-100',
        'focus:border-[var(--accent)] focus:ring-1 focus:ring-[var(--accent)] focus:outline-none',
        'disabled:cursor-not-allowed disabled:opacity-40',
        error && 'border-[var(--destructive)] focus:ring-[var(--destructive)]',
        className,
      )}
      {...props}
    />
  ),
)
Input.displayName = 'Input'
