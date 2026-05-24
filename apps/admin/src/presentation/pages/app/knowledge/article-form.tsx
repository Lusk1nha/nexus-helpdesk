import { zodResolver } from "@hookform/resolvers/zod"
import { motion } from "motion/react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button, FormError, FormField, Input } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useCreateKnowledge } from "@/application/knowledge/use-knowledge"

const schema = z.object({
  title: z.string().min(3, "Title must be at least 3 characters"),
  content: z.string().min(10, "Content must be at least 10 characters"),
})

type FormInput = z.infer<typeof schema>

interface Props {
  onCancel: () => void
  onSuccess: () => void
}

export function ArticleForm({ onCancel, onSuccess }: Props) {
  const create = useCreateKnowledge()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormInput>({ resolver: zodResolver(schema) })

  const onSubmit = async (data: FormInput) => {
    await create.mutateAsync(data)
    reset()
    onSuccess()
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: -8 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -8 }}
      transition={{ duration: 0.18 }}
      className="overflow-hidden rounded-sm border border-(--accent)/30 bg-(--surface)"
    >
      <div className="relative border-b border-(--border) px-5 py-4">
        <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-(--accent)/50 via-(--accent)/25 to-transparent" />
        <p className="font-mono text-[10px] font-semibold tracking-widest text-(--muted) uppercase">
          new article
        </p>
        <p className="mt-1 font-mono text-xs text-(--fg)">
          Add a new article to the knowledge base
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
              "resize-y font-mono text-sm text-(--fg) transition-colors outline-none",
              "placeholder:text-(--muted) focus-visible:border-(--accent)",
              errors.content && "border-(--destructive)"
            )}
            {...register("content")}
          />
        </FormField>

        <div className="flex justify-end gap-2 border-t border-(--border) pt-4">
          <Button
            type="button"
            variant="secondary"
            size="sm"
            onClick={() => {
              reset()
              onCancel()
            }}
          >
            cancel
          </Button>
          <Button type="submit" size="sm" loading={create.isPending}>
            {create.isPending ? "Creating..." : "Create article"}
          </Button>
        </div>
      </form>
    </motion.div>
  )
}
