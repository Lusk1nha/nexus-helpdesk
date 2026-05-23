import { useRoutes } from "react-router"

import { compose, routes } from "@/presentation/router"

/**
 * Root component. Routes are declared as data in
 * `src/presentation/router/routes.tsx` (per-feature files), not as JSX here.
 *
 * To add a new route, edit the routes file of the matching feature folder.
 */
export function App() {
  return useRoutes(compose(routes))
}
