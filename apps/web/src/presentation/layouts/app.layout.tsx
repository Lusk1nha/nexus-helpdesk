import {
  BookOpenIcon,
  ChatTextIcon,
  GaugeIcon,
  ListIcon,
  ShieldIcon,
  SignOutIcon,
  XIcon,
} from "@phosphor-icons/react"
import { motion, AnimatePresence } from "motion/react"
import { useState } from "react"
import { NavLink, Navigate, Outlet, useNavigate } from "react-router"

import { ThemeSwitcher } from "@nexus/theme"
import { cn } from "@nexus/utils"

import {
  useIsAuthenticated,
  useLogout,
  useSession,
} from "@/application/auth/use-session"
import { useAuthInit } from "@/application/auth/use-auth-init"
import { useTokenRefreshScheduler } from "@/application/auth/use-token-refresh"
import { useTenantSlug } from "@/application/tenant/use-tenant-slug"
import { useTenantBranding } from "@/application/tenant/use-tenant-branding"
import { NoTenantPage } from "@/presentation/pages/no-tenant/no-tenant.page"
import { paths } from "@/presentation/router/paths"

const roleColors: Record<string, string> = {
  admin: "text-(--accent)",
  agent: "text-(--success)",
  customer: "text-(--muted)",
}

function SidebarContent({
  onClose,
}: {
  onClose?: () => void
}) {
  const slug = useTenantSlug()
  const user = useSession()
  const { data: branding } = useTenantBranding(slug)
  const logout = useLogout()
  const navigate = useNavigate()

  const handleLogout = () => {
    logout()
    navigate(paths.login, { replace: true })
  }

  const navItems = [
    { to: paths.app.dashboard, icon: GaugeIcon, label: "overview" },
    { to: paths.app.tickets, icon: ChatTextIcon, label: "tickets" },
    ...(user?.role !== "customer"
      ? [{ to: paths.app.knowledge, icon: BookOpenIcon, label: "knowledge" }]
      : []),
    ...(user?.role === "admin"
      ? [{ to: paths.app.admin, icon: ShieldIcon, label: "admin" }]
      : []),
  ]

  return (
    <div className="flex h-full flex-col">
      {/* Brand */}
      <div className="border-b border-(--border) px-4 py-4">
        <div className="flex items-center justify-between">
          <div>
            <div className="mb-1 flex items-center gap-2">
              <span className="text-sm font-semibold text-(--accent)">◈</span>
              <span className="font-mono text-sm font-medium text-(--fg)">
                nexus
              </span>
            </div>
            {branding && (
              <p
                className="truncate font-mono text-[11px] text-(--muted)"
                title={branding.name}
              >
                {branding.name}
              </p>
            )}
          </div>
          {onClose && (
            <button
              onClick={onClose}
              className="p-1 rounded-sm text-(--muted) hover:text-(--fg) hover:bg-(--surface-2) transition-colors"
            >
              <XIcon className="h-4 w-4" />
            </button>
          )}
        </div>
      </div>

      {/* Nav */}
      <nav className="flex-1 space-y-0.5 px-2 py-3">
        {navItems.map(({ to, icon: Icon, label }) => (
          <NavLink
            key={to}
            to={to}
            end={label === "tickets"}
            onClick={onClose}
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
      <div className="space-y-1 border-t border-(--border) p-3">
        <div className="px-3 py-2">
          <p
            className={cn(
              "font-mono text-xs font-medium",
              roleColors[user?.role ?? ""] ?? "text-(--muted)"
            )}
          >
            {user?.role}
          </p>
          <p className="mt-0.5 truncate font-mono text-[10px] text-(--border)">
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
    </div>
  )
}

export function AppLayout() {
  const slug = useTenantSlug()
  const isAuthenticated = useIsAuthenticated()
  const authReady = useAuthInit()
  const [mobileOpen, setMobileOpen] = useState(false)

  useTokenRefreshScheduler()

  if (!slug) return <NoTenantPage />
  if (!authReady) return null
  if (!isAuthenticated) return <Navigate to={paths.login} replace />

  return (
    <div className="flex min-h-dvh bg-(--bg)">
      {/* Desktop sidebar */}
      <aside className="hidden md:flex w-52 shrink-0 flex-col border-r border-(--border) bg-(--surface)">
        <SidebarContent />
      </aside>

      {/* Mobile sidebar overlay */}
      <AnimatePresence>
        {mobileOpen && (
          <>
            <motion.div
              key="overlay"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={() => setMobileOpen(false)}
              className="fixed inset-0 z-40 bg-black/60 backdrop-blur-[2px] md:hidden"
            />
            <motion.aside
              key="mobile-sidebar"
              initial={{ x: -208 }}
              animate={{ x: 0 }}
              exit={{ x: -208 }}
              transition={{ type: "spring", stiffness: 400, damping: 40 }}
              className="fixed inset-y-0 left-0 z-50 w-52 border-r border-(--border) bg-(--surface) md:hidden"
            >
              <SidebarContent onClose={() => setMobileOpen(false)} />
            </motion.aside>
          </>
        )}
      </AnimatePresence>

      {/* Main area */}
      <div className="flex flex-1 flex-col overflow-hidden min-w-0">
        {/* Top bar */}
        <header className="flex items-center justify-between border-b border-(--border) bg-(--surface) px-4 py-3">
          <button
            onClick={() => setMobileOpen(true)}
            className="p-1.5 rounded-sm text-(--muted) hover:text-(--fg) hover:bg-(--surface-2) transition-colors md:hidden"
          >
            <ListIcon className="h-4 w-4" />
          </button>
          <div className="flex-1 md:flex-none" />
          <ThemeSwitcher />
        </header>

        {/* Page content */}
        <main className="flex-1 overflow-auto p-4 md:p-6">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
