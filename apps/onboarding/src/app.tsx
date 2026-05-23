import { Route, Routes } from "react-router"

import { OnboardingLayout } from "@/presentation/layouts/onboarding.layout"
import { LandingPage } from "@/presentation/pages/landing.page"
import { RegisterPage } from "@/presentation/pages/register.page"
import { AboutPage } from "@/presentation/pages/about.page" // <--- Adicione a importação

export function App() {
  return (
    <Routes>
      <Route element={<OnboardingLayout />}>
        <Route path="/" element={<LandingPage />} />
        <Route path="/register" element={<RegisterPage />} />
        {/* Nova Rota adicionada */}
        <Route path="/about" element={<AboutPage />} /> 
      </Route>
    </Routes>
  )
}