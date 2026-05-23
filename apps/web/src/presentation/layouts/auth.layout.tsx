import { Navigate, Outlet } from 'react-router'

import { useIsAuthenticated } from '@/application/auth/use-session'
import { ThemeSwitcher } from '@/presentation/components/theme/theme-switcher'

/**
 * Wraps public auth pages (login, register).
 * Redirects to /app/tickets if the user is already authenticated.
 */
export function AuthLayout() {
  const isAuthenticated = useIsAuthenticated()

  if (isAuthenticated) {
    return <Navigate to="/app/tickets" replace />
  }

  return (
    <div className="relative min-h-dvh flex flex-col bg-[var(--bg)]">
      {/* Dot grid background */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          backgroundImage:
            'radial-gradient(circle, var(--border) 1px, transparent 1px)',
          backgroundSize: '28px 28px',
          opacity: 0.5,
        }}
      />

      {/* Top bar */}
      <header className="relative z-20 flex items-center justify-between px-6 py-4 border-b border-[var(--border)]/50">
        <div className="flex items-center gap-2 font-mono text-sm">
          <span className="text-[var(--accent)] font-semibold">◈</span>
          <span className="text-[var(--muted)]">nexus</span>
          <span className="text-[var(--border)]">/</span>
          <span className="text-[var(--fg)]">helpdesk</span>
        </div>
        <ThemeSwitcher />
      </header>

      {/* Content */}
      <main className="relative z-10 flex flex-1 items-center justify-center p-6">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="relative z-10 py-4 px-6 border-t border-[var(--border)]/50">
        <p className="text-center text-xs text-[var(--muted)] font-mono">
          nexus_helpdesk v1.0 —{' '}
          <span className="text-[var(--accent)]">multi-tenant</span> · ai-powered · realtime
        </p>
      </footer>
    </div>
  )
}
