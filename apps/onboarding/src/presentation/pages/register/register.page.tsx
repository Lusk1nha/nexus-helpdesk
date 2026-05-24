import { zodResolver } from "@hookform/resolvers/zod"
import { motion } from "motion/react"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import { EyeIcon, EyeClosedIcon, GlobeIcon } from "@phosphor-icons/react"

import { registerSchema, type RegisterInput } from "@nexus/auth"
import { Button, FormError, FormField, Input } from "@nexus/ui"
import { generateSlug } from "@nexus/utils"

import { useCheckSlugAvailability } from "@/application/auth/use-check-slug"
import { useRegister } from "@/application/auth/use-register"
import { workspaceUrl } from "@/env"

import { SlugStatusIcon } from "./slug-status-icon"
import { SuccessSplash } from "./success-splash"

export function RegisterPage() {
  const registerTenant = useRegister()
  const [showPassword, setShowPassword] = useState(false)
  const [provisionedSlug, setProvisionedSlug] = useState<string | null>(null)

  const {
    register,
    handleSubmit,
    watch,
    setValue,
    formState: { errors, isSubmitting, dirtyFields },
  } = useForm<RegisterInput>({
    resolver: zodResolver(registerSchema),
    defaultValues: { tenantName: "", tenantSlug: "" },
    mode: "onChange",
  })

  const currentSlug = watch("tenantSlug")
  const tenantName = watch("tenantName")

  // Auto-suggest a slug from the company name until the user manually edits it.
  useEffect(() => {
    if (!dirtyFields.tenantSlug) {
      setValue("tenantSlug", generateSlug(tenantName || ""), {
        shouldValidate: false,
        shouldDirty: false,
      })
    }
  }, [tenantName, dirtyFields.tenantSlug, setValue])

  // Real-time availability check against the backend.
  const slugCheck = useCheckSlugAvailability(currentSlug)

  const slugLocallyInvalid = Boolean(errors.tenantSlug)
  const slugAvailable = slugCheck.data?.available
  const slugReason = slugCheck.data?.reason

  // Register hook with `setValueAs` so the value reaching Zod is always
  // normalized — protects against paste from address bar, browser autofill, etc.
  const slugRegister = register("tenantSlug", {
    setValueAs: (v: unknown) =>
      String(v ?? "")
        .toLowerCase()
        .trim(),
  })

  const onSubmit = async (data: RegisterInput) => {
    try {
      const { tenantSlug } = await registerTenant.mutateAsync(data)
      // Hand off to <SuccessSplash />, which handles the actual redirect.
      setProvisionedSlug(tenantSlug)
    } catch (error) {
      // React Query already exposes the error via registerTenant.isError /
      // registerTenant.error — the surface is handled by <FormError /> below.
      console.error("Falha ao provisionar o Tenant:", error)
    }
  }

  if (provisionedSlug) {
    return <SuccessSplash tenantSlug={provisionedSlug} />
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="w-full max-w-xl"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        <div className="border-b border-(--border) bg-(--surface-2) px-6 pt-6 pb-4">
          <div className="mb-4 flex items-center gap-2">
            <span className="h-2.5 w-2.5 rounded-full bg-(--destructive) opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--warning) opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--success) opacity-70" />
          </div>
          <p className="mb-1 font-mono text-xs text-(--muted)">
            <span className="text-(--success)">$</span> nexus provision --new
          </p>
          <h1 className="font-mono text-xl font-semibold text-(--fg)">
            Create your workspace
          </h1>
          <p className="mt-1 text-sm text-(--muted)">
            Set up your multi-tenant helpdesk environment in seconds.
          </p>
        </div>

        <form onSubmit={handleSubmit(onSubmit)} noValidate className="p-6">
          <FormError
            error={registerTenant.error}
            fallbackMessage="It was not possible to proceed with this request!"
          />

          <div className="space-y-8">
            {/* Section 1: Organization */}
            <section className="space-y-4">
              <h2 className="font-mono text-xs font-semibold tracking-wider text-(--muted) uppercase">
                1. Organization Details
              </h2>

              <div className="grid gap-4 sm:grid-cols-2">
                <FormField
                  label="Company Name"
                  htmlFor="tenantName"
                  error={errors.tenantName?.message}
                  required
                >
                  <Input
                    id="tenantName"
                    placeholder="Acme Corp"
                    error={!!errors.tenantName}
                    {...register("tenantName")}
                  />
                </FormField>

                <FormField
                  label="Workspace URL"
                  htmlFor="tenantSlug"
                  error={
                    errors.tenantSlug?.message ??
                    (slugAvailable === false
                      ? (slugReason ?? undefined)
                      : undefined)
                  }
                  required
                >
                  <div className="relative">
                    <GlobeIcon className="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-(--muted)" />
                    <Input
                      id="tenantSlug"
                      placeholder="acme"
                      autoCapitalize="none"
                      autoCorrect="off"
                      spellCheck={false}
                      className="pr-9 pl-9 font-mono text-sm lowercase"
                      error={!!errors.tenantSlug || slugAvailable === false}
                      {...slugRegister}
                      onChange={(e) => {
                        // Strip invalid chars + force lowercase as the user types
                        // so what they see is what gets validated/submitted.
                        const cleaned = e.target.value
                          .toLowerCase()
                          .replace(/[^a-z0-9-]/g, "")
                        e.target.value = cleaned
                        slugRegister.onChange(e)
                      }}
                    />
                    <div className="absolute top-1/2 right-3 -translate-y-1/2">
                      <SlugStatusIcon
                        loading={slugCheck.isFetching}
                        available={slugAvailable}
                        invalid={slugLocallyInvalid}
                      />
                    </div>
                  </div>
                </FormField>
              </div>

              <div className="rounded border border-(--border)/50 bg-(--bg) p-3 font-mono text-xs text-(--muted)">
                Your workspace will be accessible at:{" "}
                <span className="text-(--accent)">
                  {currentSlug ? currentSlug : "<slug>"}
                </span>
                .nexus.com
              </div>
            </section>

            <hr className="border-(--border)/50" />

            {/* Section 2: Admin */}
            <section className="space-y-4">
              <h2 className="font-mono text-xs font-semibold tracking-wider text-(--muted) uppercase">
                2. Administrator Account
              </h2>

              <div className="grid gap-4 sm:grid-cols-2">
                <FormField
                  label="Full Name"
                  htmlFor="adminFullName"
                  error={errors.adminFullName?.message}
                  required
                >
                  <Input
                    id="adminFullName"
                    placeholder="Jane Doe"
                    error={!!errors.adminFullName}
                    {...register("adminFullName")}
                  />
                </FormField>

                <FormField
                  label="Work Email"
                  htmlFor="adminEmail"
                  error={errors.adminEmail?.message}
                  required
                >
                  <Input
                    id="adminEmail"
                    type="email"
                    placeholder="jane@acme.com"
                    error={!!errors.adminEmail}
                    {...register("adminEmail")}
                  />
                </FormField>
              </div>

              <div className="grid gap-4 sm:grid-cols-2">
                <FormField
                  label="Password"
                  htmlFor="adminPassword"
                  error={errors.adminPassword?.message}
                  required
                >
                  <div className="relative">
                    <Input
                      id="adminPassword"
                      type={showPassword ? "text" : "password"}
                      error={!!errors.adminPassword}
                      className="pr-9"
                      {...register("adminPassword")}
                    />
                    <button
                      type="button"
                      onClick={() => setShowPassword((v) => !v)}
                      className="absolute top-1/2 right-2.5 -translate-y-1/2 text-(--muted) hover:text-(--fg)"
                      tabIndex={-1}
                      aria-label={
                        showPassword ? "Hide password" : "Show password"
                      }
                    >
                      {showPassword ? (
                        <EyeClosedIcon className="h-3.5 w-3.5" />
                      ) : (
                        <EyeIcon className="h-3.5 w-3.5" />
                      )}
                    </button>
                  </div>
                </FormField>

                <FormField
                  label="Confirm Password"
                  htmlFor="confirmPassword"
                  error={errors.confirmPassword?.message}
                  required
                >
                  <Input
                    id="confirmPassword"
                    type={showPassword ? "text" : "password"}
                    error={!!errors.confirmPassword}
                    {...register("confirmPassword")}
                  />
                </FormField>
              </div>
            </section>
          </div>

          <div className="mt-8 border-t border-(--border) pt-6">
            <Button
              type="submit"
              className="w-full cursor-pointer"
              loading={isSubmitting || registerTenant.isPending}
              disabled={slugAvailable === false}
            >
              {isSubmitting || registerTenant.isPending
                ? "Provisioning Workspace..."
                : "Initialize Workspace →"}
            </Button>

            <p className="mt-4 text-center font-mono text-xs text-(--muted)">
              Already have a workspace?{" "}
              {currentSlug ? (
                <a
                  href={workspaceUrl(currentSlug, "/login")}
                  className="text-(--accent) underline-offset-2 hover:underline"
                >
                  Sign in at {currentSlug}.nexus.com →
                </a>
              ) : (
                <span className="text-(--muted)/70">
                  enter your slug above to sign in
                </span>
              )}
            </p>
          </div>
        </form>
      </div>
    </motion.div>
  )
}
