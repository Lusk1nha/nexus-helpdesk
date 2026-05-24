import { zodResolver } from "@hookform/resolvers/zod"
import { ShieldCheckIcon, EyeIcon, EyeClosedIcon } from "@phosphor-icons/react"
import { motion } from "motion/react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { useNavigate } from "react-router"

import { Button, FormError, FormField, Input } from "@nexus/ui"
import { loginSchema, type LoginInput } from "@nexus/auth"

import { useLogin } from "@/application/auth/use-login"
import { paths } from "@/presentation/router/paths"

export function LoginPage() {
  const navigate = useNavigate()
  const login = useLogin()
  const [showPassword, setShowPassword] = useState(false)

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginInput>({
    resolver: zodResolver(loginSchema),
  })

  const onSubmit = async (data: LoginInput) => {
    await login.mutateAsync(data)
    navigate(paths.app.tenant, { replace: true })
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 12 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.25, ease: "easeOut" }}
      className="w-full max-w-sm"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Header */}
        <div className="border-b border-(--border) px-6 pt-7 pb-5">
          <div className="mb-5 flex h-9 w-9 items-center justify-center rounded-sm bg-(--accent)/10">
            <ShieldCheckIcon className="h-4.5 w-4.5 text-(--accent)" weight="duotone" />
          </div>
          <h1 className="font-mono text-lg font-semibold text-(--fg)">
            Admin sign in
          </h1>
          <p className="mt-1 text-xs text-(--muted)">
            Restricted to admin accounts only
          </p>
        </div>

        {/* Form */}
        <form
          onSubmit={handleSubmit(onSubmit)}
          noValidate
          className="space-y-4 px-6 py-6"
        >
          <FormError
            error={login.error}
            fallbackMessage="Invalid credentials."
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
              placeholder="admin@company.com"
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
            {login.isPending ? "Signing in..." : "Sign in"}
          </Button>
        </form>
      </div>

      <p className="mt-4 text-center font-mono text-[11px] text-(--border)">
        All access attempts are logged and audited
      </p>
    </motion.div>
  )
}
