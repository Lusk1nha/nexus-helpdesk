import { zodResolver } from "@hookform/resolvers/zod"
import { motion } from "motion/react"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import {
  EyeIcon,
  EyeClosedIcon,
  GlobeIcon,
  CheckIcon,
  ShieldCheckIcon,
  CpuIcon,
  UsersIcon,
  ArrowSquareOutIcon,
} from "@phosphor-icons/react"

import { registerSchema, type RegisterInput } from "@nexus/auth"
import { Button, FormError, FormField, Input } from "@nexus/ui"
import { generateSlug } from "@nexus/utils"
import { cn } from "@nexus/utils"

import { useCheckSlugAvailability } from "@/application/auth/use-check-slug"
import { useRegister } from "@/application/auth/use-register"
import { workspaceUrl, env } from "@/env"

import { SlugStatusIcon } from "./slug-status-icon"
import { SuccessSplash } from "./success-splash"

const FEATURES = [
  { icon: CpuIcon, text: "AI-powered ticket drafts with local LLMs" },
  { icon: ShieldCheckIcon, text: "Strict multi-tenant data isolation" },
  { icon: UsersIcon, text: "Unlimited agents and customers" },
]

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

  useEffect(() => {
    if (!dirtyFields.tenantSlug) {
      setValue("tenantSlug", generateSlug(tenantName || ""), {
        shouldValidate: false,
        shouldDirty: false,
      })
    }
  }, [tenantName, dirtyFields.tenantSlug, setValue])

  const slugCheck = useCheckSlugAvailability(currentSlug)
  const slugLocallyInvalid = Boolean(errors.tenantSlug)
  const slugAvailable = slugCheck.data?.available
  const slugReason = slugCheck.data?.reason

  const slugRegister = register("tenantSlug", {
    setValueAs: (v: unknown) =>
      String(v ?? "")
        .toLowerCase()
        .trim(),
  })

  const onSubmit = async (data: RegisterInput) => {
    try {
      const { tenantSlug } = await registerTenant.mutateAsync(data)
      setProvisionedSlug(tenantSlug)
    } catch (error) {
      console.error("Falha ao provisionar o Tenant:", error)
    }
  }

  if (provisionedSlug) {
    return <SuccessSplash tenantSlug={provisionedSlug} />
  }

  const previewUrl = env.workspaceUrlTemplate.replace(
    "{slug}",
    currentSlug || "your-company"
  )

  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="w-full max-w-5xl"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface) shadow-xl shadow-black/10 md:grid md:grid-cols-[300px_1fr] lg:grid-cols-[340px_1fr]">

        {/* ── Left panel ───────────────────────────────────────────── */}
        <div className="flex flex-col justify-between border-b border-(--border) bg-(--surface-2) px-5 py-5 sm:px-8 sm:py-8 md:border-b-0 md:border-r">
          <div>
            {/* Brand */}
            <div className="flex items-center gap-2 mb-5 sm:mb-8">
              <span className="text-lg font-semibold text-(--accent)">◈</span>
              <span className="font-mono text-sm font-medium text-(--fg)">nexus</span>
            </div>

            <h2 className="font-mono text-lg font-semibold text-(--fg) leading-snug mb-1.5 sm:text-xl sm:mb-2">
              Your workspace<br />in 60 seconds
            </h2>
            <p className="font-mono text-xs text-(--muted) leading-relaxed mb-5 sm:mb-8">
              Set up your AI-powered helpdesk. No credit card required.
            </p>

            {/* Feature list — hidden on pure mobile (stacked layout), shown md+ */}
            <div className="hidden md:block space-y-3 mb-10">
              {FEATURES.map(({ icon: Icon, text }) => (
                <div key={text} className="flex items-start gap-3">
                  <div className="flex h-5 w-5 shrink-0 items-center justify-center rounded-sm bg-(--accent)/10 mt-0.5">
                    <Icon className="h-3 w-3 text-(--accent)" />
                  </div>
                  <span className="font-mono text-xs text-(--muted) leading-relaxed">{text}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Live URL preview */}
          <div className="rounded-sm border border-(--border) bg-(--bg) p-3 sm:p-4">
            <p className="font-mono text-[10px] text-(--muted) uppercase tracking-wider mb-2">
              Your workspace URL
            </p>
            <div className="flex items-center gap-1.5 font-mono text-xs">
              <GlobeIcon className="h-3.5 w-3.5 shrink-0 text-(--accent)" />
              <span
                className={cn(
                  "transition-colors truncate",
                  currentSlug ? "text-(--fg)" : "text-(--border)"
                )}
              >
                {currentSlug || "your-company"}
                <span className="text-(--muted)">.nexus.com</span>
              </span>
              {slugAvailable === true && (
                <span className="ml-auto shrink-0 flex items-center gap-1 text-(--success) text-[10px]">
                  <CheckIcon className="h-3 w-3" weight="bold" />
                  available
                </span>
              )}
            </div>

            {currentSlug && (
              <a
                href={previewUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="mt-2 flex items-center gap-1 font-mono text-[10px] text-(--muted) hover:text-(--accent) transition-colors"
              >
                <ArrowSquareOutIcon className="h-3 w-3" />
                preview link (dev)
              </a>
            )}
          </div>
        </div>

        {/* ── Right panel: form ─────────────────────────────────────── */}
        <div className="px-5 py-5 sm:px-8 sm:py-8">
          <div className="mb-6">
            <h1 className="font-mono text-lg font-semibold text-(--fg)">
              Create your workspace
            </h1>
            <p className="mt-1 text-sm text-(--muted)">
              Fill in your organization and admin account details below.
            </p>
          </div>

          <form onSubmit={handleSubmit(onSubmit)} noValidate className="space-y-8">
            <FormError
              error={registerTenant.error}
              fallbackMessage="Could not provision workspace. Please try again."
            />

            {/* Section 1 */}
            <section className="space-y-4">
              <h2 className="font-mono text-[10px] font-semibold uppercase tracking-widest text-(--muted)">
                1 — Organization
              </h2>

              <div className="grid gap-4 sm:grid-cols-2">
                <FormField
                  label="Company name"
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
                  label="Workspace slug"
                  htmlFor="tenantSlug"
                  error={
                    errors.tenantSlug?.message ??
                    (slugAvailable === false ? (slugReason ?? undefined) : undefined)
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
            </section>

            <hr className="border-(--border)/50" />

            {/* Section 2 */}
            <section className="space-y-4">
              <h2 className="font-mono text-[10px] font-semibold uppercase tracking-widest text-(--muted)">
                2 — Administrator account
              </h2>

              <div className="grid gap-4 sm:grid-cols-2">
                <FormField
                  label="Full name"
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
                  label="Work email"
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
                      aria-label={showPassword ? "Hide password" : "Show password"}
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
                  label="Confirm password"
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

            <div className="border-t border-(--border) pt-6 space-y-3">
              <Button
                type="submit"
                className="w-full"
                loading={isSubmitting || registerTenant.isPending}
                disabled={slugAvailable === false}
              >
                {isSubmitting || registerTenant.isPending
                  ? "Provisioning workspace..."
                  : "Initialize workspace →"}
              </Button>

              <p className="text-center font-mono text-xs text-(--muted)">
                Already have a workspace?{" "}
                {currentSlug ? (
                  <a
                    href={workspaceUrl(currentSlug, "/login")}
                    className="text-(--accent) underline-offset-2 hover:underline"
                  >
                    Sign in at {currentSlug}.nexus.com →
                  </a>
                ) : (
                  <span className="text-(--border)">enter your slug above</span>
                )}
              </p>
            </div>
          </form>
        </div>
      </div>
    </motion.div>
  )
}
