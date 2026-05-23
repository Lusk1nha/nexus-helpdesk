import { Navigate } from "react-router"

import { useSession } from "@/application/auth/use-session"
import type { Role } from "@nexus/auth"

import { paths } from "./paths"

interface RequireRoleProps {
  role: Role | Role[]
  children: React.ReactNode
}

/**
 * Wraps a route element to restrict access to specific roles.
 * Customers hitting an admin-only route get redirected to /app/tickets.
 *
 * Authentication itself (logged in vs not) is handled at the layout level
 * by AppLayout/AuthLayout — this guard only handles role-based access.
 */
export function RequireRole({ role, children }: RequireRoleProps) {
  const user = useSession()
  const allowed = Array.isArray(role) ? role : [role]

  if (!user || !allowed.includes(user.role)) {
    return <Navigate to={paths.app.tickets} replace />
  }
  return <>{children}</>
}
