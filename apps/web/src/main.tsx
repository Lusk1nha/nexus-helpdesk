import { ThemeProvider, useTheme, type ThemeId } from "@nexus/theme"
import { QueryProvider } from "@nexus/query"
import { StrictMode, useEffect } from "react"
import { createRoot } from "react-dom/client"
import { BrowserRouter } from "react-router"

import { App } from "./app"
import { useTenantSlug } from "./application/tenant/use-tenant-slug"
import { useTenantBranding } from "./application/tenant/use-tenant-branding"
import { ToastProvider } from "./presentation/components/toast/toast"

import "./index.css"

function TenantThemeLoader({ children }: { children: React.ReactNode }) {
  const slug = useTenantSlug()
  const { data: branding } = useTenantBranding(slug)
  const { applyDefault } = useTheme()

  useEffect(() => {
    if (branding) applyDefault(branding.theme as ThemeId)
  }, [branding, applyDefault])

  return <>{children}</>
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <ThemeProvider>
        <QueryProvider>
          <ToastProvider>
            <TenantThemeLoader>
              <App />
            </TenantThemeLoader>
          </ToastProvider>
        </QueryProvider>
      </ThemeProvider>
    </BrowserRouter>
  </StrictMode>
)
