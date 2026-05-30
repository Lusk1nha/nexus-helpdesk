import {
  ShieldIcon,
  UsersIcon,
  BookOpenIcon,
  ArrowSquareOutIcon,
  CircleIcon,
} from "@phosphor-icons/react"
import { motion } from "motion/react"
import { useNavigate } from "react-router"

import { cn } from "@nexus/utils"

import { useTeam } from "@/application/identity/use-team"
import { useTickets } from "@/application/tickets/use-tickets"
import type { Ticket } from "@/domain/tickets/ticket"
import { paths } from "@/presentation/router/paths"

const roleColors: Record<string, string> = {
  admin: "text-(--accent)",
  agent: "text-(--success)",
  customer: "text-(--muted)",
}

export function AdminPage() {
  const navigate = useNavigate()
  const { data: tickets } = useTickets()
  const { data: team, isLoading: teamLoading } = useTeam()

  const list: Ticket[] = tickets ?? []
  const count = (status: Ticket["status"]) =>
    list.filter((t) => t.status === status).length

  const agents = (team ?? []).filter(
    (m) => m.role === "agent" || m.role === "admin"
  ).length
  const customers = (team ?? []).filter((m) => m.role === "customer").length

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="mx-auto max-w-3xl space-y-6"
    >
      {/* Header */}
      <div className="flex items-center gap-2.5">
        <div className="flex h-7 w-7 items-center justify-center rounded-sm bg-(--accent)/10">
          <ShieldIcon className="h-3.5 w-3.5 text-(--accent)" />
        </div>
        <div>
          <h1 className="font-mono text-sm font-semibold text-(--fg)">Admin</h1>
          <p className="font-mono text-[10px] text-(--muted)">
            workspace overview &amp; team
          </p>
        </div>
      </div>

      {/* Stat strip */}
      <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
        {[
          { label: "tickets", value: list.length },
          { label: "open", value: count("open") },
          { label: "agents", value: agents },
          { label: "customers", value: customers },
        ].map((s, i) => (
          <motion.div
            key={s.label}
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.2, delay: i * 0.04 }}
            className="rounded-sm border border-(--border) bg-(--surface) px-4 py-3.5"
          >
            <span className="font-mono text-[10px] text-(--muted)">
              {s.label}
            </span>
            <p className="mt-2 font-mono text-2xl font-semibold text-(--fg)">
              {s.value}
            </p>
          </motion.div>
        ))}
      </div>

      {/* Team */}
      <div className="space-y-2">
        <div className="flex items-center gap-1.5">
          <UsersIcon className="h-3.5 w-3.5 text-(--muted)" />
          <h2 className="font-mono text-xs font-medium text-(--muted)">team</h2>
        </div>

        {teamLoading ? (
          <div className="space-y-2">
            {[...Array(3)].map((_, i) => (
              <div
                key={i}
                className="h-12 rounded-sm border border-(--border) bg-(--surface) animate-pulse"
              />
            ))}
          </div>
        ) : (
          <div className="space-y-2">
            {(team ?? []).map((m, i) => (
              <motion.div
                key={m.userId}
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.2, delay: i * 0.03 }}
                className="flex items-center justify-between gap-3 rounded-sm border border-(--border) bg-(--surface) px-4 py-2.5"
              >
                <div className="min-w-0">
                  <p className="truncate font-mono text-xs text-(--fg)">
                    {m.fullName}
                  </p>
                  <p className="truncate font-mono text-[10px] text-(--muted)">
                    {m.email}
                  </p>
                </div>
                <div className="flex shrink-0 items-center gap-2">
                  <span
                    className={cn(
                      "inline-flex items-center gap-1 font-mono text-[10px]",
                      m.isActive ? "text-(--success)" : "text-(--border)"
                    )}
                  >
                    <CircleIcon weight="fill" className="h-1.5 w-1.5" />
                    {m.isActive ? "active" : "inactive"}
                  </span>
                  <span
                    className={cn(
                      "font-mono text-[10px] font-medium rounded-sm border border-(--border) px-1.5 py-0.5",
                      roleColors[m.role] ?? "text-(--muted)"
                    )}
                  >
                    {m.role}
                  </span>
                </div>
              </motion.div>
            ))}
          </div>
        )}
      </div>

      {/* Quick links */}
      <div className="space-y-2">
        <h2 className="font-mono text-xs font-medium text-(--muted)">
          quick links
        </h2>
        <div className="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <button
            onClick={() => navigate(paths.app.knowledge)}
            className="flex items-center gap-2.5 rounded-sm border border-(--border) bg-(--surface) px-4 py-3 text-left transition-all hover:border-(--accent)/40 hover:bg-(--surface-hover)"
          >
            <BookOpenIcon className="h-4 w-4 text-(--accent)" />
            <div>
              <p className="font-mono text-xs text-(--fg)">Knowledge base</p>
              <p className="font-mono text-[10px] text-(--muted)">
                manage RAG documents
              </p>
            </div>
          </button>
          <a
            href="https://github.com"
            onClick={(e) => e.preventDefault()}
            className="flex items-center gap-2.5 rounded-sm border border-(--border) bg-(--surface) px-4 py-3 text-left opacity-60"
          >
            <ArrowSquareOutIcon className="h-4 w-4 text-(--muted)" />
            <div>
              <p className="font-mono text-xs text-(--fg)">Admin console</p>
              <p className="font-mono text-[10px] text-(--muted)">
                full tenant administration
              </p>
            </div>
          </a>
        </div>
      </div>
    </motion.div>
  )
}
