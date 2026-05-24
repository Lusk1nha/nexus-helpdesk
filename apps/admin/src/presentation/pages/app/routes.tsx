import { Navigate } from "react-router"

import { paths } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { knowledgeRoutes } from "./knowledge/routes"
import { tenantRoutes } from "./tenant/routes"
import { usersRoutes } from "./users/routes"

export const appRoutes: AppRoute[] = [
  { index: true, element: <Navigate to={paths.app.tenant} replace /> },
  ...tenantRoutes,
  ...knowledgeRoutes,
  ...usersRoutes,
]
