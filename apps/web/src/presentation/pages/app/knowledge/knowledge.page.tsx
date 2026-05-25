import {
  BookOpenIcon,
  PlusIcon,
  TrashIcon,
  MagnifyingGlassIcon,
  FilePlusIcon,
} from "@phosphor-icons/react"
import { zodResolver } from "@hookform/resolvers/zod"
import { motion, AnimatePresence } from "motion/react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button, FormError, FormField, Input, Textarea } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useKnowledge } from "@/application/knowledge/use-knowledge"
import { useCreateKnowledge } from "@/application/knowledge/use-create-knowledge"
import { useDeleteKnowledge } from "@/application/knowledge/use-delete-knowledge"
import type { KnowledgeDocument } from "@/domain/knowledge/knowledge"

const schema = z.object({
  title: z.string().min(3, "Title must be at least 3 characters"),
  content: z.string().min(10, "Content must be at least 10 characters"),
})
type FormData = z.infer<typeof schema>

function timeFromEpoch(epoch: number) {
  const diff = Date.now() - epoch * 1000
  const m = Math.floor(diff / 60000)
  if (m < 1) return "just now"
  if (m < 60) return `${m}m ago`
  const h = Math.floor(m / 60)
  if (h < 24) return `${h}h ago`
  return `${Math.floor(h / 24)}d ago`
}

function DocCard({
  doc,
  index,
  onDelete,
  deleting,
}: {
  doc: KnowledgeDocument
  index: number
  onDelete: (id: string) => void
  deleting: boolean
}) {
  const [confirmDelete, setConfirmDelete] = useState(false)

  return (
    <motion.div
      initial={{ opacity: 0, y: 6 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, scale: 0.97 }}
      transition={{ delay: index * 0.04 }}
      className="rounded-sm border border-(--border) bg-(--surface) px-4 py-3 group"
    >
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2 mb-1">
            <span className="font-mono text-[10px] text-(--accent) bg-(--accent)/10 border border-(--accent)/20 rounded-sm px-1.5 py-0.5">
              {doc.documentType}
            </span>
            <span className="font-mono text-[10px] text-(--border)">
              {timeFromEpoch(doc.indexedAt)}
            </span>
          </div>
          <p className="font-mono text-sm font-medium text-(--fg) mb-1">
            {doc.title}
          </p>
          <p className="font-mono text-xs text-(--muted) line-clamp-2 leading-relaxed">
            {doc.contentPreview}
          </p>
          <p className="mt-1.5 font-mono text-[10px] text-(--border)">
            by {doc.indexedBy} · #{doc.id.slice(0, 8)}
          </p>
        </div>

        <div className="shrink-0">
          {confirmDelete ? (
            <div className="flex items-center gap-1">
              <button
                onClick={() => setConfirmDelete(false)}
                className="font-mono text-[10px] text-(--muted) hover:text-(--fg) px-2 py-1 rounded-sm transition-colors"
              >
                cancel
              </button>
              <Button
                size="sm"
                variant="destructive"
                onClick={() => onDelete(doc.id)}
                loading={deleting}
                className="text-[10px] h-6 px-2"
              >
                confirm
              </Button>
            </div>
          ) : (
            <button
              onClick={() => setConfirmDelete(true)}
              className={cn(
                "p-1.5 rounded-sm text-(--border) opacity-0 group-hover:opacity-100",
                "hover:text-(--destructive) hover:bg-(--destructive)/10 transition-all"
              )}
            >
              <TrashIcon className="h-3.5 w-3.5" />
            </button>
          )}
        </div>
      </div>
    </motion.div>
  )
}

export function KnowledgePage() {
  const [search, setSearch] = useState("")
  const [showForm, setShowForm] = useState(false)
  const [deletingId, setDeletingId] = useState<string | null>(null)

  const { data, isLoading } = useKnowledge()
  const create = useCreateKnowledge()
  const del = useDeleteKnowledge()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormData>({ resolver: zodResolver(schema) })

  const onSubmit = async (data: FormData) => {
    await create.mutateAsync(data)
    reset()
    setShowForm(false)
  }

  const handleDelete = async (id: string) => {
    setDeletingId(id)
    try {
      await del.mutateAsync(id)
    } finally {
      setDeletingId(null)
    }
  }

  const filtered = (data?.items ?? []).filter(
    (doc) =>
      !search ||
      doc.title.toLowerCase().includes(search.toLowerCase()) ||
      doc.contentPreview.toLowerCase().includes(search.toLowerCase())
  )

  return (
    <div className="max-w-4xl mx-auto space-y-5">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: -8 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <div className="flex items-center gap-2 mb-0.5">
            <span className="font-mono text-xs text-(--accent)">◈</span>
            <span className="font-mono text-xs text-(--muted)">
              knowledge base
            </span>
          </div>
          <h1 className="font-mono text-lg font-semibold text-(--fg)">
            Knowledge
          </h1>
        </div>
        <Button size="sm" onClick={() => setShowForm((v) => !v)}>
          <PlusIcon className="h-3.5 w-3.5" />
          {showForm ? "cancel" : "add article"}
        </Button>
      </motion.div>

      {/* Create form */}
      <AnimatePresence>
        {showForm && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: "auto" }}
            exit={{ opacity: 0, height: 0 }}
            transition={{ duration: 0.2 }}
            className="overflow-hidden"
          >
            <div className="rounded-sm border border-(--border) bg-(--surface) overflow-hidden">
              <div className="h-0.5 bg-(--accent)" />
              <div className="px-5 py-4 border-b border-(--border)">
                <div className="flex items-center gap-2 mb-0.5">
                  <FilePlusIcon className="h-3.5 w-3.5 text-(--accent)" />
                  <span className="font-mono text-xs text-(--muted)">
                    new article
                  </span>
                </div>
                <p className="font-mono text-sm font-medium text-(--fg)">
                  Add knowledge to the AI base
                </p>
              </div>
              <form
                onSubmit={handleSubmit(onSubmit)}
                noValidate
                className="px-5 py-4 space-y-4"
              >
                <FormError
                  error={create.error}
                  fallbackMessage="Failed to create article."
                />

                <FormField
                  label="Title"
                  htmlFor="title"
                  error={errors.title?.message}
                  required
                >
                  <Input
                    id="title"
                    placeholder="e.g. How to reset your password"
                    error={!!errors.title}
                    {...register("title")}
                  />
                </FormField>

                <FormField
                  label="Content"
                  htmlFor="content"
                  error={errors.content?.message}
                  required
                >
                  <Textarea
                    id="content"
                    placeholder="Write the knowledge base article content here. The AI will use this to answer customer questions."
                    rows={6}
                    error={!!errors.content}
                    {...register("content")}
                  />
                </FormField>

                <div className="flex justify-end gap-2">
                  <Button
                    type="button"
                    variant="secondary"
                    size="sm"
                    onClick={() => {
                      reset()
                      setShowForm(false)
                    }}
                  >
                    cancel
                  </Button>
                  <Button type="submit" size="sm" loading={create.isPending}>
                    {create.isPending ? "indexing..." : "index article →"}
                  </Button>
                </div>
              </form>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Search */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.1 }}
        className="relative"
      >
        <MagnifyingGlassIcon className="absolute left-3 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-(--muted)" />
        <input
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search articles..."
          className={cn(
            "w-full rounded-sm border border-(--border) bg-(--surface)",
            "pl-8 pr-3 py-2 font-mono text-xs text-(--fg)",
            "placeholder:text-(--muted) focus:outline-none focus:border-(--accent)",
            "transition-colors"
          )}
        />
      </motion.div>

      {/* Articles */}
      {isLoading ? (
        <div className="space-y-2">
          {[...Array(3)].map((_, i) => (
            <div
              key={i}
              className="h-20 rounded-sm border border-(--border) bg-(--surface) animate-pulse"
            />
          ))}
        </div>
      ) : !filtered.length ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="flex flex-col items-center justify-center py-14 text-center"
        >
          <BookOpenIcon className="h-8 w-8 text-(--border) mb-3" />
          <p className="font-mono text-sm text-(--muted)">
            {search ? "no articles match your search" : "knowledge base is empty"}
          </p>
          <p className="font-mono text-[10px] text-(--border) mt-1">
            {search
              ? "try a different keyword"
              : "add articles to help the AI answer customer questions"}
          </p>
        </motion.div>
      ) : (
        <AnimatePresence mode="popLayout">
          <div className="space-y-2">
            {filtered.map((doc, i) => (
              <DocCard
                key={doc.id}
                doc={doc}
                index={i}
                onDelete={handleDelete}
                deleting={deletingId === doc.id}
              />
            ))}
          </div>
        </AnimatePresence>
      )}

      {!!data?.count && (
        <motion.p
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center font-mono text-[10px] text-(--border)"
        >
          {filtered.length} of {data.count} article
          {data.count !== 1 ? "s" : ""}
          {search ? ` matching "${search}"` : " in knowledge base"}
        </motion.p>
      )}
    </div>
  )
}
