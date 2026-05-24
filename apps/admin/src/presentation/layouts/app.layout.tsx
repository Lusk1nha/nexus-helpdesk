import {
  BookOpenIcon,
  BuildingsIcon,
  LayoutIcon,
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
import { paths } from "@/presentation/router/paths"

export function AppLayout() {
  const isAuthenticated = useIsAuthenticated()
  const user = useSession()
  const logout = useLogout()
  const navigate = useNavigate()

  if (!isAuthenticated) {
    return <Navigate to={paths.login} replace />
  }

  if (user?.role !== "admin") {
    return (
      <div className="flex min-h-dvh items-center justify-center bg-(--bg)">
        <div className="text-center font-mono">
          <p className="text-sm text-(--destructive)">access denied</p>
          <p className="mt-1 text-xs text-(--muted)">
            admin role required
          </p>
        </div>
      </div>
    )
  }

  const handleLogout = () => {
    logout()
    navigate(paths.login, { replace: true })
  }

  const navItems = [
    { to: paths.app.tenant, icon: BuildingsIcon, label: "tenant" },
    { to: paths.app.knowledge, icon: BookOpenIcon, label: "knowledge" },
  ]

  return (
    <div className="flex min-h-dvh bg-(--bg)">
      <aside className="flex w-56 shrink-0 flex-col border-r border-(--border) bg-(--surface)">
        <div className="flex items-center gap-2 border-b border-(--border) px-4 py-4">
          <span className="text-sm font-semibold text-(--accent)">◈</span>
          <span className="font-mono text-sm font-medium text-(--fg)">
            nexus
          </span>
          <span className="ml-auto rounded-sm bg-(--accent)/10 px-1.5 py-0.5 font-mono text-[10px] text-(--accent)">
            admin
          </span>
        </div>

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

        <div className="space-y-2 border-t border-(--border) p-3">
          <div className="px-2 py-1">
            <p className="truncate font-mono text-xs text-(--muted)">
              {user?.role}
            </p>
            <p className="truncate font-mono text-[10px] text-(--border)">
              {user?.tenantId}
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
            logout
          </button>
        </div>
      </aside>

      <div className="flex flex-1 flex-col overflow-hidden">
        <header className="flex items-center justify-between border-b border-(--border) bg-(--surface) px-6 py-3">
          <div className="flex items-center gap-1 font-mono text-xs text-(--muted)">
            <LayoutIcon className="h-3.5 w-3.5" />
            <span className="ml-1">admin panel</span>
          </div>
          <ThemeSwitcher />
        </header>

        <main className="flex-1 overflow-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
