import { API } from "@nexus/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

import type { CreateKnowledgeInput } from "@/domain/knowledge/knowledge"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useCreateKnowledge() {
  const qc = useQueryClient()

  return useMutation({
    mutationFn: (input: CreateKnowledgeInput) =>
      fetchApi<{ documentId: string; message: string }>(() =>
        http.post(API.knowledge.create, { json: input }).json()
      ),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["knowledge"] })
    },
  })
}
