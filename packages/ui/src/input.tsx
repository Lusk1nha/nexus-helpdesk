import * as React from "react"

import { cn } from "@nexus/utils"

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  error?: boolean
}

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, error, type, ...props }, ref) => (
    <input
      ref={ref}
      type={type}
      className={cn(
        "flex h-9 w-full",
        "bg-(--surface) text-(--fg) placeholder:text-(--muted)",
        "rounded-sm border border-(--border)",
        "px-3 py-1 font-mono text-sm",
        "transition-colors duration-100",
        "focus:border-(--accent) focus:ring-1 focus:ring-(--accent) focus:outline-none",
        "disabled:cursor-not-allowed disabled:opacity-40",
        error && "border-(--destructive) focus:ring-(--destructive)",
        className
      )}
      {...props}
    />
  )
)
Input.displayName = "Input"
