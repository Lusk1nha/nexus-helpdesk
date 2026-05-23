import { motion, type Variants } from "motion/react"
import { CpuIcon } from "@phosphor-icons/react"

interface AboutProjectProps {
  variants: Variants
}

export function AboutProject({ variants }: AboutProjectProps) {
  return (
    <motion.div variants={variants} className="flex flex-col gap-4">
      <div className="flex items-center gap-2 border-b border-(--border) pb-2">
        <CpuIcon className="h-5 w-5 text-(--accent)" />
        <h2 className="font-mono text-xl font-semibold text-(--fg)">
          About Nexus_
        </h2>
      </div>
      <p className="text-sm leading-relaxed text-(--muted)">
        Nexus Helpdesk was born from the desire to solve complex
        architectural challenges. Moving away from expensive cloud APIs, it
        explores a <strong className="text-(--fg)">Local-First AI</strong>{" "}
        approach, running Large Language Models (like Llama-3/Phi-3)
        directly on the infrastructure via Ollama.
      </p>
      <p className="text-sm leading-relaxed text-(--muted)">
        Under the hood, it utilizes a blazingly fast{" "}
        <strong className="text-(--fg)">Rust (Axum)</strong> backend, a
        strict Multi-Tenant relational database, and an Event-Driven
        background worker system built with `tokio`. The frontend is a
        modern React 19 Monorepo designed for maximum performance and
        isolation.
      </p>
    </motion.div>
  )
}