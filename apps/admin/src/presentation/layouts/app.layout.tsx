import {
  BookOpenIcon,
  BuildingsIcon,
  CaretDownIcon,
  SignOutIcon,
  UserCircleIcon,
} from "@phosphor-icons/react"
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
]

export function AppLayout() {
  const isAuthenticated = useIsAuthenticated()
  const user = useSession()
  const logout = useLogout()
  const navigate = useNavigate()
  const [userMenuOpen, setUserMenuOpen] = useState(false)
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

  if (!isAuthenticated) {
    return <Navigate to={paths.login} replace />
  }

  if (user?.role !== "admin") {
    return (
      <div className="flex min-h-dvh items-center justify-center bg-(--bg)">
        <div className="rounded-sm border border-(--destructive)/30 bg-(--destructive)/5 px-8 py-6 text-center">
          <p className="font-mono text-sm font-medium text-(--destructive)">
            Access denied
          </p>
          <p className="mt-1 font-mono text-xs text-(--muted)">
            This panel requires an admin account
          </p>
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
      <header className="sticky top-0 z-30 flex h-12 items-center gap-6 border-b border-(--border) bg-(--surface) px-6">
        {/* Brand */}
        <div className="flex items-center gap-2 shrink-0">
          <span className="font-semibold text-(--accent)">◈</span>
          <span className="font-mono text-sm font-medium text-(--fg)">nexus</span>
          <span className="rounded-sm bg-(--accent)/15 px-1.5 py-0.5 font-mono text-[10px] font-medium text-(--accent)">
            admin
          </span>
        </div>

        {/* Divider */}
        <div className="h-4 w-px bg-(--border)" />

        {/* Nav links */}
        <nav className="flex items-center gap-1">
          {NAV_ITEMS.map(({ to, icon: Icon, label }) => (
            <NavLink
              key={to}
              to={to}
              className={({ isActive }) =>
                cn(
                  "flex items-center gap-1.5 rounded-sm px-3 py-1.5",
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

        {/* Right side */}
        <div className="ml-auto flex items-center gap-3">
          <ThemeSwitcher />

          {/* User menu */}
          <div className="relative" ref={menuRef}>
            <button
              onClick={() => setUserMenuOpen((o) => !o)}
              className={cn(
                "flex items-center gap-1.5 rounded-sm px-2.5 py-1.5",
                "font-mono text-xs text-(--muted) transition-colors",
                "border border-(--border) hover:border-(--muted) hover:text-(--fg)",
                userMenuOpen && "border-(--muted) text-(--fg)"
              )}
            >
              <UserCircleIcon className="h-3.5 w-3.5" />
              <span className="hidden sm:inline">{user?.role}</span>
              <CaretDownIcon
                className={cn(
                  "h-2.5 w-2.5 transition-transform",
                  userMenuOpen && "rotate-180"
                )}
              />
            </button>

            {userMenuOpen && (
              <div className="absolute right-0 top-full mt-1 w-44 overflow-hidden rounded-sm border border-(--border) bg-(--surface) shadow-lg shadow-black/20">
                <div className="border-b border-(--border) px-3 py-2">
                  <p className="font-mono text-[11px] text-(--muted)">
                    Signed in as
                  </p>
                  <p className="font-mono text-xs font-medium text-(--fg)">
                    {user?.role}
                  </p>
                </div>
                <button
                  onClick={handleLogout}
                  className={cn(
                    "flex w-full items-center gap-2 px-3 py-2",
                    "font-mono text-xs text-(--muted) transition-colors",
                    "hover:bg-(--destructive)/10 hover:text-(--destructive)"
                  )}
                >
                  <SignOutIcon className="h-3.5 w-3.5" />
                  Sign out
                </button>
              </div>
            )}
          </div>
        </div>
      </header>

      {/* Page content */}
      <main className="flex-1 overflow-auto">
        <div className="mx-auto max-w-4xl px-6 py-8">
          <Outlet />
        </div>
      </main>
    </div>
  )
}
