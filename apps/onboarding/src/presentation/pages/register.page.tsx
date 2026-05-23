import { zodResolver } from "@hookform/resolvers/zod"
import { motion } from "motion/react"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import { EyeIcon, EyeClosedIcon, GlobeIcon } from "@phosphor-icons/react"

import { registerSchema, type RegisterInput } from "@nexus/auth"
import { Button, FormError, FormField, Input } from "@nexus/ui"

import { useRegister } from "@/application/auth/use-register"
import { generateSlug } from "@nexus/utils"

export function RegisterPage() {
  const registerTenant = useRegister()
  const [showPassword, setShowPassword] = useState(false)

  const {
    register,
    handleSubmit,
    watch,
    setValue,
    formState: { errors, isSubmitting, dirtyFields },
  } = useForm<RegisterInput>({
    resolver: zodResolver(registerSchema),
    defaultValues: {
      tenantName: "",
      tenantSlug: "",
    },
  })

  const currentSlug = watch("tenantSlug")
  const tenantName = watch("tenantName")

  useEffect(() => {
    if (!dirtyFields.tenantSlug) {
      const suggestedSlug = generateSlug(tenantName || "")

      setValue("tenantSlug", suggestedSlug, {
        shouldValidate: false,
        shouldDirty: false,
      })
    }
  }, [tenantName, dirtyFields.tenantSlug, setValue])

  const onSubmit = async (data: RegisterInput) => {
    try {
      // 1. Chama a API em Rust via TanStack Query
      const { tenantSlug } = await registerTenant.mutateAsync(data)

      // 2. Redirecionamento Mágico para o Workspace criado
      const protocol = window.location.protocol

      // Monta a URL: http://[slug].localhost:5173/login (ou .nexus.com em prod)
      window.location.href = `${protocol}//${tenantSlug}.localhost:5173/login?registered=true`
    } catch (error) {
      // O React Query já vai setar registerTenant.isError como true
      console.error("Falha ao provisionar o Tenant:", error)
    }
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="w-full max-w-xl"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Header imitando terminal */}
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
          {/* Tratativa visual de erro da API */}
          <FormError
            error={registerTenant.error}
            fallbackMessage="It was not possible to proceed with this request!"
          />

          <div className="space-y-8">
            {/* Secão 1: Organização */}
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
                  error={errors.tenantSlug?.message}
                  required
                >
                  <div className="relative">
                    <GlobeIcon className="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-(--muted)" />
                    <Input
                      id="tenantSlug"
                      placeholder="acme"
                      className="pl-9 font-mono text-sm"
                      error={!!errors.tenantSlug}
                      {...register("tenantSlug")}
                    />
                  </div>
                </FormField>
              </div>

              {/* Preview da URL */}
              <div className="rounded border border-(--border)/50 bg-(--bg) p-3 font-mono text-xs text-(--muted)">
                Your workspace will be accessible at:{" "}
                <span className="text-(--accent)">
                  {currentSlug ? currentSlug.toLowerCase() : "<slug>"}
                </span>
                .nexus.localhost
              </div>
            </section>

            <hr className="border-(--border)/50" />

            {/* Secão 2: Admin */}
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
            >
              {isSubmitting || registerTenant.isPending
                ? "Provisioning Workspace..."
                : "Initialize Workspace →"}
            </Button>
          </div>
        </form>
      </div>
    </motion.div>
  )
}
