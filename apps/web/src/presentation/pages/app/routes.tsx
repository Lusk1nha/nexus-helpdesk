import { Navigate } from "react-router"

import type { AppRoute } from "@/presentation/router/types"

import { DashboardPage } from "./dashboard.page"

/**
 * Routes mounted under <AppLayout /> (require authentication).
 *
 * To add a new app page:
 *   1. Create `<name>.page.tsx` in this folder (or a subfolder)
 *   2. Add an entry below — path is relative to "/app"
 *   3. Optional: set `requiredRole` to restrict to "admin" or "agent"
 *
 * Example:
 *   { path: "admin", element: <AdminPage />, requiredRole: "admin" }
 *   { path: "knowledge", element: <KnowledgePage />, requiredRole: ["admin", "agent"] }
 */
export const appRoutes: AppRoute[] = [
  { index: true, element: <Navigate to="/app/tickets" replace /> },
  { path: "tickets", element: <DashboardPage /> },
]
