import {
  ChatTextIcon,
  PlusIcon,
  FunnelIcon,
  ArrowClockwiseIcon,
} from "@phosphor-icons/react"
import { motion, AnimatePresence } from "motion/react"
import { useState } from "react"
import { useNavigate } from "react-router"

import { Button } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useSession } from "@/application/auth/use-session"
import { useTickets } from "@/application/tickets/use-tickets"
import { useSystemSse } from "@/application/tickets/use-ticket-sse"
import { StatusBadge } from "@/presentation/components/status-badge"
import type { Ticket, TicketStatus } from "@/domain/tickets/ticket"
import { paths } from "@/presentation/router/paths"

import { CreateTicketModal } from "./create-ticket.modal"

const FILTERS: { label: string; value: TicketStatus | undefined }[] = [
  { label: "all", value: undefined },
  { label: "open", value: "open" },
  { label: "processing_ai", value: "processing_ai" },
  { label: "awaiting", value: "awaiting_agent_approval" },
  { label: "resolved", value: "resolved" },
  { label: "closed", value: "closed" },
]

function timeAgo(iso: string) {
  const diff = Date.now() - new Date(iso).getTime()
  const m = Math.floor(diff / 60000)
  if (m < 1) return "just now"
  if (m < 60) return `${m}m ago`
  const h = Math.floor(m / 60)
  if (h < 24) return `${h}h ago`
  return `${Math.floor(h / 24)}d ago`
}

function TicketCard({ ticket, index }: { ticket: Ticket; index: number }) {
  const navigate = useNavigate()

  return (
    <motion.button
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2, delay: index * 0.04 }}
      onClick={() => navigate(paths.app.ticketDetail(ticket.id))}
      className={cn(
        "group w-full text-left rounded-sm border border-(--border) bg-(--surface)",
        "px-4 py-3 transition-all duration-150",
        "hover:border-(--accent)/40 hover:bg-(--surface-hover)",
        "focus-visible:outline-2 focus-visible:outline-(--accent)"
      )}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2 mb-1">
            <span className="font-mono text-[10px] text-(--muted) shrink-0">
              #{ticket.id.slice(0, 8)}
            </span>
            <StatusBadge status={ticket.status as TicketStatus} />
          </div>
          <p className="font-mono text-sm font-medium text-(--fg) truncate group-hover:text-(--accent) transition-colors">
            {ticket.title}
          </p>
          <p className="mt-0.5 font-mono text-xs text-(--muted) line-clamp-1">
            {ticket.description}
          </p>
        </div>
        <span className="font-mono text-[10px] text-(--border) shrink-0 mt-0.5">
          {timeAgo(ticket.updatedAt)}
        </span>
      </div>
    </motion.button>
  )
}

export function TicketsPage() {
  const user = useSession()
  const navigate = useNavigate()
  const [filter, setFilter] = useState<TicketStatus | undefined>(undefined)
  const [createOpen, setCreateOpen] = useState(false)

  const { data: tickets, isLoading, refetch, isFetching } = useTickets(filter)

  useSystemSse()

  const isCustomer = user?.role === "customer"

  return (
    <div className="space-y-5 max-w-3xl mx-auto">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: -8 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.25 }}
        className="flex items-center justify-between"
      >
        <div>
          <div className="flex items-center gap-2 mb-0.5">
            <span className="font-mono text-xs text-(--accent)">◈</span>
            <span className="font-mono text-xs text-(--muted)">
              {isCustomer ? "my tickets" : "all tickets"}
            </span>
          </div>
          <h1 className="font-mono text-lg font-semibold text-(--fg)">
            Tickets
          </h1>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => refetch()}
            disabled={isFetching}
            className="p-2 rounded-sm text-(--muted) hover:text-(--fg) hover:bg-(--surface-2) transition-colors disabled:opacity-40"
          >
            <ArrowClockwiseIcon
              className={cn("h-3.5 w-3.5", isFetching && "animate-spin")}
            />
          </button>
          <Button size="sm" onClick={() => setCreateOpen(true)}>
            <PlusIcon className="h-3.5 w-3.5" />
            new ticket
          </Button>
        </div>
      </motion.div>

      {/* Filter tabs */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.1 }}
        className="flex items-center gap-1 overflow-x-auto pb-1 scrollbar-none"
      >
        <FunnelIcon className="h-3 w-3 text-(--muted) shrink-0 mr-1" />
        {FILTERS.map((f) => (
          <button
            key={f.label}
            onClick={() => setFilter(f.value)}
            className={cn(
              "shrink-0 rounded-sm px-3 py-1 font-mono text-xs transition-all",
              filter === f.value
                ? "bg-(--accent) text-(--accent-fg)"
                : "border border-(--border) text-(--muted) hover:text-(--fg) hover:border-(--accent)/40"
            )}
          >
            {f.label}
          </button>
        ))}
      </motion.div>

      {/* Ticket list */}
      {isLoading ? (
        <div className="space-y-2">
          {[...Array(4)].map((_, i) => (
            <div
              key={i}
              className="h-16 rounded-sm border border-(--border) bg-(--surface) animate-pulse"
            />
          ))}
        </div>
      ) : !tickets?.length ? (
        <motion.div
          initial={{ opacity: 0, scale: 0.97 }}
          animate={{ opacity: 1, scale: 1 }}
          className="flex flex-col items-center justify-center py-16 text-center"
        >
          <ChatTextIcon className="h-8 w-8 text-(--border) mb-3" />
          <p className="font-mono text-sm text-(--muted)">no tickets found</p>
          <p className="font-mono text-xs text-(--border) mt-1">
            {filter
              ? `no tickets with status "${filter}"`
              : isCustomer
                ? "open your first ticket below"
                : "waiting for customers to submit tickets"}
          </p>
          {isCustomer && (
            <Button
              size="sm"
              variant="outline"
              className="mt-4"
              onClick={() => setCreateOpen(true)}
            >
              <PlusIcon className="h-3.5 w-3.5" />
              open ticket
            </Button>
          )}
        </motion.div>
      ) : (
        <AnimatePresence mode="popLayout">
          <div className="space-y-2">
            {tickets.map((ticket, i) => (
              <TicketCard key={ticket.id} ticket={ticket} index={i} />
            ))}
          </div>
        </AnimatePresence>
      )}

      {/* Stats footer */}
      {!!tickets?.length && (
        <motion.p
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.3 }}
          className="font-mono text-[10px] text-(--border) text-center"
        >
          {tickets.length} ticket{tickets.length !== 1 ? "s" : ""}
          {filter ? ` · status: ${filter}` : " · all statuses"}
        </motion.p>
      )}

      <CreateTicketModal
        open={createOpen}
        onClose={() => setCreateOpen(false)}
        onCreated={(id) => {
          setCreateOpen(false)
          navigate(paths.app.ticketDetail(id))
        }}
      />
    </div>
  )
}
