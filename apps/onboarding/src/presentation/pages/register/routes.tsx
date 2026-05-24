import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { RegisterPage } from "./register.page"

export const registerRoutes: AppRoute[] = [
  { path: segments.register, element: <RegisterPage /> },
]
