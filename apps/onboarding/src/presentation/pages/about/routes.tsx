import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { AboutPage } from "./about.page"

export const aboutRoutes: AppRoute[] = [
  { path: segments.about, element: <AboutPage /> },
]
