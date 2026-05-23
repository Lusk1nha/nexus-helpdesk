import {
  LayoutDashboard,
  LogOut,
  MessageSquare,
  Settings,
  Shield,
  BookOpen,
} from "lucide-react"
import { NavLink, Navigate, Outlet, useNavigate } from "react-router"

import { ThemeSwitcher } from "@nexus/theme"
import { cn } from "@nexus/ui"

import {
  useIsAuthenticated,
  useLogout,
  useSession,
} from "@/application/auth/use-session"

/**
 * Authenticated application shell with sidebar navigation.
 * Redirects to /login if the user is not authenticated.
 */
export function AppLayout() {
  const isAuthenticated = useIsAuthenticated()
  const user = useSession()
  const logout = useLogout()
  const navigate = useNavigate()

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />
  }

  const handleLogout = () => {
    logout()
    navigate("/login", { replace: true })
  }

  const navItems = [
    { to: "/app/tickets", icon: MessageSquare, label: "tickets" },

    ...(user?.role !== "customer"
      ? [{ to: "/app/knowledge", icon: BookOpen, label: "knowledge" }]
      : []),

    ...(user?.role === "admin"
      ? [{ to: "/app/admin", icon: Shield, label: "admin" }]
      : []),
  ]

  return (
    <div className="flex min-h-dvh bg-[var(--bg)]">
      {/* Sidebar */}
      <aside className="flex w-56 shrink-0 flex-col border-r border-[var(--border)] bg-[var(--surface)]">
        {/* Brand */}
        <div className="flex items-center gap-2 border-b border-[var(--border)] px-4 py-4">
          <span className="text-sm font-semibold text-[var(--accent)]">◈</span>
          <span className="font-mono text-sm font-medium text-[var(--fg)]">
            nexus
          </span>
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
                    ? "bg-[var(--accent)]/10 text-[var(--accent)]"
                    : "text-[var(--muted)] hover:bg-[var(--surface-2)] hover:text-[var(--fg)]"
                )
              }
            >
              <Icon className="h-3.5 w-3.5 shrink-0" />
              {label}
            </NavLink>
          ))}
        </nav>

        {/* User + logout */}
        <div className="space-y-2 border-t border-[var(--border)] p-3">
          <div className="px-2 py-1">
            <p className="truncate font-mono text-xs text-[var(--muted)]">
              {user?.role}
            </p>
          </div>
          <button
            onClick={handleLogout}
            className={cn(
              "flex w-full items-center gap-2.5 rounded-sm px-3 py-2",
              "font-mono text-xs text-[var(--muted)]",
              "transition-colors hover:bg-[var(--destructive)]/10 hover:text-[var(--destructive)]"
            )}
          >
            <LogOut className="h-3.5 w-3.5 shrink-0" />
            logout
          </button>
        </div>
      </aside>

      {/* Main area */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top bar */}
        <header className="flex items-center justify-between border-b border-[var(--border)] bg-[var(--surface)] px-6 py-3">
          <div className="flex items-center gap-1 font-mono text-xs text-[var(--muted)]">
            <LayoutDashboard className="h-3.5 w-3.5" />
            <span className="ml-1">dashboard</span>
          </div>
          <div className="flex items-center gap-3">
            <ThemeSwitcher />
            <button className="text-[var(--muted)] transition-colors hover:text-[var(--fg)]">
              <Settings className="h-4 w-4" />
            </button>
          </div>
        </header>

        {/* Page content */}
        <main className="flex-1 overflow-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
