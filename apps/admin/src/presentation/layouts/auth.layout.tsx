import { Navigate, Outlet } from "react-router"
import { GearSixIcon, UsersIcon, BookOpenIcon } from "@phosphor-icons/react"

import { ThemeSwitcher } from "@nexus/theme"

import { useIsAuthenticated } from "@/application/auth/use-session"
import { paths } from "@/presentation/router/paths"

const FEATURES = [
  { icon: UsersIcon, label: "Manage agents and roles" },
  { icon: BookOpenIcon, label: "Curate the knowledge base" },
  { icon: GearSixIcon, label: "Configure workspace settings" },
]

export function AuthLayout() {
  const isAuthenticated = useIsAuthenticated()

  if (isAuthenticated) {
    return <Navigate to={paths.app.tenant} replace />
  }

  return (
    <div className="flex min-h-dvh bg-(--bg)">
      {/* Left panel */}
      <div className="hidden lg:flex lg:w-[420px] lg:shrink-0 flex-col justify-between bg-(--surface) border-r border-(--border) px-10 py-12">
        <div>
          <div className="flex items-center gap-2.5 mb-12">
            <span className="text-xl font-semibold text-(--accent)">◈</span>
            <span className="font-mono text-base font-medium text-(--fg)">nexus</span>
            <span className="rounded-sm bg-(--accent)/15 px-2 py-0.5 font-mono text-[11px] font-medium text-(--accent)">
              admin
            </span>
          </div>

          <h1 className="font-mono text-2xl font-semibold text-(--fg) leading-tight mb-3">
            Admin Console
          </h1>
          <p className="font-mono text-sm text-(--muted) leading-relaxed mb-10">
            Restricted access panel for tenant administrators. Manage your workspace, team, and knowledge base from one place.
          </p>

          <div className="space-y-4">
            {FEATURES.map(({ icon: Icon, label }) => (
              <div key={label} className="flex items-center gap-3">
                <div className="flex h-7 w-7 shrink-0 items-center justify-center rounded-sm bg-(--accent)/10">
                  <Icon className="h-3.5 w-3.5 text-(--accent)" />
                </div>
                <span className="font-mono text-xs text-(--muted)">{label}</span>
              </div>
            ))}
          </div>
        </div>

        <div className="border-t border-(--border) pt-6">
          <p className="font-mono text-[11px] text-(--border)">
            nexus_helpdesk · admin console
          </p>
        </div>
      </div>

      {/* Right panel */}
      <div className="flex flex-1 flex-col">
        <header className="flex items-center justify-end border-b border-(--border) px-6 py-4">
          <ThemeSwitcher />
        </header>

        <main className="flex flex-1 items-center justify-center p-8">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
