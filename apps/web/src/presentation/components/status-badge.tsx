import { cn } from "@nexus/utils"
import type { TicketStatus } from "@/domain/tickets/ticket"

const CONFIG: Record<
  TicketStatus,
  { label: string; dot: string; text: string; bg: string }
> = {
  open: {
    label: "open",
    dot: "bg-(--accent)",
    text: "text-(--accent)",
    bg: "bg-(--accent)/10 border-(--accent)/20",
  },
  processing_ai: {
    label: "processing_ai",
    dot: "bg-(--warning) animate-pulse",
    text: "text-(--warning)",
    bg: "bg-(--warning)/10 border-(--warning)/20",
  },
  awaiting_agent_approval: {
    label: "awaiting_approval",
    dot: "bg-orange-400 animate-pulse",
    text: "text-orange-400",
    bg: "bg-orange-400/10 border-orange-400/20",
  },
  resolved: {
    label: "resolved",
    dot: "bg-(--success)",
    text: "text-(--success)",
    bg: "bg-(--success)/10 border-(--success)/20",
  },
  closed: {
    label: "closed",
    dot: "bg-(--muted)",
    text: "text-(--muted)",
    bg: "bg-(--muted)/10 border-(--muted)/20",
  },
}

interface StatusBadgeProps {
  status: TicketStatus
  className?: string
}

export function StatusBadge({ status, className }: StatusBadgeProps) {
  const cfg = CONFIG[status]

  return (
    <span
      className={cn(
        "inline-flex items-center gap-1.5 rounded-sm border px-2 py-0.5",
        "font-mono text-[10px] font-medium",
        cfg.bg,
        cfg.text,
        className
      )}
    >
      <span className={cn("h-1.5 w-1.5 rounded-full shrink-0", cfg.dot)} />
      {cfg.label}
    </span>
  )
}
