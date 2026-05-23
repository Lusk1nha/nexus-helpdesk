import { motion, type Variants } from "motion/react"
import { Link } from "react-router"
import { ArrowRight, Cpu, ShieldCheck, Zap } from "lucide-react"

import { buttonVariants } from "@nexus/ui"

export function LandingPage() {
  const container: Variants = {
    hidden: { opacity: 0 },
    show: {
      opacity: 1,
      transition: { staggerChildren: 0.15 },
    },
  }

  const item: Variants = {
    hidden: { opacity: 0, y: 20 },
    show: { opacity: 1, y: 0, transition: { duration: 0.4, ease: "easeOut" } },
  }

  return (
    <motion.div
      variants={container}
      initial="hidden"
      animate="show"
      className="flex w-full max-w-5xl flex-col items-center text-center"
    >
      {/* Etiqueta / Status */}
      <motion.div
        variants={item}
        className="mb-6 flex items-center gap-2 rounded-full border border-(--accent)/30 bg-(--accent)/10 px-3 py-1 font-mono text-xs text-(--accent)"
      >
        <span className="relative flex h-2 w-2">
          <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-(--accent) opacity-75"></span>
          <span className="relative inline-flex h-2 w-2 rounded-full bg-(--accent)"></span>
        </span>
        nexus_engine v1.0 online
      </motion.div>

      {/* Hero Section */}
      <motion.h1
        variants={item}
        className="mb-6 max-w-3xl font-mono text-4xl font-bold tracking-tight text-(--fg) sm:text-5xl lg:text-6xl"
      >
        Support at the speed of <br className="hidden sm:block" />
        <span className="text-(--accent)">Local-First AI.</span>
      </motion.h1>

      <motion.p
        variants={item}
        className="mb-10 max-w-2xl text-lg text-(--muted) sm:text-xl"
      >
        The multi-tenant helpdesk built for privacy and performance. Powered by
        a blazingly fast Rust backend and zero-cost local LLMs.
      </motion.p>

      {/* CTA Button */}
      <motion.div variants={item} className="mb-20">
        <Link
          className={buttonVariants({
            size: "lg",
            className: "h-12 px-8 font-mono text-base",
          })}
          to="/register"
        >
          Initialize Workspace <ArrowRight className="ml-2 h-4 w-4" />
        </Link>
      </motion.div>

      {/* Features Grid */}
      <motion.div
        variants={item}
        className="grid w-full gap-6 text-left sm:grid-cols-3"
      >
        {/* Feature 1 */}
        <div className="rounded-md border border-(--border) bg-(--surface) p-6 transition-colors hover:border-(--accent)/50">
          <Cpu className="mb-4 h-8 w-8 text-(--accent)" />
          <h3 className="mb-2 font-mono text-lg font-semibold text-(--fg)">
            Agentic RAG
          </h3>
          <p className="text-sm text-(--muted)">
            Automated context retrieval using Qdrant and Ollama. Draft responses
            are generated locally, ensuring sensitive data never leaves your
            infrastructure.
          </p>
        </div>

        {/* Feature 2 */}
        <div className="rounded-md border border-(--border) bg-(--surface) p-6 transition-colors hover:border-(--accent)/50">
          <ShieldCheck className="mb-4 h-8 w-8 text-(--success)" />
          <h3 className="mb-2 font-mono text-lg font-semibold text-(--fg)">
            Strict Multi-Tenancy
          </h3>
          <p className="text-sm text-(--muted)">
            Enterprise-grade isolation. Data is logically separated at the
            database level with shared Unit of Work abstractions in Rust.
          </p>
        </div>

        {/* Feature 3 */}
        <div className="rounded-md border border-(--border) bg-(--surface) p-6 transition-colors hover:border-(--accent)/50">
          <Zap className="mb-4 h-8 w-8 text-(--warning)" />
          <h3 className="mb-2 font-mono text-lg font-semibold text-(--fg)">
            Event-Driven
          </h3>
          <p className="text-sm text-(--muted)">
            Built on Axum and Tokio. Asynchronous processing via `mpsc` queues
            means the interface never blocks while the AI generates answers.
          </p>
        </div>
      </motion.div>
    </motion.div>
  )
}
