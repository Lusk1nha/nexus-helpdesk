import { zodResolver } from "@hookform/resolvers/zod"
import { BuildingsIcon, FloppyDiskIcon } from "@phosphor-icons/react"
import { useEffect } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button, FormError, FormField, Input, Textarea } from "@nexus/ui"

import { useTenant, useUpdateTenant } from "@/application/tenant/use-tenant"

const schema = z.object({
  name: z.string().min(2, "Name must be at least 2 characters"),
  description: z.string().optional(),
})

type FormInput = z.infer<typeof schema>

export function TenantPage() {
  const { data: tenant, isLoading, error } = useTenant()
  const update = useUpdateTenant()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isDirty },
  } = useForm<FormInput>({
    resolver: zodResolver(schema),
  })

  useEffect(() => {
    if (tenant) {
      reset({ name: tenant.name, description: tenant.description })
    }
  }, [tenant, reset])

  const onSubmit = async (data: FormInput) => {
    await update.mutateAsync(data)
    reset(data)
  }

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      <div className="flex items-center gap-2">
        <BuildingsIcon className="h-4 w-4 text-(--accent)" />
        <h1 className="font-mono text-sm font-semibold text-(--fg)">
          tenant settings
        </h1>
      </div>

      <div className="rounded-sm border border-(--border) bg-(--surface)">
        <div className="border-b border-(--border) px-5 py-4">
          <p className="font-mono text-xs font-medium text-(--fg)">
            organization
          </p>
          <p className="mt-0.5 font-mono text-xs text-(--muted)">
            Manage your tenant details
          </p>
        </div>

        {isLoading ? (
          <div className="px-5 py-8 text-center font-mono text-xs text-(--muted)">
            loading...
          </div>
        ) : error ? (
          <div className="px-5 py-8 text-center font-mono text-xs text-(--destructive)">
            failed to load tenant data
          </div>
        ) : (
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

            <div className="border-t border-(--border) pt-4">
              <div className="space-y-2">
                <p className="font-mono text-xs text-(--muted)">
                  <span className="text-(--border)">slug:</span>{" "}
                  <span className="text-(--fg)">{tenant?.slug}</span>
                </p>
                <p className="font-mono text-xs text-(--muted)">
                  <span className="text-(--border)">id:</span>{" "}
                  <span className="text-(--fg)">{tenant?.id}</span>
                </p>
                <p className="font-mono text-xs text-(--muted)">
                  <span className="text-(--border)">created:</span>{" "}
                  <span className="text-(--fg)">
                    {tenant?.createdAt
                      ? new Date(tenant.createdAt).toLocaleDateString()
                      : "—"}
                  </span>
                </p>
              </div>
            </div>

            <div className="flex justify-end">
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
          </form>
        )}
      </div>

      {update.isSuccess && (
        <p className="font-mono text-xs text-(--success)">
          ✓ tenant updated successfully
        </p>
      )}
    </div>
  )
}
