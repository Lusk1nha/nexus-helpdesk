import { motion } from "motion/react"
import { useEffect } from "react"
import { CheckCircleIcon, SpinnerGapIcon } from "@phosphor-icons/react"

import { workspaceUrl } from "@/env"

interface Props {
  tenantSlug: string
  /** Delay before redirect (ms). Lets the user see the success state. */
  delayMs?: number
}

/**
 * Shown for ~1.5s after a successful registration before we hand the user
 * off to their newly-provisioned subdomain. Without this the user sees a
 * blank screen on slow connections while DNS resolves the new host.
 */
export function SuccessSplash({ tenantSlug, delayMs = 1500 }: Props) {
  const target = workspaceUrl(tenantSlug, "/login?registered=true")

  useEffect(() => {
    const id = setTimeout(() => {
      window.location.href = target
    }, delayMs)
    return () => clearTimeout(id)
  }, [target, delayMs])

  return (
    <motion.div
      role="status"
      aria-live="polite"
      initial={{ opacity: 0, scale: 0.96 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.25, ease: "easeOut" }}
      className="flex w-full max-w-md flex-col items-center gap-6 rounded-sm border border-(--success)/40 bg-(--surface) p-10 text-center"
    >
      <motion.div
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{ delay: 0.1, type: "spring", stiffness: 200, damping: 12 }}
      >
        <CheckCircleIcon weight="fill" className="h-14 w-14 text-(--success)" />
      </motion.div>

      <div className="space-y-1">
        <p className="font-mono text-xs text-(--muted)">
          <span className="text-(--success)">$</span> nexus provision --ok
        </p>
        <h2 className="font-mono text-xl font-semibold text-(--fg)">
          Workspace ready
        </h2>
        <p className="text-sm text-(--muted)">
          <span className="font-mono text-(--accent)">{tenantSlug}</span> has
          been provisioned successfully.
        </p>
      </div>

      <div className="flex items-center gap-2 font-mono text-xs text-(--muted)">
        <SpinnerGapIcon className="h-3.5 w-3.5 animate-spin" />
        redirecting to your dashboard…
      </div>

      <a
        href={target}
        className="font-mono text-xs text-(--accent) underline-offset-2 hover:underline"
      >
        click here if you are not redirected automatically
      </a>
    </motion.div>
  )
}
