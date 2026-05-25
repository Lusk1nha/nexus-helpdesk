import type { RouteObject } from "react-router"

import type { AppRoute } from "./types"

/**
 * Transforms our `AppRoute[]` config into the `RouteObject[]` format that
 * React Router's `useRoutes()` consumes. Future guards (auth, role) would
 * be wired here so individual route files stay declarative.
 */
export function compose(routes: AppRoute[]): RouteObject[] {
  return routes.map((route): RouteObject => {
    const { children, ...rest } = route
    return {
      ...rest,
      ...(children && { children: compose(children) }),
    } as RouteObject
  })
}
