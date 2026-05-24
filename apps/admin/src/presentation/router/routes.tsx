import { Navigate } from "react-router"

import { AppLayout } from "@/presentation/layouts/app.layout"
import { AuthLayout } from "@/presentation/layouts/auth.layout"
import { appRoutes } from "@/presentation/pages/app/routes"
import { authRoutes } from "@/presentation/pages/auth/routes"

import { paths, segments } from "./paths"
import type { AppRoute } from "./types"

export const routes: AppRoute[] = [
  { path: paths.home, element: <Navigate to={paths.login} replace /> },
  { element: <AuthLayout />, children: authRoutes },
  { path: segments.app, element: <AppLayout />, children: appRoutes },
  { path: "*", element: <Navigate to={paths.login} replace /> },
]
