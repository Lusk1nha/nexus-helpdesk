import { zodResolver } from "@hookform/resolvers/zod"
import { EyeIcon, EyeClosedIcon } from "@phosphor-icons/react"
import { motion } from "motion/react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { Link, useNavigate } from "react-router"

import { Button, FormError, FormField, Input } from "@nexus/ui"

import { useLogin } from "@/application/auth/use-login"
import { loginSchema, type LoginInput } from "@nexus/auth"
import { useTenantBranding } from "@/application/tenant/use-tenant-branding"
import { useTenantSlug } from "@/application/tenant/use-tenant-slug"
import { paths } from "@/presentation/router/paths"

export function LoginPage() {
  const navigate = useNavigate()
  const login = useLogin()
  const [showPassword, setShowPassword] = useState(false)

  const slug = useTenantSlug()
  const { data: branding } = useTenantBranding(slug)

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginInput>({
    resolver: zodResolver(loginSchema),
  })

  const onSubmit = async (data: LoginInput) => {
    await login.mutateAsync(data)
    navigate(paths.app.tickets, { replace: true })
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="w-full max-w-sm"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Accent top strip */}
        <div className="h-0.5 w-full bg-(--accent)" />

        {/* Card header */}
        <div className="border-b border-(--border) px-6 pt-6 pb-5">
          <div className="mb-1 flex items-center gap-2">
            <span className="font-mono text-xs font-semibold text-(--accent)">
              ◈
            </span>
            <span className="font-mono text-xs text-(--muted)">
              {branding?.name ?? slug ?? "nexus"}
            </span>
          </div>
          <h1 className="font-mono text-lg font-semibold text-(--fg)">
            Welcome back
          </h1>
          <p className="mt-0.5 text-xs text-(--muted)">
            Sign in to your workspace
          </p>
        </div>

        {/* Form */}
        <form
          onSubmit={handleSubmit(onSubmit)}
          noValidate
          className="space-y-4 px-6 py-5"
        >
          <FormError
            error={login.error}
            fallbackMessage="Invalid credentials. Please try again."
          />

          <FormField
            label="Email"
            htmlFor="email"
            error={errors.email?.message}
            required
          >
            <Input
              id="email"
              type="email"
              placeholder="you@company.com"
              autoComplete="email"
              error={!!errors.email}
              {...register("email")}
            />
          </FormField>

          <FormField
            label="Password"
            htmlFor="password"
            error={errors.password?.message}
            required
          >
            <div className="relative">
              <Input
                id="password"
                type={showPassword ? "text" : "password"}
                placeholder="••••••••"
                autoComplete="current-password"
                error={!!errors.password}
                className="pr-9"
                {...register("password")}
              />
              <button
                type="button"
                onClick={() => setShowPassword((v) => !v)}
                className="absolute top-1/2 right-2.5 -translate-y-1/2 text-(--muted) transition-colors hover:text-(--fg)"
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

          <Button type="submit" className="w-full" loading={login.isPending}>
            {login.isPending ? "Signing in..." : "Sign in →"}
          </Button>

          <p className="text-center font-mono text-[11px] text-(--muted)">
            Forgot your password? Contact your workspace admin to reset it.
          </p>
        </form>

        {/* Footer: customer signup */}
        <div className="border-t border-(--border) px-6 py-4 text-center">
          <span className="font-mono text-xs text-(--muted)">
            New here?{" "}
          </span>
          <Link
            to={paths.register}
            className="font-mono text-xs text-(--accent) hover:underline"
          >
            Create an account
          </Link>
        </div>
      </div>

      <p className="mt-4 text-center font-mono text-xs text-(--border)">
        <span className="text-(--muted)">workspace:</span>{" "}
        {branding?.name ?? slug}
      </p>
    </motion.div>
  )
}
