import { motion, type Variants } from "motion/react"
import { WarningIcon } from "@phosphor-icons/react"

interface AboutDisclaimerProps {
  variants: Variants
}

export function AboutDisclaimer({ variants }: AboutDisclaimerProps) {
  return (
    <motion.div
      variants={variants}
      className="flex items-start gap-4 rounded-md border border-(--warning)/50 bg-(--warning)/10 p-4"
    >
      <WarningIcon className="mt-0.5 h-5 w-5 shrink-0 text-(--warning)" />
      <div>
        <h3 className="font-mono text-sm font-semibold text-(--warning)">
          Proof of Concept & Portfolio Project
        </h3>
        <p className="mt-1 text-sm text-(--warning)/80">
          Nexus Helpdesk is a test application built to demonstrate advanced
          software engineering concepts, including Multi-Tenancy (DDD),
          Event-Driven Architecture in Rust, and Local-First AI integrations.
          It is not a commercial product intended for production use.
        </p>
      </div>
    </motion.div>
  )
}