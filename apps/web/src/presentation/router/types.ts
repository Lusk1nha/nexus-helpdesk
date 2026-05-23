import type { RouteObject } from "react-router"

import type { Role } from "@nexus/auth"

/**
 * Extension of React Router's RouteObject with an optional `requiredRole`
 * field. When set, the route element is wrapped in a `<RequireRole>` guard
 * by the compose() helper.
 *
 * To add a new route:
 *   1. Create the page component under `src/presentation/pages/<area>/`
 *   2. Add an entry to the matching `routes.tsx` in that folder
 *   3. Done. Composition is automatic.
 */
export interface AppRoute extends Omit<RouteObject, "children"> {
  /** Optional role restriction. Single role or array of allowed roles. */
  requiredRole?: Role | Role[]
  children?: AppRoute[]
}
