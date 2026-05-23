import type { AppRoute } from "@/presentation/router/types"

import { LoginPage } from "./login.page"
import { RegisterPage } from "./register.page"

/**
 * Routes mounted under <AuthLayout /> (public, redirect to /app if logged in).
 *
 * To add a new auth page:
 *   1. Create `<name>.page.tsx` in this folder
 *   2. Add an entry below — path is relative to the parent ("/")
 */
export const authRoutes: AppRoute[] = [
  { path: "login", element: <LoginPage /> },
  { path: "register", element: <RegisterPage /> },
]
