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

import { TitleTypingAnimation } from "@/presentation/components/title-typing"

const STATS = [
  { value: "100%", label: "Private" },
  { value: "Rust", label: "Backend" },
  { value: "Local", label: "LLMs" },
  { value: "RAG", label: "AI Engine" },
]

const TERMINAL_LINES = [
  { prompt: true, text: "nexus status --workspace acme" },
  { prompt: false, col1: "tenant", col2: "acme.nexus.com", col3: "online" },
  { prompt: false, col1: "model", col2: "llama3.2-3b", col3: "loaded" },
  { prompt: false, col1: "latency", col2: "84ms", col3: "p99" },
  {
    prompt: false,
    col1: "tickets",
    col2: "3 open · 1 pending",
    col3: "ai-draft",
  },
  { prompt: false, col1: "knowledge", col2: "42 articles", col3: "indexed" },
  { prompt: true, text: "" },
]

const FEATURES = [
  {
    icon: CpuIcon,
    title: "Agentic RAG",
    color: "accent",
    description:
      "Automated context retrieval with Qdrant and Ollama. Draft responses generated locally — sensitive data never leaves your infra.",
  },
  {
    icon: ShieldCheckIcon,
    title: "Strict Multi-Tenancy",
    color: "success",
    description:
      "Enterprise-grade isolation. Data separated at database level with shared Unit of Work abstractions in Rust.",
  },
  {
    icon: LightningIcon,
    title: "Event-Driven",
    color: "warning",
    description:
      "Built on Axum + Tokio. Async processing via mpsc queues means the interface never blocks while AI generates answers.",
  },
]

export function LandingPage() {
  const container: Variants = {
    hidden: { opacity: 0 },
    show: { opacity: 1, transition: { staggerChildren: 0.12 } },
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
      {/* Hero glow */}
      <div className="pointer-events-none absolute top-0 left-1/2 -z-10 h-72 w-full max-w-3xl -translate-x-1/2 rounded-full bg-(--accent)/12 blur-[120px]" />

      {/* Status badge */}
      <motion.div
        variants={item}
        className="mb-6 flex items-center gap-2 rounded-full border border-(--accent)/30 bg-(--accent)/8 px-3 py-1.5 font-mono text-[10px] font-medium text-(--accent) backdrop-blur-sm sm:px-4 sm:text-xs"
      >
        <span className="relative flex h-2 w-2 shrink-0">
          <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-(--accent) opacity-75" />
          <span className="relative inline-flex h-2 w-2 rounded-full bg-(--accent)" />
        </span>
        <span className="hidden sm:inline">
          nexus_engine v1.0 · all systems operational
        </span>
        <span className="sm:hidden">nexus_engine v1.0 · online</span>
      </motion.div>

      {/* Headline */}
      <motion.h1
        variants={item}
        className="mb-5 max-w-4xl font-mono text-3xl font-bold tracking-tight text-(--fg) sm:text-4xl lg:text-5xl"
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
          <span className="absolute -bottom-2 left-0 h-1 w-full rounded-full bg-(--accent)/25" />
        </span>
      </motion.h1>

      <motion.p
        variants={item}
        className="mb-6 max-w-2xl text-base leading-relaxed text-(--muted) sm:text-lg"
      >
        The multi-tenant helpdesk built for privacy and performance. Powered by
        a blazingly fast{" "}
        <strong className="font-semibold text-(--fg)">Rust</strong> backend and
        zero-cost{" "}
        <strong className="font-semibold text-(--fg)">local LLMs</strong>.
      </motion.p>

      {/* Stats strip — 2×2 on mobile, single row on sm+ */}
      <motion.div
        variants={item}
        className="mb-8 w-full max-w-xs overflow-hidden rounded-sm border border-(--border) bg-(--surface) sm:w-auto sm:max-w-none"
      >
        <div className="grid grid-cols-2 sm:flex">
          {STATS.map((s, i) => (
            <div
              key={s.label}
              className={cn(
                "flex flex-col items-center px-5 py-3 font-mono sm:px-6",
                i % 2 !== 0 && "border-l border-(--border)",
                i >= 2 && "border-t border-(--border) sm:border-t-0",
                i !== 0 && "sm:border-l sm:border-(--border)"
              )}
            >
              <span className="text-sm font-semibold text-(--accent) sm:text-base">
                {s.value}
              </span>
              <span className="mt-0.5 text-[10px] text-(--muted) sm:text-[11px]">
                {s.label}
              </span>
            </div>
          ))}
        </div>
      </motion.div>

      {/* CTA */}
      <motion.div
        variants={item}
        className="mb-12 flex w-full flex-col items-center justify-center gap-3 sm:w-auto sm:flex-row"
      >
        <Link
          className={cn(
            buttonVariants({ size: "lg" }),
            "h-11 w-full px-8 font-mono text-sm sm:w-auto"
          )}
          to="/register"
        >
          Initialize Workspace <ArrowRightIcon className="ml-2 h-4 w-4" />
        </Link>
        <Link
          className={cn(
            buttonVariants({ variant: "outline", size: "lg" }),
            "h-11 w-full bg-(--surface) px-8 font-mono text-sm hover:bg-(--surface-2) sm:w-auto"
          )}
          to="/about"
        >
          <TerminalWindowIcon className="mr-2 h-4 w-4" /> Explore Architecture
        </Link>
      </motion.div>

      {/* Terminal demo — overflow-x-auto to prevent page-level scroll */}
      <motion.div variants={item} className="mb-12 w-full max-w-2xl">
        <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface) text-left shadow-xl shadow-black/10">
          {/* Window chrome */}
          <div className="flex items-center gap-2 border-b border-(--border) bg-(--surface-2) px-3 py-2.5 sm:px-4 sm:py-3">
            <span className="h-2.5 w-2.5 rounded-full bg-(--destructive) opacity-60" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--warning) opacity-60" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--success) opacity-60" />
            <span className="ml-3 font-mono text-[11px] text-(--muted)">
              nexus — terminal
            </span>
          </div>
          {/* Scrollable content area */}
          <div className="overflow-x-auto">
            <div className="min-w-[360px] space-y-1.5 px-3 py-4 font-mono text-xs sm:px-4">
              {TERMINAL_LINES.map((line, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  transition={{ delay: 0.8 + i * 0.12, duration: 0.2 }}
                >
                  {line.prompt ? (
                    <p className="text-(--muted)">
                      <span className="mr-2 text-(--accent)">$</span>
                      <span className="text-(--fg)">{line.text}</span>
                      {i === TERMINAL_LINES.length - 1 && (
                        <motion.span
                          animate={{ opacity: [1, 0, 1] }}
                          transition={{ duration: 0.9, repeat: Infinity }}
                          className="ml-0.5 inline-block h-3 w-1.5 bg-(--accent) align-middle"
                        />
                      )}
                    </p>
                  ) : (
                    <div className="flex gap-3 text-(--fg)">
                      <span className="w-20 shrink-0 text-(--muted)">
                        {line.col1}
                      </span>
                      <span className="flex-1 truncate">{line.col2}</span>
                      <span className="shrink-0 text-(--success)">
                        {line.col3}
                      </span>
                    </div>
                  )}
                </motion.div>
              ))}
            </div>
          </div>
        </div>
      </motion.div>

      {/* Features grid */}
      <motion.div
        variants={item}
        className="grid w-full gap-4 text-left sm:gap-5 md:grid-cols-3"
      >
        {FEATURES.map(({ icon: Icon, title, color, description }) => (
          <motion.div
            key={title}
            whileHover={{ y: -4 }}
            transition={{ duration: 0.18 }}
            className={cn(
              "group flex flex-col rounded-sm border border-(--border) bg-(--surface) p-5 sm:p-6",
              "transition-colors hover:bg-(--surface-2)",
              color === "accent" && "hover:border-(--accent)/50",
              color === "success" && "hover:border-(--success)/50",
              color === "warning" && "hover:border-(--warning)/50"
            )}
          >
            <div
              className={cn(
                "mb-4 inline-flex self-start rounded-sm p-2.5 transition-colors",
                color === "accent" &&
                  "bg-(--accent)/10 group-hover:bg-(--accent)/20",
                color === "success" &&
                  "bg-(--success)/10 group-hover:bg-(--success)/20",
                color === "warning" &&
                  "bg-(--warning)/10 group-hover:bg-(--warning)/20"
              )}
            >
              <Icon
                className={cn(
                  "h-5 w-5",
                  color === "accent" && "text-(--accent)",
                  color === "success" && "text-(--success)",
                  color === "warning" && "text-(--warning)"
                )}
              />
            </div>
            <h3 className="mb-2 font-mono text-sm font-semibold text-(--fg)">
              {title}
            </h3>
            <p className="text-sm leading-relaxed text-(--muted)">
              {description}
            </p>
          </motion.div>
        ))}
      </motion.div>
    </motion.div>
  )
}
