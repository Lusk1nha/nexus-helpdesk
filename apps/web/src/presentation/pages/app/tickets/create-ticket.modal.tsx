import { zodResolver } from "@hookform/resolvers/zod"
import { XIcon } from "@phosphor-icons/react"
import { AnimatePresence, motion } from "motion/react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button, FormError, FormField, Input, Textarea } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useCreateTicket } from "@/application/tickets/use-create-ticket"

const schema = z.object({
  title: z.string().min(3, "Title must be at least 3 characters"),
  description: z.string().min(1, "Description is required"),
})
type FormData = z.infer<typeof schema>

interface CreateTicketModalProps {
  open: boolean
  onClose: () => void
  onCreated: (id: string) => void
}

export function CreateTicketModal({
  open,
  onClose,
  onCreated,
}: CreateTicketModalProps) {
  const create = useCreateTicket()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormData>({ resolver: zodResolver(schema) })

  const onSubmit = async (data: FormData) => {
    const result = await create.mutateAsync(data)
    reset()
    onCreated(result.ticketId)
  }

  const handleClose = () => {
    reset()
    create.reset()
    onClose()
  }

  return (
    <AnimatePresence>
      {open && (
        <>
          {/* Backdrop */}
          <motion.div
            key="backdrop"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            onClick={handleClose}
            className="fixed inset-0 z-40 bg-black/60 backdrop-blur-[2px]"
          />

          {/* Modal */}
          <motion.div
            key="modal"
            initial={{ opacity: 0, scale: 0.95, y: 16 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 8 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            className={cn(
              "fixed left-1/2 top-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2",
              "mx-4"
            )}
          >
            <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface) shadow-2xl">
              {/* Accent strip */}
              <div className="h-0.5 bg-(--accent)" />

              {/* Header */}
              <div className="flex items-center justify-between border-b border-(--border) px-5 py-4">
                <div>
                  <div className="flex items-center gap-2 mb-0.5">
                    <span className="font-mono text-xs text-(--accent)">◈</span>
                    <span className="font-mono text-xs text-(--muted)">
                      new ticket
                    </span>
                  </div>
                  <h2 className="font-mono text-base font-semibold text-(--fg)">
                    Open a support ticket
                  </h2>
                </div>
                <button
                  onClick={handleClose}
                  className="rounded-sm p-1.5 text-(--muted) hover:text-(--fg) hover:bg-(--surface-2) transition-colors"
                >
                  <XIcon className="h-4 w-4" />
                </button>
              </div>

              {/* Form */}
              <form
                onSubmit={handleSubmit(onSubmit)}
                noValidate
                className="space-y-4 px-5 py-5"
              >
                <FormError
                  error={create.error}
                  fallbackMessage="Failed to create ticket."
                />

                <FormField
                  label="Title"
                  htmlFor="title"
                  error={errors.title?.message}
                  required
                >
                  <Input
                    id="title"
                    placeholder="Brief description of the issue"
                    error={!!errors.title}
                    {...register("title")}
                  />
                </FormField>

                <FormField
                  label="Description"
                  htmlFor="description"
                  error={errors.description?.message}
                  required
                >
                  <Textarea
                    id="description"
                    placeholder="Describe the problem in detail..."
                    rows={4}
                    error={!!errors.description}
                    {...register("description")}
                  />
                </FormField>

                <div className="flex items-center justify-end gap-2 pt-1">
                  <Button
                    type="button"
                    variant="secondary"
                    size="sm"
                    onClick={handleClose}
                  >
                    cancel
                  </Button>
                  <Button type="submit" size="sm" loading={create.isPending}>
                    {create.isPending ? "submitting..." : "submit ticket →"}
                  </Button>
                </div>
              </form>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  )
}
