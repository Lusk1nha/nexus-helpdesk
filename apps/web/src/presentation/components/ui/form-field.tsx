import * as React from 'react'

import { cn } from '@/lib/utils'
import { Label } from './label'

interface FormFieldProps {
  label: string
  htmlFor?: string
  error?: string
  required?: boolean
  className?: string
  children: React.ReactNode
}

export function FormField({
  label,
  htmlFor,
  error,
  required,
  className,
  children,
}: FormFieldProps) {
  return (
    <div className={cn('flex flex-col gap-1.5', className)}>
      <Label htmlFor={htmlFor} required={required}>
        {label}
      </Label>
      {children}
      {error && (
        <p className="text-xs text-[var(--destructive)] font-mono mt-0.5">
          {error}
        </p>
      )}
    </div>
  )
}
