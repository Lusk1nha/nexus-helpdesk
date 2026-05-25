import { Navigate } from "react-router"

import { paths, segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"
import { RequireRole } from "@/presentation/router/guards"

import { TicketsPage } from "./tickets/tickets.page"
import { TicketDetailPage } from "./tickets/ticket-detail.page"
import { KnowledgePage } from "./knowledge/knowledge.page"

export const appRoutes: AppRoute[] = [
  { index: true, element: <Navigate to={paths.app.tickets} replace /> },
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
]
