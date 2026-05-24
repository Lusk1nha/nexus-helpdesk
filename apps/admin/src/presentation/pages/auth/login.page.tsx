import { zodResolver } from "@hookform/resolvers/zod"
import { motion } from "motion/react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { useNavigate } from "react-router"
import { EyeIcon, EyeClosedIcon } from "@phosphor-icons/react"

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
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="w-full max-w-sm"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        <div className="border-b border-(--border) px-6 pt-6 pb-4">
          <div className="mb-4 flex items-center gap-2">
            <span className="h-2.5 w-2.5 rounded-full bg-(--destructive) opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--warning) opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--success) opacity-70" />
          </div>
          <p className="mb-1 font-mono text-xs text-(--muted)">
            <span className="text-(--success)">$</span> nexus admin
            authenticate
          </p>
          <h1 className="font-mono text-lg font-semibold text-(--fg)">
            Admin sign in
          </h1>
          <p className="mt-0.5 text-xs text-(--muted)">
            Restricted to admin accounts only
          </p>
        </div>

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
            {login.isPending ? "Authenticating..." : "Authenticate →"}
          </Button>
        </form>
      </div>

      <p className="mt-4 text-center font-mono text-xs text-(--border)">
        <span className="text-(--muted)">restricted:</span> admin accounts only
      </p>
    </motion.div>
  )
}
