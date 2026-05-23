import { AlertCircle, CheckCircle2, Info, TriangleAlert } from "lucide-react"
import * as React from "react"

import { cn } from "./utils"

type AlertVariant = "info" | "success" | "warning" | "error"

const icons: Record<AlertVariant, React.ElementType> = {
  info: Info,
  success: CheckCircle2,
  warning: TriangleAlert,
  error: AlertCircle,
}

const styles: Record<AlertVariant, string> = {
  info: "border-(--accent) text-(--accent) bg-(--accent)/8",
  success: "border-(--success) text-(--success) bg-(--success)/8",
  warning: "border-(--warning) text-(--warning) bg-(--warning)/8",
  error: "border-(--destructive) text-(--destructive) bg-(--destructive)/8",
}

interface AlertProps {
  variant?: AlertVariant
  title?: string
  children: React.ReactNode
  className?: string
}

export function Alert({
  variant = "info",
  title,
  children,
  className,
}: AlertProps) {
  const Icon = icons[variant]

  return (
    <div
      role="alert"
      className={cn(
        "flex gap-3 rounded-sm border px-3 py-2.5 font-mono text-sm",
        styles[variant],
        className
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
