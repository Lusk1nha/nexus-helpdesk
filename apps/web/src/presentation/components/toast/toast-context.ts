import { createContext, useContext } from "react"

export type ToastVariant = "info" | "success" | "warning"

export interface ToastInput {
  title: string
  description?: string
  variant?: ToastVariant
  /** Auto-dismiss after this many ms (default 5000). */
  durationMs?: number
}

export interface ToastContextValue {
  toast: (input: ToastInput) => void
}

export const ToastContext = createContext<ToastContextValue | null>(null)

export function useToast(): ToastContextValue {
  const ctx = useContext(ToastContext)
  if (!ctx) {
    throw new Error("useToast must be used within a ToastProvider")
  }
  return ctx
}
