import { motion } from "motion/react"
import { useEffect, useState } from "react"
import {
  CheckIcon,
  SpinnerGapIcon,
  ArrowRightIcon,
} from "@phosphor-icons/react"

import { workspaceUrl } from "@/env"

interface Props {
  tenantSlug: string
  delayMs?: number
}

const STEPS = [
  "Migrating database schema",
  "Creating admin account",
  "Configuring workspace settings",
  "Linking subdomain",
  "Starting AI engine",
]

export function SuccessSplash({ tenantSlug, delayMs = 3200 }: Props) {
  const target = workspaceUrl(tenantSlug, "/login?registered=true")
  const [completedSteps, setCompletedSteps] = useState(0)
  const [redirecting, setRedirecting] = useState(false)

  // Tick through steps, then redirect
  useEffect(() => {
    const stepDelay = delayMs / (STEPS.length + 1)

    const timers: ReturnType<typeof setTimeout>[] = STEPS.map((_, i) =>
      setTimeout(() => setCompletedSteps(i + 1), stepDelay * (i + 1))
    )

    const redirectTimer = setTimeout(() => {
      setRedirecting(true)
      setTimeout(() => {
        window.location.href = target
      }, 600)
    }, delayMs)

    return () => {
      timers.forEach(clearTimeout)
      clearTimeout(redirectTimer)
    }
  }, [target, delayMs])

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.97 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.25, ease: "easeOut" }}
      className="w-full max-w-md"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface) shadow-xl shadow-black/10">
        {/* Window chrome */}
        <div className="flex items-center gap-2 border-b border-(--border) bg-(--surface-2) px-4 py-3">
          <span className="h-2.5 w-2.5 rounded-full bg-(--destructive) opacity-60" />
          <span className="h-2.5 w-2.5 rounded-full bg-(--warning) opacity-60" />
          <span className="h-2.5 w-2.5 rounded-full bg-(--success) opacity-60" />
          <span className="ml-3 font-mono text-[11px] text-(--muted)">
            nexus provision — {tenantSlug}
          </span>
        </div>

        <div className="space-y-3 px-4 py-5 font-mono text-sm sm:px-6 sm:py-6">
          {/* Prompt line */}
          <p className="text-xs text-(--muted)">
            <span className="mr-2 text-(--accent)">$</span>
            nexus provision --workspace {tenantSlug}
          </p>

          {/* Steps */}
          <div className="space-y-2 pt-1">
            {STEPS.map((step, i) => {
              const done = completedSteps > i
              const active = completedSteps === i

              return (
                <motion.div
                  key={step}
                  initial={{ opacity: 0, x: -8 }}
                  animate={{ opacity: active || done ? 1 : 0.25, x: 0 }}
                  transition={{ delay: i * 0.05, duration: 0.2 }}
                  className="flex items-center gap-3"
                >
                  <div className="flex h-4 w-4 shrink-0 items-center justify-center">
                    {done ? (
                      <motion.div
                        initial={{ scale: 0 }}
                        animate={{ scale: 1 }}
                        transition={{
                          type: "spring",
                          stiffness: 300,
                          damping: 15,
                        }}
                      >
                        <CheckIcon
                          weight="bold"
                          className="h-3.5 w-3.5 text-(--success)"
                        />
                      </motion.div>
                    ) : active ? (
                      <SpinnerGapIcon className="h-3.5 w-3.5 animate-spin text-(--muted)" />
                    ) : (
                      <span className="h-1 w-1 rounded-full bg-(--border)" />
                    )}
                  </div>
                  <span
                    className={
                      done
                        ? "text-(--fg)"
                        : active
                          ? "text-(--muted)"
                          : "text-(--border)"
                    }
                  >
                    {step}
                    {done && (
                      <span className="ml-2 text-xs text-(--success)">✓</span>
                    )}
                  </span>
                </motion.div>
              )
            })}
          </div>

          {/* Redirect line */}
          {completedSteps === STEPS.length && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.3 }}
              className="space-y-3 border-t border-(--border) pt-3"
            >
              <p className="text-xs text-(--success)">
                <span className="mr-2">✓</span>
                Workspace <span className="font-semibold">
                  {tenantSlug}
                </span>{" "}
                provisioned successfully.
              </p>

              {redirecting ? (
                <div className="flex items-center gap-2 text-xs text-(--muted)">
                  <SpinnerGapIcon className="h-3.5 w-3.5 animate-spin" />
                  Redirecting to your dashboard…
                </div>
              ) : (
                <div className="flex items-center gap-2 text-xs text-(--muted)">
                  <ArrowRightIcon className="h-3.5 w-3.5 text-(--accent)" />
                  <a
                    href={target}
                    className="text-(--accent) underline-offset-2 hover:underline"
                  >
                    {tenantSlug}.nexus.com/login
                  </a>
                </div>
              )}
            </motion.div>
          )}
        </div>
      </div>
    </motion.div>
  )
}
