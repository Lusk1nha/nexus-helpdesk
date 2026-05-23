import { AlertCircle, CheckCircle2, Info, TriangleAlert } from 'lucide-react'
import * as React from 'react'

import { cn } from '@/lib/utils'

type AlertVariant = 'info' | 'success' | 'warning' | 'error'

const icons: Record<AlertVariant, React.ElementType> = {
  info: Info,
  success: CheckCircle2,
  warning: TriangleAlert,
  error: AlertCircle,
}

const styles: Record<AlertVariant, string> = {
  info: 'border-[var(--accent)] text-[var(--accent)] bg-[var(--accent)]/8',
  success: 'border-[var(--success)] text-[var(--success)] bg-[var(--success)]/8',
  warning: 'border-[var(--warning)] text-[var(--warning)] bg-[var(--warning)]/8',
  error: 'border-[var(--destructive)] text-[var(--destructive)] bg-[var(--destructive)]/8',
}

interface AlertProps {
  variant?: AlertVariant
  title?: string
  children: React.ReactNode
  className?: string
}

export function Alert({ variant = 'info', title, children, className }: AlertProps) {
  const Icon = icons[variant]

  return (
    <div
      role="alert"
      className={cn(
        'flex gap-3 rounded-sm border px-3 py-2.5 font-mono text-sm',
        styles[variant],
        className,
      )}
    >
      <Icon className="mt-0.5 h-4 w-4 shrink-0" />
      <div className="flex flex-col gap-0.5">
        {title && <span className="font-semibold">{title}</span>}
        <span className="opacity-90">{children}</span>
      </div>
    </div>
  )
}
