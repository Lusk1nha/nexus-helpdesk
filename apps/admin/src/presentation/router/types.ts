import type { RouteObject } from "react-router"

import type { Role } from "@nexus/auth"

export interface AppRoute extends Omit<RouteObject, "children"> {
  requiredRole?: Role | Role[]
  children?: AppRoute[]
}
