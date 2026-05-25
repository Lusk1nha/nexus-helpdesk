import { API } from "@nexus/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export function useDeleteKnowledge() {
  const qc = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      fetchApi<{ message: string }>(() =>
        http.delete(API.knowledge.delete(id)).json()
      ),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["knowledge"] })
    },
  })
}
