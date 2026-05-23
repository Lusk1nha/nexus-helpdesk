import { Navigate, Outlet } from "react-router"

import { ThemeSwitcher } from "@nexus/theme"

import { useIsAuthenticated } from "@/application/auth/use-session"

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
    <div className="relative flex min-h-dvh flex-col bg-(--bg)">
      {/* Dot grid background */}
      <div
        className="pointer-events-none absolute inset-0"
        style={{
          backgroundImage:
            "radial-gradient(circle, var(--border) 1px, transparent 1px)",
          backgroundSize: "28px 28px",
          opacity: 0.5,
        }}
      />

      {/* Top bar */}
      <header className="relative z-20 flex items-center justify-between border-b border-(--border)/50 px-6 py-4">
        <div className="flex items-center gap-2 font-mono text-sm">
          <span className="font-semibold text-(--accent)">◈</span>
          <span className="text-(--muted)">nexus</span>
          <span className="text-(--border)">/</span>
          <span className="text-(--fg)">helpdesk</span>
        </div>
        <ThemeSwitcher />
      </header>

      {/* Content */}
      <main className="relative z-10 flex flex-1 items-center justify-center p-6">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="relative z-10 border-t border-(--border)/50 px-6 py-4">
        <p className="text-center font-mono text-xs text-(--muted)">
          nexus_helpdesk v1.0 —{" "}
          <span className="text-(--accent)">multi-tenant</span> ·
          ai-powered · realtime
        </p>
      </footer>
    </div>
  )
}
