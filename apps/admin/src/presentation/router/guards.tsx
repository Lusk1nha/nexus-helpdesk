import { Navigate } from "react-router"

import type { Role } from "@nexus/auth"

import { useSession } from "@/application/auth/use-session"
import { paths } from "./paths"

interface RequireRoleProps {
  role: Role | Role[]
  children: React.ReactNode
}

export function RequireRole({ role, children }: RequireRoleProps) {
  const user = useSession()
  const allowed = Array.isArray(role) ? role : [role]

  if (!user || !allowed.includes(user.role)) {
    return <Navigate to={paths.login} replace />
  }
  return <>{children}</>
}
