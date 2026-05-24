import {
  BookOpenIcon,
  ChatTextIcon,
  ShieldIcon,
  SignOutIcon,
} from "@phosphor-icons/react"
import { NavLink, Navigate, Outlet, useNavigate } from "react-router"

import { ThemeSwitcher } from "@nexus/theme"
import { cn } from "@nexus/utils"

import {
  useIsAuthenticated,
  useLogout,
  useSession,
} from "@/application/auth/use-session"
import { useTenantSlug } from "@/application/tenant/use-tenant-slug"
import { useTenantBranding } from "@/application/tenant/use-tenant-branding"
import { NoTenantPage } from "@/presentation/pages/no-tenant/no-tenant.page"
import { paths } from "@/presentation/router/paths"

export function AppLayout() {
  const slug = useTenantSlug()
  const isAuthenticated = useIsAuthenticated()
  const user = useSession()
  const { data: branding } = useTenantBranding(slug)
  const logout = useLogout()
  const navigate = useNavigate()

  if (!slug) return <NoTenantPage />

  if (!isAuthenticated) {
    return <Navigate to={paths.login} replace />
  }

  const handleLogout = () => {
    logout()
    navigate(paths.login, { replace: true })
  }

  const navItems = [
    { to: paths.app.tickets, icon: ChatTextIcon, label: "tickets" },

    ...(user?.role !== "customer"
      ? [{ to: paths.app.knowledge, icon: BookOpenIcon, label: "knowledge" }]
      : []),

    ...(user?.role === "admin"
      ? [{ to: paths.app.admin, icon: ShieldIcon, label: "admin" }]
      : []),
  ]

  const roleColors: Record<string, string> = {
    admin: "text-(--accent)",
    agent: "text-(--success)",
    customer: "text-(--muted)",
  }

  return (
    <div className="flex min-h-dvh bg-(--bg)">
      {/* Sidebar */}
      <aside className="flex w-52 shrink-0 flex-col border-r border-(--border) bg-(--surface)">
        {/* Brand + tenant */}
        <div className="border-b border-(--border) px-4 py-4">
          <div className="flex items-center gap-2 mb-2">
            <span className="text-sm font-semibold text-(--accent)">◈</span>
            <span className="font-mono text-sm font-medium text-(--fg)">nexus</span>
          </div>
          {branding && (
            <p className="font-mono text-[11px] text-(--muted) truncate" title={branding.name}>
              {branding.name}
            </p>
          )}
        </div>

        {/* Nav */}
        <nav className="flex-1 space-y-0.5 px-2 py-3">
          {navItems.map(({ to, icon: Icon, label }) => (
            <NavLink
              key={to}
              to={to}
              className={({ isActive }) =>
                cn(
                  "flex items-center gap-2.5 rounded-sm px-3 py-2",
                  "font-mono text-xs transition-colors",
                  isActive
                    ? "bg-(--accent)/10 text-(--accent)"
                    : "text-(--muted) hover:bg-(--surface-2) hover:text-(--fg)"
                )
              }
            >
              <Icon className="h-3.5 w-3.5 shrink-0" />
              {label}
            </NavLink>
          ))}
        </nav>

        {/* User footer */}
        <div className="border-t border-(--border) p-3 space-y-1">
          <div className="px-3 py-2">
            <p className={cn("font-mono text-xs font-medium", roleColors[user?.role ?? ""] ?? "text-(--muted)")}>
              {user?.role}
            </p>
            <p className="font-mono text-[10px] text-(--border) truncate mt-0.5">
              {slug}.nexus
            </p>
          </div>
          <button
            onClick={handleLogout}
            className={cn(
              "flex w-full items-center gap-2.5 rounded-sm px-3 py-2",
              "font-mono text-xs text-(--muted)",
              "transition-colors hover:bg-(--destructive)/10 hover:text-(--destructive)"
            )}
          >
            <SignOutIcon className="h-3.5 w-3.5 shrink-0" />
            sign out
          </button>
        </div>
      </aside>

      {/* Main area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top bar */}
        <header className="flex items-center justify-end border-b border-(--border) bg-(--surface) px-5 py-3">
          <ThemeSwitcher />
        </header>

        {/* Page content */}
        <main className="flex-1 overflow-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
