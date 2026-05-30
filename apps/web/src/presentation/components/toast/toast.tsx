import {
  CheckCircleIcon,
  InfoIcon,
  WarningIcon,
  XIcon,
} from "@phosphor-icons/react"
import { AnimatePresence, motion } from "motion/react"
import { useCallback, useRef, useState } from "react"
import { createPortal } from "react-dom"

import { cn } from "@nexus/utils"

import {
  ToastContext,
  type ToastInput,
  type ToastVariant,
} from "./toast-context"

interface Toast extends ToastInput {
  id: number
}

const VARIANT_STYLES: Record<
  ToastVariant,
  { border: string; icon: string; Icon: typeof InfoIcon }
> = {
  info: {
    border: "border-(--accent)/30",
    icon: "text-(--accent)",
    Icon: InfoIcon,
  },
  success: {
    border: "border-(--success)/30",
    icon: "text-(--success)",
    Icon: CheckCircleIcon,
  },
  warning: {
    border: "border-orange-400/30",
    icon: "text-orange-400",
    Icon: WarningIcon,
  },
}

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([])
  const nextId = useRef(0)

  const dismiss = useCallback((id: number) => {
    setToasts((prev) => prev.filter((t) => t.id !== id))
  }, [])

  const toast = useCallback(
    (input: ToastInput) => {
      const id = nextId.current++
      setToasts((prev) => [...prev, { ...input, id }])
      const duration = input.durationMs ?? 5000
      window.setTimeout(() => dismiss(id), duration)
    },
    [dismiss]
  )

  return (
    <ToastContext.Provider value={{ toast }}>
      {children}
      {createPortal(
        <div className="pointer-events-none fixed bottom-4 right-4 z-[100] flex w-full max-w-xs flex-col gap-2">
          <AnimatePresence>
            {toasts.map((t) => {
              const style = VARIANT_STYLES[t.variant ?? "info"]
              const Icon = style.Icon
              return (
                <motion.div
                  key={t.id}
                  layout
                  initial={{ opacity: 0, x: 24, scale: 0.96 }}
                  animate={{ opacity: 1, x: 0, scale: 1 }}
                  exit={{ opacity: 0, x: 24, scale: 0.96 }}
                  transition={{ duration: 0.2, ease: "easeOut" }}
                  className={cn(
                    "pointer-events-auto overflow-hidden rounded-sm border bg-(--surface) shadow-lg",
                    style.border
                  )}
                >
                  <div className="flex items-start gap-2.5 px-3.5 py-3">
                    <Icon className={cn("mt-0.5 h-4 w-4 shrink-0", style.icon)} />
                    <div className="min-w-0 flex-1">
                      <p className="font-mono text-xs font-medium text-(--fg)">
                        {t.title}
                      </p>
                      {t.description && (
                        <p className="mt-0.5 font-mono text-[11px] text-(--muted)">
                          {t.description}
                        </p>
                      )}
                    </div>
                    <button
                      onClick={() => dismiss(t.id)}
                      className="shrink-0 text-(--muted) transition-colors hover:text-(--fg)"
                    >
                      <XIcon className="h-3 w-3" />
                    </button>
                  </div>
                </motion.div>
              )
            })}
          </AnimatePresence>
        </div>,
        document.body
      )}
    </ToastContext.Provider>
  )
}
