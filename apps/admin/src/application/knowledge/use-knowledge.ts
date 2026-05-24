import { API } from "@nexus/api"
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export interface KnowledgeArticle {
  id: string
  title: string
  content: string
  status: "pending" | "approved" | "rejected"
  createdAt: string
  updatedAt: string
}

export function useKnowledge() {
  return useQuery({
    queryKey: ["knowledge"],
    queryFn: () =>
      fetchApi<KnowledgeArticle[]>(() => http.get(API.knowledge.list).json()),
  })
}

export interface CreateKnowledgeInput {
  title: string
  content: string
}

export function useCreateKnowledge() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (input: CreateKnowledgeInput) =>
      fetchApi<KnowledgeArticle>(() =>
        http.post(API.knowledge.create, { json: input }).json()
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["knowledge"] })
    },
  })
}

export function useDeleteKnowledge() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) =>
      fetchApi<void>(() => http.delete(API.knowledge.delete(id)).json()),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["knowledge"] })
    },
  })
}
