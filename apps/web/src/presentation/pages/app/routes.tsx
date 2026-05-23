import { Navigate } from "react-router"

import { paths, segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { DashboardPage } from "./dashboard.page"

/**
 * Routes mounted under <AppLayout /> (require authentication).
 *
 * To add a new app page:
 *   1. Create `<name>.page.tsx` in this folder (or a subfolder)
 *   2. Register the segment + absolute path in `router/paths.ts`
 *   3. Add an entry below using `segments.<name>`
 *   4. Optional: set `requiredRole` to restrict to "admin" or "agent"
 *
 * Example:
 *   { path: segments.admin, element: <AdminPage />, requiredRole: "admin" }
 *   { path: segments.knowledge, element: <KnowledgePage />, requiredRole: ["admin", "agent"] }
 */
export const appRoutes: AppRoute[] = [
  { index: true, element: <Navigate to={paths.app.tickets} replace /> },
  { path: segments.tickets, element: <DashboardPage /> },
]
