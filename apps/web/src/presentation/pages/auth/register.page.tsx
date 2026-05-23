import { zodResolver } from "@hookform/resolvers/zod"
import { AnimatePresence, motion } from "motion/react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { Link, useNavigate } from "react-router"
import { CheckCircle2, Eye, EyeOff } from "lucide-react"

import { Alert, Button, FormField, Input } from "@nexus/ui"

import { useRegister } from "@/application/auth/use-register"
import { registerSchema, type RegisterInput } from "@/domain/auth/auth.schemas"

export function RegisterPage() {
  const navigate = useNavigate()
  const register_ = useRegister()
  const [showPassword, setShowPassword] = useState(false)
  const [showConfirm, setShowConfirm] = useState(false)

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<RegisterInput>({
    resolver: zodResolver(registerSchema),
  })

  const onSubmit = async (data: RegisterInput) => {
    await register_.mutateAsync(data)
  }

  if (register_.isSuccess) {
    return (
      <motion.div
        initial={{ opacity: 0, scale: 0.96 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.25 }}
        className="w-full max-w-sm"
      >
        <div className="space-y-4 rounded-sm border border-(--success) bg-(--surface) px-6 py-8 text-center">
          <CheckCircle2 className="mx-auto h-10 w-10 text-(--success)" />
          <div>
            <h2 className="font-mono text-base font-semibold text-(--fg)">
              Company registered!
            </h2>
            <p className="mt-1 text-xs text-(--muted)">
              Your workspace is ready. Sign in to get started.
            </p>
          </div>
          <Button
            variant="outline"
            className="w-full"
            onClick={() => navigate("/login", { replace: true })}
          >
            Go to login →
          </Button>
        </div>
      </motion.div>
    )
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="w-full max-w-sm"
    >
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Card header */}
        <div className="border-b border-(--border) px-6 pt-6 pb-4">
          <div className="mb-4 flex items-center gap-2">
            <span className="h-2.5 w-2.5 rounded-full bg-(--destructive) opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--warning) opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-(--success) opacity-70" />
          </div>
          <p className="mb-1 font-mono text-xs text-(--muted)">
            <span className="text-(--success)">$</span> nexus register
            --new-tenant
          </p>
          <h1 className="font-mono text-lg font-semibold text-(--fg)">
            Create workspace
          </h1>
          <p className="mt-0.5 text-xs text-(--muted)">
            Set up your company's helpdesk in seconds
          </p>
        </div>

        {/* Form */}
        <form
          onSubmit={handleSubmit(onSubmit)}
          noValidate
          className="space-y-4 px-6 py-5"
        >
          <AnimatePresence mode="wait">
            {register_.isError && (
              <motion.div
                key="error"
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: "auto" }}
                exit={{ opacity: 0, height: 0 }}
                transition={{ duration: 0.15 }}
              >
                <Alert variant="error">
                  {(register_.error as Error)?.message ??
                    "Registration failed. Please try again."}
                </Alert>
              </motion.div>
            )}
          </AnimatePresence>

          {/* Section: Company */}
          <div className="space-y-3">
            <p className="font-mono text-[10px] tracking-widest text-(--accent) uppercase">
              Company
            </p>
            <FormField
              label="Company name"
              htmlFor="tenantName"
              error={errors.tenantName?.message}
              required
            >
              <Input
                id="tenantName"
                placeholder="Acme Corp"
                autoComplete="organization"
                error={!!errors.tenantName}
                {...register("tenantName")}
              />
            </FormField>
          </div>

          {/* Section: Admin account */}
          <div className="space-y-3">
            <p className="font-mono text-[10px] tracking-widest text-(--accent) uppercase">
              Admin account
            </p>
            <FormField
              label="Full name"
              htmlFor="adminFullName"
              error={errors.adminFullName?.message}
              required
            >
              <Input
                id="adminFullName"
                placeholder="Jane Doe"
                autoComplete="name"
                error={!!errors.adminFullName}
                {...register("adminFullName")}
              />
            </FormField>
            <FormField
              label="Email"
              htmlFor="adminEmail"
              error={errors.adminEmail?.message}
              required
            >
              <Input
                id="adminEmail"
                type="email"
                placeholder="admin@company.com"
                autoComplete="email"
                error={!!errors.adminEmail}
                {...register("adminEmail")}
              />
            </FormField>
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
                  placeholder="min. 8 characters"
                  autoComplete="new-password"
                  error={!!errors.adminPassword}
                  className="pr-9"
                  {...register("adminPassword")}
                />
                <button
                  type="button"
                  onClick={() => setShowPassword((v) => !v)}
                  className="absolute top-1/2 right-2.5 -translate-y-1/2 text-(--muted) transition-colors hover:text-(--fg)"
                  tabIndex={-1}
                >
                  {showPassword ? (
                    <EyeOff className="h-3.5 w-3.5" />
                  ) : (
                    <Eye className="h-3.5 w-3.5" />
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
              <div className="relative">
                <Input
                  id="confirmPassword"
                  type={showConfirm ? "text" : "password"}
                  placeholder="repeat password"
                  autoComplete="new-password"
                  error={!!errors.confirmPassword}
                  className="pr-9"
                  {...register("confirmPassword")}
                />
                <button
                  type="button"
                  onClick={() => setShowConfirm((v) => !v)}
                  className="absolute top-1/2 right-2.5 -translate-y-1/2 text-(--muted) transition-colors hover:text-(--fg)"
                  tabIndex={-1}
                >
                  {showConfirm ? (
                    <EyeOff className="h-3.5 w-3.5" />
                  ) : (
                    <Eye className="h-3.5 w-3.5" />
                  )}
                </button>
              </div>
            </FormField>
          </div>

          <Button
            type="submit"
            className="w-full"
            loading={register_.isPending}
          >
            {register_.isPending
              ? "Creating workspace..."
              : "Create workspace →"}
          </Button>
        </form>

        {/* Footer */}
        <div className="px-6 pb-5">
          <p className="text-center font-mono text-xs text-(--muted)">
            Already have an account?{" "}
            <Link
              to="/login"
              className="text-(--accent) underline-offset-2 hover:underline"
            >
              Sign in
            </Link>
          </p>
        </div>
      </div>
    </motion.div>
  )
}
