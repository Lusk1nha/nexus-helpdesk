import { Navigate } from "react-router"

import { AppLayout } from "@/presentation/layouts/app.layout"
import { AuthLayout } from "@/presentation/layouts/auth.layout"
import { appRoutes } from "@/presentation/pages/app/routes"
import { authRoutes } from "@/presentation/pages/auth/routes"

import { paths, segments } from "./paths"
import type { AppRoute } from "./types"

/**
 * Top-level route map. Each layout (auth, app, etc.) declares its children
 * here, but the actual routes inside each section live next to their pages
 * (see `pages/<section>/routes.tsx`).
 *
 * To add a new section:
 *   1. Create the layout in `presentation/layouts/`
 *   2. Create routes in `presentation/pages/<section>/routes.tsx`
 *   3. Add an entry below mounting the layout + children
 *   4. Add the section's segment + paths to `router/paths.ts`
 */
export const routes: AppRoute[] = [
  // Root → login
  { path: paths.home, element: <Navigate to={paths.login} replace /> },

  // Public area (login, register, password recovery, etc.)
  { element: <AuthLayout />, children: authRoutes },

  // Authenticated app (tickets, knowledge, admin, settings, etc.)
  { path: segments.app, element: <AppLayout />, children: appRoutes },

  // Catch-all → login
  { path: "*", element: <Navigate to={paths.login} replace /> },
]
