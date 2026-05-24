import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { UsersPage } from "./users.page"

export const usersRoutes: AppRoute[] = [
  { path: segments.users, element: <UsersPage /> },
]
