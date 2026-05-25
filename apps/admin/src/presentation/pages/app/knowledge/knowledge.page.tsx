import { BookOpenIcon, PlusIcon, XIcon } from "@phosphor-icons/react"
import { AnimatePresence, motion } from "motion/react"
import { useState } from "react"

import { Button } from "@nexus/ui"
import { cn } from "@nexus/utils"

import {
  useKnowledge,
  type KnowledgeArticle,
} from "@/application/knowledge/use-knowledge"

import { ArticleForm } from "./article-form"
import { ArticleItem } from "./article-item"

type StatusFilter = "all" | KnowledgeArticle["status"]

const STATUS_FILTERS: { value: StatusFilter; label: string; dot?: string }[] = [
  { value: "all", label: "All" },
  { value: "pending", label: "Pending", dot: "bg-(--warning)" },
  { value: "approved", label: "Approved", dot: "bg-(--success)" },
  { value: "rejected", label: "Rejected", dot: "bg-(--destructive)" },
]

export function KnowledgePage() {
  const { data: articles } = useKnowledge()
  const [showForm, setShowForm] = useState(false)
  const [statusFilter, setStatusFilter] = useState<StatusFilter>("all")
  const [search, setSearch] = useState("")

  const list = articles.data ?? []

  const filtered = list.filter((a) => {
    const matchStatus = statusFilter === "all" || a.status === statusFilter
    const matchSearch = a.title.toLowerCase().includes(search.toLowerCase())
    return matchStatus && matchSearch
  })

  const countFor = (status: StatusFilter) =>
    status === "all"
      ? list.length
      : list.filter((a) => a.status === status).length

  const STAT_ITEMS = [
    { label: "total", value: list.length, color: "text-(--fg)" },
    { label: "pending", value: countFor("pending"), color: "text-(--warning)" },
    {
      label: "approved",
      value: countFor("approved"),
      color: "text-(--success)",
    },
    {
      label: "rejected",
      value: countFor("rejected"),
      color: "text-(--destructive)",
    },
  ]

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="mx-auto max-w-3xl space-y-5"
    >
      {/* Page header */}
      <div className="flex items-start justify-between gap-4">
        <div className="flex items-center gap-2.5">
          <div className="flex h-7 w-7 items-center justify-center rounded-sm bg-(--accent)/10">
            <BookOpenIcon className="h-3.5 w-3.5 text-(--accent)" />
          </div>
          <div>
            <h1 className="font-mono text-sm font-semibold text-(--fg)">
              knowledge base
            </h1>
            <p className="font-mono text-[10px] text-(--muted)">
              Manage articles and content
            </p>
          </div>
        </div>
        <Button size="sm" onClick={() => setShowForm((v) => !v)}>
          {showForm ? (
            <>
              <XIcon className="h-3.5 w-3.5" /> cancel
            </>
          ) : (
            <>
              <PlusIcon className="h-3.5 w-3.5" /> new article
            </>
          )}
        </Button>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-4 overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {STAT_ITEMS.map(({ label, value, color }, i) => (
          <div
            key={label}
            className={cn(
              "flex flex-col items-center px-3 py-3 sm:px-5",
              i !== 0 && "border-l border-(--border)"
            )}
          >
            <span
              className={cn(
                "font-mono text-base font-semibold sm:text-lg",
                color
              )}
            >
              {value}
            </span>
            <span className="mt-0.5 font-mono text-[10px] text-(--muted)">
              {label}
            </span>
          </div>
        ))}
      </div>

      {/* Article form */}
      <AnimatePresence>
        {showForm && (
          <ArticleForm
            onCancel={() => setShowForm(false)}
            onSuccess={() => setShowForm(false)}
          />
        )}
      </AnimatePresence>

      {/* Articles list */}
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Filters + search */}
        <div className="relative flex flex-col gap-3 border-b border-(--border) px-5 py-3 sm:flex-row sm:items-center sm:justify-between">
          <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-(--accent)/30 via-(--accent)/15 to-transparent" />
          <div className="flex items-center gap-0.5">
            {STATUS_FILTERS.map(({ value, label, dot }) => (
              <button
                key={value}
                onClick={() => setStatusFilter(value)}
                className={cn(
                  "flex items-center gap-1.5 rounded-sm px-2.5 py-1.5 font-mono text-xs transition-colors",
                  statusFilter === value
                    ? "bg-(--accent)/10 text-(--fg)"
                    : "text-(--muted) hover:bg-(--surface-2) hover:text-(--fg)"
                )}
              >
                {dot && (
                  <span
                    className={cn("h-1.5 w-1.5 shrink-0 rounded-full", dot)}
                  />
                )}
                {label}
                <span className="rounded-sm bg-(--surface-2) px-1 py-0.5 text-[10px] text-(--border)">
                  {countFor(value)}
                </span>
              </button>
            ))}
          </div>
          <input
            type="text"
            placeholder="search articles..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className={cn(
              "h-7 w-full rounded-sm border border-(--border) bg-(--bg) px-3",
              "font-mono text-xs text-(--fg) placeholder:text-(--border)",
              "transition-colors outline-none focus:border-(--accent) sm:w-48"
            )}
          />
        </div>

        {filtered.length === 0 ? (
          <div className="flex flex-col items-center gap-3 px-5 py-14 text-center">
            <div className="flex h-12 w-12 items-center justify-center rounded-sm border border-(--border) bg-(--surface-2)">
              <BookOpenIcon className="h-5 w-5 text-(--border)" />
            </div>
            <div>
              <p className="font-mono text-xs font-medium text-(--fg)">
                {search || statusFilter !== "all"
                  ? "no articles match"
                  : "no articles yet"}
              </p>
              {!search && statusFilter === "all" && (
                <p className="mt-1 font-mono text-[10px] text-(--muted)">
                  create the first article to get started
                </p>
              )}
            </div>
          </div>
        ) : (
          <motion.ul
            initial="hidden"
            animate="show"
            variants={{
              hidden: {},
              show: { transition: { staggerChildren: 0.04 } },
            }}
            className="divide-y divide-(--border)"
          >
            <AnimatePresence>
              {filtered.map((article) => (
                <ArticleItem key={article.id} article={article} />
              ))}
            </AnimatePresence>
          </motion.ul>
        )}
      </div>
    </motion.div>
  )
}
