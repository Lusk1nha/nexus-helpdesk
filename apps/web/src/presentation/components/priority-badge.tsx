import { cn } from "@nexus/utils"
import type { TicketPriority } from "@/domain/tickets/ticket"

const CONFIG: Record<
  TicketPriority,
  { label: string; dot: string; text: string; bg: string }
> = {
  low: {
    label: "low",
    dot: "bg-(--muted)",
    text: "text-(--muted)",
    bg: "bg-(--muted)/10 border-(--muted)/20",
  },
  normal: {
    label: "normal",
    dot: "bg-(--accent)",
    text: "text-(--accent)",
    bg: "bg-(--accent)/10 border-(--accent)/20",
  },
  high: {
    label: "high",
    dot: "bg-(--destructive) animate-pulse",
    text: "text-(--destructive)",
    bg: "bg-(--destructive)/10 border-(--destructive)/20",
  },
}

interface PriorityBadgeProps {
  priority: TicketPriority
  className?: string
}

export function PriorityBadge({ priority, className }: PriorityBadgeProps) {
  const cfg = CONFIG[priority] ?? CONFIG.normal

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
