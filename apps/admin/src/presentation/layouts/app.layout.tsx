import {
  BookOpenIcon,
  BuildingsIcon,
  CaretDownIcon,
  ListIcon,
  SignOutIcon,
  UserCircleIcon,
  UsersIcon,
  XIcon,
} from "@phosphor-icons/react"
import { motion, AnimatePresence } from "motion/react"
import { useState, useRef, useEffect } from "react"
import { NavLink, Navigate, Outlet, useNavigate } from "react-router"

import { ThemeSwitcher } from "@nexus/theme"
import { cn } from "@nexus/utils"

import {
  useIsAuthenticated,
  useLogout,
  useSession,
} from "@/application/auth/use-session"
import { paths } from "@/presentation/router/paths"

const NAV_ITEMS = [
  { to: paths.app.tenant, icon: BuildingsIcon, label: "Tenant" },
  { to: paths.app.knowledge, icon: BookOpenIcon, label: "Knowledge" },
  { to: paths.app.users, icon: UsersIcon, label: "Users" },
]

export function AppLayout() {
  const isAuthenticated = useIsAuthenticated()
  const user = useSession()
  const logout = useLogout()
  const navigate = useNavigate()
  const [userMenuOpen, setUserMenuOpen] = useState(false)
  const [mobileNavOpen, setMobileNavOpen] = useState(false)
  const menuRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setUserMenuOpen(false)
      }
    }
    document.addEventListener("mousedown", handleClickOutside)
    return () => document.removeEventListener("mousedown", handleClickOutside)
  }, [])

  if (!isAuthenticated) return <Navigate to={paths.login} replace />

  if (user?.role !== "admin") {
    return (
      <div className="flex min-h-dvh items-center justify-center bg-(--bg)">
        <div className="rounded-sm border border-(--destructive)/30 bg-(--destructive)/5 px-8 py-6 text-center">
          <p className="font-mono text-sm font-medium text-(--destructive)">access denied</p>
          <p className="mt-1 font-mono text-xs text-(--muted)">This panel requires an admin account</p>
        </div>
      </div>
    )
  }

  const handleLogout = () => {
    logout()
    navigate(paths.login, { replace: true })
  }

  return (
    <div className="flex min-h-dvh flex-col bg-(--bg)">
      {/* Top navigation */}
      <header className="sticky top-0 z-30 border-b border-(--border) bg-(--surface)">
        {/* Accent gradient line at top */}
        <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-(--accent)/50 to-transparent" />

        <div className="flex h-12 items-center gap-4 px-4 sm:gap-6 sm:px-6">
          {/* Brand */}
          <div className="flex items-center gap-2 shrink-0">
            <span className="font-semibold text-(--accent)">◈</span>
            <span className="font-mono text-sm font-medium text-(--fg)">nexus</span>
            <span className="rounded-sm bg-(--accent)/15 px-1.5 py-0.5 font-mono text-[10px] font-medium text-(--accent) border border-(--accent)/20">
              admin
            </span>
          </div>

          {/* Desktop nav */}
          <div className="hidden h-4 w-px bg-(--border) md:block" />
          <nav className="hidden items-center gap-0.5 md:flex">
            {NAV_ITEMS.map(({ to, icon: Icon, label }) => (
              <NavLink
                key={to}
                to={to}
                className={({ isActive }) =>
                  cn(
                    "relative flex items-center gap-1.5 rounded-sm px-3 py-1.5",
                    "font-mono text-xs transition-colors",
                    isActive
                      ? "text-(--fg)"
                      : "text-(--muted) hover:bg-(--surface-2) hover:text-(--fg)"
                  )
                }
              >
                {({ isActive }) => (
                  <>
                    <Icon className="h-3.5 w-3.5 shrink-0" />
                    {label}
                    {isActive && (
                      <motion.span
                        layoutId="nav-indicator"
                        className="absolute inset-x-1 -bottom-[calc(theme(spacing.6)+1px)] h-px bg-(--accent)"
                        transition={{ duration: 0.2 }}
                      />
                    )}
                  </>
                )}
              </NavLink>
            ))}
          </nav>

          {/* Right side */}
          <div className="ml-auto flex items-center gap-2 sm:gap-3">
            <ThemeSwitcher />

            {/* User menu */}
            <div className="relative" ref={menuRef}>
              <button
                onClick={() => setUserMenuOpen((o) => !o)}
                className={cn(
                  "flex items-center gap-1.5 rounded-sm px-2.5 py-1.5",
                  "font-mono text-xs text-(--muted) transition-colors",
                  "border border-(--border) hover:border-(--muted)/60 hover:text-(--fg)",
                  userMenuOpen && "border-(--accent)/40 text-(--fg) bg-(--accent)/5"
                )}
              >
                <UserCircleIcon className="h-3.5 w-3.5" />
                <span className="hidden sm:inline">{user?.role}</span>
                <CaretDownIcon
                  className={cn(
                    "h-2.5 w-2.5 transition-transform duration-200",
                    userMenuOpen && "rotate-180"
                  )}
                />
              </button>

              <AnimatePresence>
                {userMenuOpen && (
                  <motion.div
                    initial={{ opacity: 0, y: -4, scale: 0.97 }}
                    animate={{ opacity: 1, y: 0, scale: 1 }}
                    exit={{ opacity: 0, y: -4, scale: 0.97 }}
                    transition={{ duration: 0.12 }}
                    className="absolute right-0 top-full mt-1.5 w-48 overflow-hidden rounded-sm border border-(--border) bg-(--surface) shadow-xl shadow-black/20"
                  >
                    <div className="border-b border-(--border) px-3 py-2.5">
                      <p className="font-mono text-[10px] uppercase tracking-wider text-(--muted)">
                        Signed in as
                      </p>
                      <p className="mt-0.5 font-mono text-xs font-medium text-(--fg)">
                        {user?.role}
                      </p>
                    </div>
                    <button
                      onClick={handleLogout}
                      className={cn(
                        "flex w-full items-center gap-2 px-3 py-2.5",
                        "font-mono text-xs text-(--muted) transition-colors",
                        "hover:bg-(--destructive)/10 hover:text-(--destructive)"
                      )}
                    >
                      <SignOutIcon className="h-3.5 w-3.5" />
                      Sign out
                    </button>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>

            {/* Mobile hamburger */}
            <button
              onClick={() => setMobileNavOpen((o) => !o)}
              className={cn(
                "flex items-center justify-center rounded-sm p-1.5 md:hidden",
                "text-(--muted) transition-colors hover:bg-(--surface-2) hover:text-(--fg)"
              )}
              aria-label="Toggle navigation"
            >
              {mobileNavOpen ? (
                <XIcon className="h-4 w-4" />
              ) : (
                <ListIcon className="h-4 w-4" />
              )}
            </button>
          </div>
        </div>

        {/* Mobile nav */}
        <AnimatePresence>
          {mobileNavOpen && (
            <motion.nav
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: "auto" }}
              exit={{ opacity: 0, height: 0 }}
              transition={{ duration: 0.18 }}
              className="overflow-hidden border-t border-(--border) md:hidden"
            >
              <div className="px-4 py-2">
                {NAV_ITEMS.map(({ to, icon: Icon, label }) => (
                  <NavLink
                    key={to}
                    to={to}
                    onClick={() => setMobileNavOpen(false)}
                    className={({ isActive }) =>
                      cn(
                        "flex items-center gap-2.5 rounded-sm px-3 py-2.5",
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
              </div>
            </motion.nav>
          )}
        </AnimatePresence>
      </header>

      {/* Page content */}
      <main className="flex-1 overflow-auto">
        <div className="mx-auto max-w-4xl px-4 py-6 sm:px-6 sm:py-8">
          <Outlet />
        </div>
      </main>
    </div>
  )
}
