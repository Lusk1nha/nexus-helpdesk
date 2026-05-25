import { API } from "@nexus/api"
import { useQuery } from "@tanstack/react-query"

import type { KnowledgeDocument } from "@/domain/knowledge/knowledge"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useKnowledge() {
  return useQuery({
    queryKey: ["knowledge"],
    queryFn: () =>
      fetchApi<{ items: KnowledgeDocument[]; count: number }>(() =>
        http.get(API.knowledge.list).json()
      ),
    staleTime: 60_000,
  })
}
