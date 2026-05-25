import { useRoutes } from "react-router"

import { compose, routes } from "@/presentation/router"

/**
 * Root component. Routes are declared as data in
 * `src/presentation/router/routes.tsx` and composed from per-feature files
 * under `src/presentation/pages/<area>/routes.tsx`.
 */
export function App() {
  return useRoutes(compose(routes))
}
