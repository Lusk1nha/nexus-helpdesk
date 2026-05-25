import type { AppRoute } from "@/presentation/router/types"

import { LandingPage } from "./landing.page"

/**
 * Routes for the landing area (`/`). Sibling pages like /pricing, /terms,
 * /privacy would land here too. Register their segments in
 * `router/paths.ts` and add an entry below.
 */
export const landingRoutes: AppRoute[] = [
  { index: true, element: <LandingPage /> },
]
