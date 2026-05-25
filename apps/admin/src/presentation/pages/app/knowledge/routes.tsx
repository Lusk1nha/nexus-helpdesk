import { segments } from "@/presentation/router/paths"
import type { AppRoute } from "@/presentation/router/types"

import { KnowledgePage } from "./knowledge.page"

export const knowledgeRoutes: AppRoute[] = [
  { path: segments.knowledge, element: <KnowledgePage /> },
]
