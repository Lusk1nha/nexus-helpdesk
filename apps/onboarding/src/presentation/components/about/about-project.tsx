import { motion, type Variants } from "motion/react"
import {
  MonitorIcon,
  ShieldIcon,
  CpuIcon,
  DatabaseIcon,
  ArrowDownIcon,
} from "@phosphor-icons/react"

interface AboutProjectProps {
  variants: Variants
}

const ARCH_LAYERS = [
  {
    icon: MonitorIcon,
    label: "Frontend",
    color: "accent",
    items: ["React 19", "Vite + TypeScript", "Tailwind v4", "TanStack Query"],
    connector: "REST · WebSocket",
  },
  {
    icon: ShieldIcon,
    label: "API Gateway",
    color: "success",
    items: ["Rust (Axum)", "Tokio async", "DDD + Unit of Work", "JWT + API Keys"],
    connector: "mpsc async queue",
  },
  {
    icon: CpuIcon,
    label: "AI Engine",
    color: "warning",
    items: ["Ollama (local LLMs)", "Qdrant vector DB", "RAG pipeline", "Auto-indexing"],
    connector: "SQLx + multi-tenant isolation",
  },
  {
    icon: DatabaseIcon,
    label: "Data Layer",
    color: "muted",
    items: ["PostgreSQL", "SQLx migrations", "Row-level tenancy", "Refresh token store"],
    connector: null,
  },
]

const colorMap = {
  accent: {
    bg: "bg-(--accent)/8",
    border: "border-(--accent)/30",
    icon: "text-(--accent)",
    label: "text-(--accent)",
  },
  success: {
    bg: "bg-(--success)/8",
    border: "border-(--success)/30",
    icon: "text-(--success)",
    label: "text-(--success)",
  },
  warning: {
    bg: "bg-(--warning)/8",
    border: "border-(--warning)/30",
    icon: "text-(--warning)",
    label: "text-(--warning)",
  },
  muted: {
    bg: "bg-(--surface-2)",
    border: "border-(--border)",
    icon: "text-(--muted)",
    label: "text-(--muted)",
  },
}

export function AboutProject({ variants }: AboutProjectProps) {
  return (
    <motion.div
      variants={variants}
      className="grid gap-6 lg:grid-cols-[1fr_360px]"
    >
      {/* Description */}
      <div className="rounded-sm border border-(--border) bg-(--surface) p-4 space-y-4 sm:p-6">
        <div className="flex items-center gap-2 pb-3 border-b border-(--border)">
          <span className="font-mono text-xs text-(--accent) font-semibold">◈</span>
          <h2 className="font-mono text-base font-semibold text-(--fg)">
            About Nexus
          </h2>
        </div>

        <p className="text-sm leading-relaxed text-(--muted)">
          Nexus Helpdesk was born from the desire to solve complex architectural
          challenges. Moving away from expensive cloud APIs, it explores a{" "}
          <strong className="text-(--fg)">Local-First AI</strong> approach,
          running Large Language Models (Llama 3, Phi-3) directly on the
          infrastructure via Ollama — zero data egress, zero per-request cost.
        </p>

        <p className="text-sm leading-relaxed text-(--muted)">
          Under the hood: a blazingly fast{" "}
          <strong className="text-(--fg)">Rust (Axum)</strong> backend with
          strict multi-tenant isolation, an event-driven background worker
          built on Tokio, and a React 19 monorepo with shared packages for
          UI, auth, themes and API contracts.
        </p>

        <div className="grid grid-cols-2 gap-4 pt-2">
          {[
            { label: "Backend", value: "Rust + Axum" },
            { label: "Frontend", value: "React 19 + Vite" },
            { label: "AI Runtime", value: "Ollama (local)" },
            { label: "Vector DB", value: "Qdrant" },
            { label: "Database", value: "PostgreSQL" },
            { label: "Architecture", value: "DDD + UoW" },
          ].map(({ label, value }) => (
            <div key={label} className="space-y-0.5">
              <p className="font-mono text-[10px] uppercase tracking-widest text-(--muted)">
                {label}
              </p>
              <p className="font-mono text-xs font-medium text-(--fg)">{value}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Architecture diagram */}
      <div className="rounded-sm border border-(--border) bg-(--surface) p-4 sm:p-6">
        <p className="font-mono text-[10px] uppercase tracking-widest text-(--muted) mb-4">
          System Architecture
        </p>

        <div className="space-y-1">
          {ARCH_LAYERS.map(({ icon: Icon, label, color, items, connector }, i) => {
            const c = colorMap[color as keyof typeof colorMap]
            return (
              <div key={label}>
                <motion.div
                  initial={{ opacity: 0, x: -8 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 0.3 + i * 0.1, duration: 0.3 }}
                  className={`rounded-sm border ${c.border} ${c.bg} px-3 py-3`}
                >
                  <div className="flex items-center gap-2 mb-1.5">
                    <Icon className={`h-3.5 w-3.5 shrink-0 ${c.icon}`} />
                    <span className={`font-mono text-xs font-semibold ${c.label}`}>
                      {label}
                    </span>
                  </div>
                  <div className="flex flex-wrap gap-1">
                    {items.map((item) => (
                      <span
                        key={item}
                        className="font-mono text-[10px] text-(--muted)"
                      >
                        {item}
                        {items.indexOf(item) < items.length - 1 && (
                          <span className="text-(--border) ml-1">·</span>
                        )}
                      </span>
                    ))}
                  </div>
                </motion.div>

                {connector && (
                  <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ delay: 0.4 + i * 0.1 }}
                    className="flex items-center gap-1.5 py-1 pl-3"
                  >
                    <ArrowDownIcon className="h-3 w-3 text-(--border)" />
                    <span className="font-mono text-[10px] text-(--border) italic">
                      {connector}
                    </span>
                  </motion.div>
                )}
              </div>
            )
          })}
        </div>
      </div>
    </motion.div>
  )
}
