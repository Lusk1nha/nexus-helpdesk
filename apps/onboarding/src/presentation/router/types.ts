import type { RouteObject } from "react-router"

/**
 * Onboarding routes are all public — no role-based guarding needed.
 * Kept as a thin alias of RouteObject so the data-driven pattern stays
 * consistent with apps/web (where it carries `requiredRole`).
 */
export interface AppRoute extends Omit<RouteObject, "children"> {
  children?: AppRoute[]
}
