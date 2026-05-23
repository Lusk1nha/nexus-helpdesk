import { Route, Routes } from "react-router"

import { OnboardingLayout } from "@/presentation/layouts/onboarding.layout"
import { LandingPage } from "@/presentation/pages/landing.page"
import { RegisterPage } from "@/presentation/pages/register.page"

export function App() {
  return (
    <Routes>
      <Route element={<OnboardingLayout />}>
        {/* A Landing page é a home do Onboarding */}
        <Route path="/" element={<LandingPage />} />
        
        {/* O formulário de criação de conta fica em /register */}
        <Route path="/register" element={<RegisterPage />} />
      </Route>
    </Routes>
  )
}