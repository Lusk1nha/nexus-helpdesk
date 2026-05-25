import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { LoginPage } from "./login.page"

export const authRoutes: AppRoute[] = [
  { path: segments.login, element: <LoginPage /> },
]
