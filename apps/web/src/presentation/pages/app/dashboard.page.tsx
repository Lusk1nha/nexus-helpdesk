import {
  ChatTextIcon,
  GaugeIcon,
  RobotIcon,
  CheckCircleIcon,
  TrayIcon,
} from "@phosphor-icons/react"
import { motion } from "motion/react"
import { useNavigate } from "react-router"

import { cn } from "@nexus/utils"

import { useSession } from "@/application/auth/use-session"
import { useTickets } from "@/application/tickets/use-tickets"
import { useSystemSse } from "@/application/tickets/use-ticket-sse"
import { StatusBadge } from "@/presentation/components/status-badge"
import { PriorityBadge } from "@/presentation/components/priority-badge"
import type { Ticket } from "@/domain/tickets/ticket"
import { paths } from "@/presentation/router/paths"

function timeAgo(iso: string) {
  const diff = Date.now() - new Date(iso).getTime()
  const m = Math.floor(diff / 60000)
  if (m < 1) return "just now"
  if (m < 60) return `${m}m ago`
  const h = Math.floor(m / 60)
  if (h < 24) return `${h}h ago`
  return `${Math.floor(h / 24)}d ago`
}

function StatCard({
  label,
  value,
  icon: Icon,
  accent,
  delay,
}: {
  label: string
  value: number
  icon: typeof GaugeIcon
  accent: string
  delay: number
}) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2, delay }}
      className="rounded-sm border border-(--border) bg-(--surface) px-4 py-3.5"
    >
      <div className="flex items-center justify-between">
        <span className="font-mono text-[10px] text-(--muted)">{label}</span>
        <Icon className={cn("h-3.5 w-3.5", accent)} />
      </div>
      <p className="mt-2 font-mono text-2xl font-semibold text-(--fg)">
        {value}
      </p>
    </motion.div>
  )
}

export function DashboardPage() {
  const user = useSession()
  const navigate = useNavigate()
  const { data: tickets, isLoading } = useTickets()

  useSystemSse()

  const isCustomer = user?.role === "customer"

  const list: Ticket[] = tickets ?? []
  const count = (status: Ticket["status"]) =>
    list.filter((t) => t.status === status).length

  const recent = [...list]
    .sort(
      (a, b) =>
        new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
    )
    .slice(0, 5)

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
          <GaugeIcon className="h-3.5 w-3.5 text-(--accent)" />
        </div>
        <div>
          <h1 className="font-mono text-sm font-semibold text-(--fg)">
            Overview
          </h1>
          <p className="font-mono text-[10px] text-(--muted)">
            {isCustomer
              ? "your support tickets at a glance"
              : "tenant-wide ticket activity"}
          </p>
        </div>
      </div>

      {/* Stat cards */}
      {isLoading ? (
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
          {[...Array(4)].map((_, i) => (
            <div
              key={i}
              className="h-20 rounded-sm border border-(--border) bg-(--surface) animate-pulse"
            />
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
          <StatCard
            label="total"
            value={list.length}
            icon={TrayIcon}
            accent="text-(--muted)"
            delay={0.02}
          />
          <StatCard
            label="open"
            value={count("open")}
            icon={ChatTextIcon}
            accent="text-(--accent)"
            delay={0.06}
          />
          <StatCard
            label="awaiting review"
            value={count("awaiting_agent_approval")}
            icon={RobotIcon}
            accent="text-orange-400"
            delay={0.1}
          />
          <StatCard
            label="resolved"
            value={count("resolved")}
            icon={CheckCircleIcon}
            accent="text-(--success)"
            delay={0.14}
          />
        </div>
      )}

      {/* Recent activity */}
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <h2 className="font-mono text-xs font-medium text-(--muted)">
            recent activity
          </h2>
          <button
            onClick={() => navigate(paths.app.tickets)}
            className="font-mono text-[10px] text-(--accent) hover:underline"
          >
            view all →
          </button>
        </div>

        {!isLoading && recent.length === 0 ? (
          <div className="flex flex-col items-center justify-center rounded-sm border border-(--border) bg-(--surface) py-10 text-center">
            <ChatTextIcon className="mb-2 h-7 w-7 text-(--border)" />
            <p className="font-mono text-xs text-(--muted)">no tickets yet</p>
          </div>
        ) : (
          <div className="space-y-2">
            {recent.map((t, i) => (
              <motion.button
                key={t.id}
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.2, delay: i * 0.04 }}
                onClick={() => navigate(paths.app.ticketDetail(t.id))}
                className={cn(
                  "group flex w-full items-center justify-between gap-3 rounded-sm border border-(--border) bg-(--surface) px-4 py-2.5 text-left transition-all",
                  "hover:border-(--accent)/40 hover:bg-(--surface-hover)"
                )}
              >
                <div className="flex min-w-0 items-center gap-2">
                  <span className="font-mono text-[10px] text-(--muted) shrink-0">
                    #{t.id.slice(0, 8)}
                  </span>
                  <span className="truncate font-mono text-xs text-(--fg) group-hover:text-(--accent)">
                    {t.title}
                  </span>
                </div>
                <div className="flex shrink-0 items-center gap-1.5">
                  <PriorityBadge priority={t.priority} />
                  <StatusBadge status={t.status} />
                  <span className="font-mono text-[10px] text-(--border)">
                    {timeAgo(t.updatedAt)}
                  </span>
                </div>
              </motion.button>
            ))}
          </div>
        )}
      </div>
    </motion.div>
  )
}
