import { zodResolver } from '@hookform/resolvers/zod'
import { AnimatePresence, motion } from 'motion/react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { Link, useNavigate } from 'react-router'
import { Eye, EyeOff } from 'lucide-react'

import { useLogin } from '@/application/auth/use-login'
import { loginSchema, type LoginInput } from '@/domain/auth/auth.schemas'
import { Alert } from '@/presentation/components/ui/alert'
import { Button } from '@/presentation/components/ui/button'
import { FormField } from '@/presentation/components/ui/form-field'
import { Input } from '@/presentation/components/ui/input'

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
    navigate('/app/tickets', { replace: true })
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: 'easeOut' }}
      className="w-full max-w-sm"
    >
      {/* Card */}
      <div className="overflow-hidden rounded-sm border border-[var(--border)] bg-[var(--surface)]">
        {/* Card header */}
        <div className="border-b border-[var(--border)] px-6 pt-6 pb-4">
          <div className="mb-4 flex items-center gap-2">
            {/* Terminal dots */}
            <span className="h-2.5 w-2.5 rounded-full bg-[var(--destructive)] opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-[var(--warning)] opacity-70" />
            <span className="h-2.5 w-2.5 rounded-full bg-[var(--success)] opacity-70" />
          </div>
          <p className="mb-1 font-mono text-xs text-[var(--muted)]">
            <span className="text-[var(--success)]">$</span> nexus authenticate
          </p>
          <h1 className="font-mono text-lg font-semibold text-[var(--fg)]">Sign in</h1>
          <p className="mt-0.5 text-xs text-[var(--muted)]">Access your helpdesk workspace</p>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit(onSubmit)} noValidate className="space-y-4 px-6 py-5">
          <AnimatePresence mode="wait">
            {login.isError && (
              <motion.div
                key="error"
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                transition={{ duration: 0.15 }}
              >
                <Alert variant="error">
                  {(login.error as Error)?.message ?? 'Invalid credentials. Please try again.'}
                </Alert>
              </motion.div>
            )}
          </AnimatePresence>

          <FormField label="Email" htmlFor="email" error={errors.email?.message} required>
            <Input
              id="email"
              type="email"
              placeholder="you@company.com"
              autoComplete="email"
              error={!!errors.email}
              {...register('email')}
            />
          </FormField>

          <FormField label="Password" htmlFor="password" error={errors.password?.message} required>
            <div className="relative">
              <Input
                id="password"
                type={showPassword ? 'text' : 'password'}
                placeholder="••••••••"
                autoComplete="current-password"
                error={!!errors.password}
                className="pr-9"
                {...register('password')}
              />
              <button
                type="button"
                onClick={() => setShowPassword((v) => !v)}
                className="absolute top-1/2 right-2.5 -translate-y-1/2 text-[var(--muted)] transition-colors hover:text-[var(--fg)]"
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

          <Button type="submit" className="w-full" loading={login.isPending}>
            {login.isPending ? 'Authenticating...' : 'Authenticate →'}
          </Button>
        </form>

        {/* Footer */}
        <div className="px-6 pb-5">
          <p className="text-center font-mono text-xs text-[var(--muted)]">
            No account?{' '}
            <Link
              to="/register"
              className="text-[var(--accent)] underline-offset-2 hover:underline"
            >
              Register your company
            </Link>
          </p>
        </div>
      </div>

      {/* Hint below card */}
      <p className="mt-4 text-center font-mono text-xs text-[var(--border)]">
        <span className="text-[var(--muted)]">tip:</span> use your company credentials
      </p>
    </motion.div>
  )
}
