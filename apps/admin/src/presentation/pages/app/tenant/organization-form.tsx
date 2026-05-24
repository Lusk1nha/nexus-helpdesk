import { zodResolver } from "@hookform/resolvers/zod"
import {
  CalendarIcon,
  FloppyDiskIcon,
  GlobeIcon,
  HashIcon,
} from "@phosphor-icons/react"
import { useEffect } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button, FormError, FormField, Input, Textarea } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useTenant, useUpdateTenant } from "@/application/tenant/use-tenant"
import { fmtDate } from "@/lib/format-date"

const schema = z.object({
  name: z.string().min(2, "Name must be at least 2 characters"),
  description: z.string().optional(),
})

type FormInput = z.infer<typeof schema>

const META_ICON = "h-3 w-3 shrink-0 text-(--muted)"

export function OrganizationForm() {
  const { data: tenant } = useTenant()
  const update = useUpdateTenant()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isDirty },
  } = useForm<FormInput>({ resolver: zodResolver(schema) })

  useEffect(() => {
    reset({ name: tenant.name, description: tenant.description })
  }, [tenant, reset])

  const onSubmit = async (data: FormInput) => {
    await update.mutateAsync(data)
    reset(data)
  }

  return (
    <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
      <div className="relative border-b border-(--border) px-5 py-4">
        <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-(--accent)/40 via-(--accent)/20 to-transparent" />
        <p className="font-mono text-[10px] font-semibold tracking-widest text-(--muted) uppercase">
          organization
        </p>
        <p className="mt-1 font-mono text-xs text-(--fg)">
          Manage your tenant name and description
        </p>
      </div>

      <form
        onSubmit={handleSubmit(onSubmit)}
        noValidate
        className="space-y-4 px-5 py-5"
      >
        <FormError
          error={update.error}
          fallbackMessage="Failed to update tenant."
        />

        <FormField
          label="Organization name"
          htmlFor="name"
          error={errors.name?.message}
          required
        >
          <Input
            id="name"
            placeholder="Acme Corp"
            error={!!errors.name}
            {...register("name")}
          />
        </FormField>

        <FormField
          label="Description"
          htmlFor="description"
          error={errors.description?.message}
        >
          <Textarea
            id="description"
            placeholder="Describe your organization"
            error={!!errors.description}
            {...register("description")}
          />
        </FormField>

        <div className="grid grid-cols-1 gap-2 rounded-sm border border-(--border) bg-(--bg) p-3 sm:grid-cols-3">
          <div className="flex items-center gap-2">
            <GlobeIcon className={META_ICON} />
            <div className="min-w-0">
              <p className="font-mono text-[10px] tracking-wider text-(--muted) uppercase">
                slug
              </p>
              <p className="truncate font-mono text-xs text-(--fg)">
                {tenant.slug}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <HashIcon className={META_ICON} />
            <div className="min-w-0">
              <p className="font-mono text-[10px] tracking-wider text-(--muted) uppercase">
                id
              </p>
              <p className="truncate font-mono text-[11px] text-(--fg)">
                {tenant.id}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <CalendarIcon className={META_ICON} />
            <div className="min-w-0">
              <p className="font-mono text-[10px] tracking-wider text-(--muted) uppercase">
                created
              </p>
              <p className="font-mono text-xs text-(--fg)">
                {fmtDate(tenant.createdAt)}
              </p>
            </div>
          </div>
        </div>

        <div className="flex items-center justify-between border-t border-(--border) pt-4">
          {update.isSuccess && (
            <p className="font-mono text-xs text-(--success)">✓ saved</p>
          )}
          <div
            className={cn(
              "ml-auto",
              !update.isSuccess && "flex w-full justify-end"
            )}
          >
            <Button
              type="submit"
              size="sm"
              disabled={!isDirty}
              loading={update.isPending}
            >
              <FloppyDiskIcon className="h-3.5 w-3.5" />
              {update.isPending ? "Saving..." : "Save changes"}
            </Button>
          </div>
        </div>
      </form>
    </div>
  )
}
