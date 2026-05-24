import { Navigate, Outlet } from "react-router"

import { ThemeSwitcher } from "@nexus/theme"

import { useIsAuthenticated } from "@/application/auth/use-session"
import { paths } from "@/presentation/router/paths"

export function AuthLayout() {
  const isAuthenticated = useIsAuthenticated()

  if (isAuthenticated) {
    return <Navigate to={paths.app.tenant} replace />
  }

  return (
    <div className="relative flex min-h-dvh flex-col bg-(--bg)">
      <div
        className="pointer-events-none absolute inset-0"
        style={{
          backgroundImage:
            "radial-gradient(circle, var(--border) 1px, transparent 1px)",
          backgroundSize: "28px 28px",
          opacity: 0.5,
        }}
      />

      <header className="relative z-20 flex items-center justify-between border-b border-(--border)/50 px-6 py-4">
        <div className="flex items-center gap-2 font-mono text-sm">
          <span className="font-semibold text-(--accent)">◈</span>
          <span className="text-(--muted)">nexus</span>
          <span className="text-(--border)">/</span>
          <span className="text-(--fg)">admin</span>
        </div>
        <ThemeSwitcher />
      </header>

      <main className="relative z-10 flex flex-1 items-center justify-center p-6">
        <Outlet />
      </main>

      <footer className="relative z-10 flex flex-col items-center justify-center gap-1.5 border-t border-(--border)/50 px-6 py-4">
        <p className="text-center font-mono text-xs text-(--muted)">
          nexus_helpdesk admin —{" "}
          <span className="text-(--accent)">restricted access</span>
        </p>
      </footer>
    </div>
  )
}
