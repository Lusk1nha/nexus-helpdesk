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
    "focus-visible:ring-2 focus-visible:ring-(--accent) focus-visible:ring-offset-1 focus-visible:ring-offset-(--bg) focus-visible:outline-none",
  ],
  {
    variants: {
      variant: {
        primary: [
          "border-(--accent) bg-(--accent) text-(--accent-fg)",
          "hover:opacity-85 active:scale-[0.98]",
        ],
        secondary: [
          "border-(--border) bg-(--surface) text-(--fg)",
          "hover:bg-(--surface-2) active:scale-[0.98]",
        ],
        ghost: [
          "border-transparent bg-transparent text-(--fg)",
          "hover:border-(--border) hover:bg-(--surface)",
        ],
        destructive: [
          "border-(--destructive) bg-(--destructive) text-(--destructive-fg)",
          "hover:opacity-85 active:scale-[0.98]",
        ],
        outline: [
          "border-(--accent) bg-transparent text-(--accent)",
          "hover:bg-(--accent) hover:text-(--accent-fg) active:scale-[0.98]",
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
