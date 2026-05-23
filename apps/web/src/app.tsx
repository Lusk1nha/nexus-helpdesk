import { Navigate, Route, Routes } from "react-router"

import { AppLayout } from "@/presentation/layouts/app.layout"
import { AuthLayout } from "@/presentation/layouts/auth.layout"
import { LoginPage } from "@/presentation/pages/auth/login.page"
import { RegisterPage } from "@/presentation/pages/auth/register.page"
import { DashboardPage } from "@/presentation/pages/app/dashboard.page"

export function App() {
  return (
    <Routes>
      {/* Redirect root */}
      <Route path="/" element={<Navigate to="/login" replace />} />

      {/* Public routes (unauthenticated) */}
      <Route element={<AuthLayout />}>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/register" element={<RegisterPage />} />
      </Route>

      {/* Protected routes (authenticated) */}
      <Route path="/app" element={<AppLayout />}>
        <Route index element={<Navigate to="/app/tickets" replace />} />
        <Route path="tickets" element={<DashboardPage />} />
        {/* More routes will be added here */}
      </Route>

      {/* Fallback */}
      <Route path="*" element={<Navigate to="/login" replace />} />
    </Routes>
  )
}
