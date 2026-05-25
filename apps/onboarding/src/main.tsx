import { ThemeProvider } from "@nexus/theme"
import { QueryProvider } from "@nexus/query"
import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import { BrowserRouter } from "react-router"

import { App } from "./app"
import { ErrorBoundary } from "./presentation/components/error-boundary"

import "./index.css"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ErrorBoundary>
      <BrowserRouter>
        <ThemeProvider>
          <QueryProvider>
            <App />
          </QueryProvider>
        </ThemeProvider>
      </BrowserRouter>
    </ErrorBoundary>
  </StrictMode>
)
