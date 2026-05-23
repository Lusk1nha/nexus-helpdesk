import { motion, type Variants } from "motion/react"
import { Link } from "react-router"
import {
  ArrowRightIcon,
  CpuIcon,
  ShieldCheckIcon,
  LightningIcon,
  TerminalWindowIcon,
} from "@phosphor-icons/react"

import { buttonVariants } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { TitleTypingAnimation } from "../components/title-typing"

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
      className="relative flex w-full max-w-5xl flex-col items-center text-center"
    >
      {/* 
        Hero Glow 
        Usa o --accent do tema atual. O blur e a opacidade baixa garantem 
        que fique bom tanto em temas claros (Dawn) quanto escuros (OLED/Cyberpunk).
      */}
      <div className="pointer-events-none absolute top-1/4 left-1/2 -z-10 h-64 w-full max-w-2xl -translate-x-1/2 rounded-full bg-(--accent)/15 blur-[100px]" />

      {/* Etiqueta / Status */}
      <motion.div
        variants={item}
        className="mb-8 flex items-center gap-2 rounded-full border border-(--accent)/30 bg-(--accent)/10 px-4 py-1.5 font-mono text-xs font-medium text-(--accent) backdrop-blur-md"
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
        className="mb-6 max-w-4xl font-mono text-4xl font-bold tracking-tight text-(--fg) sm:text-5xl"
      >
        Support at the speed of <br className="hidden sm:block" />
        <span className="relative inline-flex flex-col text-(--accent) sm:flex-row">
          <TitleTypingAnimation
            delay={500}
            texts={[
              "Local-First AI.",
              "Zero-Cost LLMs.",
              "Agentic RAG.",
              "Rust Performance.",
            ]}
          />
          {/* Sublinhado Decorativo 
        A largura dele agora acompanha o texto sendo digitado! 
    */}
          <span className="absolute -bottom-2 left-0 h-1.5 w-full rounded-full bg-(--accent)/30 transition-all duration-100" />
        </span>
      </motion.h1>

      <motion.p
        variants={item}
        className="mb-8 max-w-2xl text-lg leading-relaxed text-(--muted) sm:text-xl"
      >
        The multi-tenant helpdesk built for privacy and performance. Powered by
        a blazingly fast{" "}
        <strong className="font-semibold text-(--fg)">Rust</strong> backend and
        zero-cost{" "}
        <strong className="font-semibold text-(--fg)">local LLMs</strong>.
      </motion.p>

      {/* Tech Stack Badges (Usando --surface e --surface-2 para profundidade) */}
      <motion.div
        variants={item}
        className="mb-10 flex flex-wrap justify-center gap-3"
      >
        {["Rust (Axum)", "React 19", "Ollama", "Qdrant Vector DB"].map(
          (tech) => (
            <span
              key={tech}
              className="rounded-md border border-(--border) bg-(--surface) px-3 py-1.5 font-mono text-xs tracking-wider text-(--muted) uppercase shadow-sm transition-colors hover:border-(--accent)/50 hover:text-(--fg)"
            >
              {tech}
            </span>
          )
        )}
      </motion.div>

      {/* CTA Buttons */}
      <motion.div
        variants={item}
        className="mb-24 flex w-full flex-col items-center justify-center gap-4 sm:w-auto sm:flex-row"
      >
        <Link
          className={cn(
            buttonVariants({ size: "lg" }),
            "h-12 w-full px-8 font-mono text-base shadow-sm sm:w-auto"
          )}
          to="/register"
        >
          Initialize Workspace <ArrowRightIcon className="ml-2 h-4 w-4" />
        </Link>
        <Link
          className={cn(
            buttonVariants({ variant: "outline", size: "lg" }),
            "h-12 w-full bg-(--surface) px-8 font-mono text-base transition-colors hover:bg-(--surface-2) sm:w-auto"
          )}
          to="/about"
        >
          <TerminalWindowIcon className="mr-2 h-5 w-5" /> Explore Architecture
        </Link>
      </motion.div>

      {/* Features Grid */}
      <motion.div
        variants={item}
        className="grid w-full gap-6 text-left md:grid-cols-3"
      >
        {/* Feature 1 */}
        <motion.div
          whileHover={{ y: -5 }}
          transition={{ duration: 0.2 }}
          className="group flex flex-col rounded-xl border border-(--border) bg-(--surface) p-6 transition-colors hover:border-(--accent)/60 hover:bg-(--surface-2)"
        >
          <div className="mb-5 inline-flex self-start rounded-lg bg-(--accent)/10 p-3 transition-colors group-hover:bg-(--accent)/20">
            <CpuIcon className="h-6 w-6 text-(--accent)" />
          </div>
          <h3 className="mb-3 font-mono text-lg font-semibold text-(--fg)">
            Agentic RAG
          </h3>
          <p className="text-sm leading-relaxed text-(--muted) transition-colors group-hover:text-(--fg)/90">
            Automated context retrieval using Qdrant and Ollama. Draft responses
            are generated locally, ensuring sensitive data never leaves your
            infrastructure.
          </p>
        </motion.div>

        {/* Feature 2 */}
        <motion.div
          whileHover={{ y: -5 }}
          transition={{ duration: 0.2 }}
          className="group flex flex-col rounded-xl border border-(--border) bg-(--surface) p-6 transition-colors hover:border-(--success)/60 hover:bg-(--surface-2)"
        >
          <div className="mb-5 inline-flex self-start rounded-lg bg-(--success)/10 p-3 transition-colors group-hover:bg-(--success)/20">
            <ShieldCheckIcon className="h-6 w-6 text-(--success)" />
          </div>
          <h3 className="mb-3 font-mono text-lg font-semibold text-(--fg)">
            Strict Multi-Tenancy
          </h3>
          <p className="text-sm leading-relaxed text-(--muted) transition-colors group-hover:text-(--fg)/90">
            Enterprise-grade isolation. Data is logically separated at the
            database level with shared Unit of Work abstractions in Rust.
          </p>
        </motion.div>

        {/* Feature 3 */}
        <motion.div
          whileHover={{ y: -5 }}
          transition={{ duration: 0.2 }}
          className="group flex flex-col rounded-xl border border-(--border) bg-(--surface) p-6 transition-colors hover:border-(--warning)/60 hover:bg-(--surface-2)"
        >
          <div className="mb-5 inline-flex self-start rounded-lg bg-(--warning)/10 p-3 transition-colors group-hover:bg-(--warning)/20">
            <LightningIcon className="h-6 w-6 text-(--warning)" />
          </div>
          <h3 className="mb-3 font-mono text-lg font-semibold text-(--fg)">
            Event-Driven
          </h3>
          <p className="text-sm leading-relaxed text-(--muted) transition-colors group-hover:text-(--fg)/90">
            Built on Axum and Tokio. Asynchronous processing via `mpsc` queues
            means the interface never blocks while the AI generates answers.
          </p>
        </motion.div>
      </motion.div>
    </motion.div>
  )
}
