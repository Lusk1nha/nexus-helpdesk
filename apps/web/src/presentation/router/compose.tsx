import type { RouteObject } from "react-router"

import { RequireRole } from "./guards"
import type { AppRoute } from "./types"

/**
 * Transforms our `AppRoute[]` config into the `RouteObject[]` format
 * React Router's `useRoutes()` expects. Wraps any route with a
 * `requiredRole` field in a `<RequireRole>` guard.
 */
export function compose(routes: AppRoute[]): RouteObject[] {
  return routes.map((route): RouteObject => {
    const { requiredRole, element, children, ...rest } = route

    const guardedElement =
      requiredRole && element ? (
        <RequireRole role={requiredRole}>{element}</RequireRole>
      ) : (
        element
      )

    return {
      ...rest,
      element: guardedElement,
      ...(children && { children: compose(children) }),
    } as RouteObject
  })
}
