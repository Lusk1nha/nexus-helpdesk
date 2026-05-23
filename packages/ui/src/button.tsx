import { cva, type VariantProps } from "class-variance-authority"
import { Loader2 } from "lucide-react"
import * as React from "react"

import { cn } from "./utils"

const buttonVariants = cva(
  [
    "inline-flex items-center justify-center gap-2",
    "font-mono text-sm font-medium",
    "border transition-all duration-100",
    "cursor-pointer select-none",
    "disabled:pointer-events-none disabled:opacity-40",
    "focus-visible:ring-2 focus-visible:ring-[var(--accent)] focus-visible:ring-offset-1 focus-visible:ring-offset-[var(--bg)] focus-visible:outline-none",
  ],
  {
    variants: {
      variant: {
        primary: [
          "border-[var(--accent)] bg-[var(--accent)] text-[var(--accent-fg)]",
          "hover:opacity-85 active:scale-[0.98]",
        ],
        secondary: [
          "border-[var(--border)] bg-[var(--surface)] text-[var(--fg)]",
          "hover:bg-[var(--surface-2)] active:scale-[0.98]",
        ],
        ghost: [
          "border-transparent bg-transparent text-[var(--fg)]",
          "hover:border-[var(--border)] hover:bg-[var(--surface)]",
        ],
        destructive: [
          "border-[var(--destructive)] bg-[var(--destructive)] text-[var(--destructive-fg)]",
          "hover:opacity-85 active:scale-[0.98]",
        ],
        outline: [
          "border-[var(--accent)] bg-transparent text-[var(--accent)]",
          "hover:bg-[var(--accent)] hover:text-[var(--accent-fg)] active:scale-[0.98]",
        ],
      },
      size: {
        sm: "h-7 rounded-sm px-3 text-xs",
        md: "h-9 rounded-sm px-4",
        lg: "h-11 rounded-sm px-6 text-base",
        icon: "h-9 w-9 rounded-sm",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  }
)

export interface ButtonProps
  extends
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  loading?: boolean
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    { className, variant, size, loading, disabled, children, ...props },
    ref
  ) => (
    <button
      ref={ref}
      className={cn(buttonVariants({ variant, size }), className)}
      disabled={disabled || loading}
      {...props}
    >
      {loading && <Loader2 className="h-3.5 w-3.5 animate-spin" />}
      {children}
    </button>
  )
)
Button.displayName = "Button"
