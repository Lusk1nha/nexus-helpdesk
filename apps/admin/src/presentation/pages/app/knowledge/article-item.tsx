import { TrashIcon } from "@phosphor-icons/react"
import { motion } from "motion/react"
import { useState } from "react"

import { cn } from "@nexus/utils"

import {
  useDeleteKnowledge,
  type KnowledgeArticle,
} from "@/application/knowledge/use-knowledge"
import { fmtDate } from "@/lib/format-date"

const STATUS_STYLES: Record<
  KnowledgeArticle["status"],
  { dot: string; badge: string; label: string }
> = {
  pending: {
    dot: "bg-(--warning)",
    badge: "bg-(--warning)/10 text-(--warning) border border-(--warning)/20",
    label: "pending",
  },
  approved: {
    dot: "bg-(--success)",
    badge: "bg-(--success)/10 text-(--success) border border-(--success)/20",
    label: "approved",
  },
  rejected: {
    dot: "bg-(--destructive)",
    badge: "bg-(--destructive)/10 text-(--destructive) border border-(--destructive)/20",
    label: "rejected",
  },
}

interface Props {
  article: KnowledgeArticle
}

export function ArticleItem({ article }: Props) {
  const remove = useDeleteKnowledge()
  const [confirming, setConfirming] = useState(false)
  const s = STATUS_STYLES[article.status]

  const handleDelete = () => {
    if (!confirming) {
      setConfirming(true)
      return
    }
    remove.mutate(article.id, { onSettled: () => setConfirming(false) })
  }

  return (
    <motion.li
      layout
      initial={{ opacity: 0, x: -6 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 6 }}
      transition={{ duration: 0.15 }}
      className="flex items-start justify-between gap-4 px-5 py-4 transition-colors hover:bg-(--surface-2)/40"
    >
      <div className="min-w-0 flex-1 space-y-1.5">
        <div className="flex items-center gap-2">
          <span className={cn("inline-flex shrink-0 rounded-full px-2 py-0.5 font-mono text-[10px] font-medium", s.badge)}>
            <span className={cn("mr-1.5 mt-px h-1.5 w-1.5 shrink-0 rounded-full self-center", s.dot)} />
            {s.label}
          </span>
        </div>
        <p className="truncate font-mono text-xs font-medium text-(--fg)">{article.title}</p>
        <p className="line-clamp-1 font-mono text-[10px] text-(--muted)">{article.content}</p>
        <p className="font-mono text-[10px] text-(--border)">{fmtDate(article.createdAt)}</p>
      </div>

      <div className="flex shrink-0 items-center gap-2 pt-0.5">
        {confirming && (
          <motion.span
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="font-mono text-[10px] text-(--destructive)"
          >
            confirm?
          </motion.span>
        )}
        <button
          onClick={handleDelete}
          disabled={remove.isPending}
          onBlur={() => setConfirming(false)}
          className={cn(
            "rounded-sm p-1.5 transition-colors",
            "disabled:pointer-events-none disabled:opacity-40",
            confirming
              ? "bg-(--destructive)/10 text-(--destructive)"
              : "text-(--border) hover:bg-(--destructive)/10 hover:text-(--destructive)"
          )}
          aria-label={confirming ? "Confirm delete" : "Delete article"}
        >
          <TrashIcon className="h-3.5 w-3.5" />
        </button>
      </div>
    </motion.li>
  )
}
