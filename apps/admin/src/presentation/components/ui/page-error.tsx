import { ArrowCounterClockwiseIcon, WifiSlashIcon } from "@phosphor-icons/react"
import { motion } from "motion/react"

import { cn } from "@nexus/utils"

interface Props {
  error: Error & { code?: string; status?: number }
  retry: () => void
}

export function PageError({ error, retry }: Props) {
  const isNetwork =
    error.message.toLowerCase().includes("network") ||
    error.message.toLowerCase().includes("fetch") ||
    error.name === "TypeError"

  const isAuth = (error as any).status === 401 || (error as any).status === 403

  const title = isAuth
    ? "access denied"
    : isNetwork
      ? "connection error"
      : "something went wrong"

  const description = isAuth
    ? "You don't have permission to view this page."
    : isNetwork
      ? "Could not reach the server. Check your connection and try again."
      : error.message || "An unexpected error occurred."

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="flex min-h-[55vh] flex-col items-center justify-center gap-5"
    >
      {/* Icon */}
      <div
        className={cn(
          "flex h-14 w-14 items-center justify-center rounded-sm border",
          isNetwork
            ? "border-(--warning)/30 bg-(--warning)/5"
            : "border-(--destructive)/30 bg-(--destructive)/5"
        )}
      >
        {isNetwork ? (
          <WifiSlashIcon
            className="h-6 w-6 text-(--warning)"
            weight="duotone"
          />
        ) : (
          <span
            className={cn(
              "font-mono text-2xl font-semibold",
              isAuth ? "text-(--accent)" : "text-(--destructive)"
            )}
          >
            ◈
          </span>
        )}
      </div>

      {/* Text */}
      <div className="text-center">
        <p className="font-mono text-sm font-semibold text-(--fg)">{title}</p>
        <p className="mt-1.5 max-w-xs font-mono text-xs text-(--muted)">
          {description}
        </p>
        {(error as any).code && (
          <p className="mt-2 font-mono text-[10px] text-(--border)">
            code: {(error as any).code}
          </p>
        )}
      </div>

      {/* Retry */}
      {!isAuth && (
        <button
          onClick={retry}
          className={cn(
            "flex items-center gap-2 rounded-sm border border-(--border) px-4 py-2",
            "font-mono text-xs text-(--muted) transition-colors",
            "hover:border-(--accent)/40 hover:text-(--fg)"
          )}
        >
          <ArrowCounterClockwiseIcon className="h-3.5 w-3.5" />
          try again
        </button>
      )}
    </motion.div>
  )
}
