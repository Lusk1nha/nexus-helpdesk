import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { TenantPage } from "./tenant.page"

export const tenantRoutes: AppRoute[] = [
  { path: segments.tenant, element: <TenantPage /> },
]
