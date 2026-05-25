import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import { BrowserRouter } from "react-router"

import { ThemeProvider } from "@nexus/theme"
import { QueryProvider } from "@nexus/query"

import { App } from "./App"

import "./index.css"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <ThemeProvider>
        <QueryProvider>
          <App />
        </QueryProvider>
      </ThemeProvider>
    </BrowserRouter>
  </StrictMode>
)
