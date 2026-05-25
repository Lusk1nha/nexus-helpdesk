import { OnboardingLayout } from "@/presentation/layouts/onboarding.layout"
import { aboutRoutes } from "@/presentation/pages/about/routes"
import { landingRoutes } from "@/presentation/pages/landing/routes"
import { NotFoundPage } from "@/presentation/pages/not-found/not-found.page"
import { registerRoutes } from "@/presentation/pages/register/routes"

import type { AppRoute } from "./types"

/**
 * Top-level route map for the onboarding app.
 * All routes share <OnboardingLayout />.
 *
 * To add a new section:
 *   1. Create the page under `presentation/pages/<section>/`
 *   2. Create `pages/<section>/routes.tsx`
 *   3. Register the segment + absolute path in `router/paths.ts`
 *   4. Add the routes to the `children` array below
 */
export const routes: AppRoute[] = [
  {
    element: <OnboardingLayout />,
    children: [
      ...landingRoutes,
      ...registerRoutes,
      ...aboutRoutes,
      // Catch-all (must stay last)
      { path: "*", element: <NotFoundPage /> },
    ],
  },
]
