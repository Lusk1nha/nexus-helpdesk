import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { LoginPage } from "./login.page"
import { RegisterPage } from "./register.page"

/**
 * Routes mounted under <AuthLayout /> (public, redirect to /app if logged in).
 *
 * To add a new auth page:
 *   1. Create `<name>.page.tsx` in this folder
 *   2. Register the segment + absolute path in `router/paths.ts`
 *   3. Add an entry below using `segments.<name>`
 */
export const authRoutes: AppRoute[] = [
  { path: segments.login, element: <LoginPage /> },
  { path: segments.register, element: <RegisterPage /> },
]
