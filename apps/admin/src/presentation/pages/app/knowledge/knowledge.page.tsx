import { zodResolver } from "@hookform/resolvers/zod"
import {
  BookOpenIcon,
  PlusIcon,
  TrashIcon,
  XIcon,
} from "@phosphor-icons/react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button, FormError, FormField, Input } from "@nexus/ui"
import { cn } from "@nexus/utils"

import {
  useKnowledge,
  useCreateKnowledge,
  useDeleteKnowledge,
  type KnowledgeArticle,
} from "@/application/knowledge/use-knowledge"

const schema = z.object({
  title: z.string().min(3, "Title must be at least 3 characters"),
  content: z.string().min(10, "Content must be at least 10 characters"),
})

type FormInput = z.infer<typeof schema>

const statusColors: Record<KnowledgeArticle["status"], string> = {
  pending: "text-(--warning)",
  approved: "text-(--success)",
  rejected: "text-(--destructive)",
}

export function KnowledgePage() {
  const { data: articles, isLoading } = useKnowledge()
  const create = useCreateKnowledge()
  const remove = useDeleteKnowledge()
  const [showForm, setShowForm] = useState(false)

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormInput>({
    resolver: zodResolver(schema),
  })

  const onSubmit = async (data: FormInput) => {
    await create.mutateAsync(data)
    reset()
    setShowForm(false)
  }

  const handleDelete = (id: string) => {
    remove.mutate(id)
  }

  return (
    <div className="mx-auto max-w-3xl space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <BookOpenIcon className="h-4 w-4 text-(--accent)" />
          <h1 className="font-mono text-sm font-semibold text-(--fg)">
            knowledge base
          </h1>
          {articles && (
            <span className="rounded-sm bg-(--surface-2) px-1.5 py-0.5 font-mono text-[10px] text-(--muted)">
              {articles.length}
            </span>
          )}
        </div>
        <Button size="sm" onClick={() => setShowForm((v) => !v)}>
          {showForm ? (
            <>
              <XIcon className="h-3.5 w-3.5" />
              cancel
            </>
          ) : (
            <>
              <PlusIcon className="h-3.5 w-3.5" />
              new article
            </>
          )}
        </Button>
      </div>

      {showForm && (
        <div className="rounded-sm border border-(--accent)/30 bg-(--surface)">
          <div className="border-b border-(--border) px-5 py-4">
            <p className="font-mono text-xs font-medium text-(--fg)">
              new article
            </p>
          </div>
          <form
            onSubmit={handleSubmit(onSubmit)}
            noValidate
            className="space-y-4 px-5 py-5"
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
                placeholder="How to reset password"
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
              <textarea
                id="content"
                placeholder="Write the article content here..."
                rows={6}
                className={cn(
                  "flex w-full min-w-0 rounded-sm border border-(--border) bg-(--surface) px-3 py-2",
                  "font-mono text-sm text-(--fg) transition-colors outline-none resize-y",
                  "placeholder:text-(--muted)",
                  "focus-visible:border-(--accent)",
                  errors.content && "border-(--destructive)"
                )}
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
                {create.isPending ? "Creating..." : "Create article"}
              </Button>
            </div>
          </form>
        </div>
      )}

      <div className="rounded-sm border border-(--border) bg-(--surface)">
        <div className="border-b border-(--border) px-5 py-3">
          <p className="font-mono text-xs text-(--muted)">articles</p>
        </div>

        {isLoading ? (
          <div className="px-5 py-8 text-center font-mono text-xs text-(--muted)">
            loading...
          </div>
        ) : !articles || articles.length === 0 ? (
          <div className="flex flex-col items-center gap-2 px-5 py-12 text-center">
            <BookOpenIcon className="h-8 w-8 text-(--border)" />
            <p className="font-mono text-xs text-(--muted)">
              no articles yet
            </p>
            <p className="font-mono text-[10px] text-(--border)">
              create the first article to get started
            </p>
          </div>
        ) : (
          <ul className="divide-y divide-(--border)">
            {articles.map((article) => (
              <li
                key={article.id}
                className="flex items-start justify-between gap-4 px-5 py-4"
              >
                <div className="min-w-0 flex-1 space-y-1">
                  <p className="truncate font-mono text-xs font-medium text-(--fg)">
                    {article.title}
                  </p>
                  <p className="line-clamp-2 font-mono text-[10px] text-(--muted)">
                    {article.content}
                  </p>
                  <div className="flex items-center gap-3">
                    <span
                      className={cn(
                        "font-mono text-[10px]",
                        statusColors[article.status]
                      )}
                    >
                      {article.status}
                    </span>
                    <span className="font-mono text-[10px] text-(--border)">
                      {new Date(article.createdAt).toLocaleDateString()}
                    </span>
                  </div>
                </div>
                <button
                  onClick={() => handleDelete(article.id)}
                  disabled={remove.isPending}
                  className={cn(
                    "shrink-0 rounded-sm p-1.5 text-(--muted) transition-colors",
                    "hover:bg-(--destructive)/10 hover:text-(--destructive)",
                    "disabled:pointer-events-none disabled:opacity-40"
                  )}
                  aria-label="Delete article"
                >
                  <TrashIcon className="h-3.5 w-3.5" />
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  )
}
