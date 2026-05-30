import { Navigate } from "react-router"

import { paths, segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"
import { RequireRole } from "@/presentation/router/guards"

import { DashboardPage } from "./dashboard.page"
import { TicketsPage } from "./tickets/tickets.page"
import { TicketDetailPage } from "./tickets/ticket-detail.page"
import { KnowledgePage } from "./knowledge/knowledge.page"
import { AdminPage } from "./admin/admin.page"

export const appRoutes: AppRoute[] = [
  { index: true, element: <Navigate to={paths.app.dashboard} replace /> },
  { path: segments.dashboard, element: <DashboardPage /> },
  { path: segments.tickets, element: <TicketsPage /> },
  {
    path: `${segments.tickets}/:id`,
    element: <TicketDetailPage />,
  },
  {
    path: segments.knowledge,
    element: (
      <RequireRole role={["admin", "agent"]}>
        <KnowledgePage />
      </RequireRole>
    ),
  },
  {
    path: segments.admin,
    element: (
      <RequireRole role={["admin"]}>
        <AdminPage />
      </RequireRole>
    ),
  },
]
