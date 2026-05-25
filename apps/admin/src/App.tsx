import { useRoutes } from "react-router"

import { compose, routes } from "@/presentation/router"

export function App() {
  return useRoutes(compose(routes))
}
