import { motion, type Variants } from "motion/react"
import { FlaskIcon } from "@phosphor-icons/react"

interface AboutDisclaimerProps {
  variants: Variants
}

export function AboutDisclaimer({ variants }: AboutDisclaimerProps) {
  return (
    <motion.div
      variants={variants}
      className="flex items-center gap-3 rounded-sm border border-(--warning)/30 bg-(--warning)/6 px-4 py-3"
    >
      <FlaskIcon className="h-3.5 w-3.5 shrink-0 text-(--warning)" />
      <p className="font-mono text-xs text-(--warning)/80">
        <span className="font-semibold text-(--warning)">Portfolio / POC</span>
        {" — "}
        Built to demonstrate Multi-Tenancy (DDD), Event-Driven Architecture in
        Rust, and Local-First AI. Not a commercial product.
      </p>
    </motion.div>
  )
}
