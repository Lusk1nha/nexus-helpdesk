import { API } from "@nexus/api"
import {
  useSuspenseQuery,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export interface KnowledgeArticle {
  id: string
  title: string
  content: string
  status: "pending" | "approved" | "rejected"
  createdAt: string
  updatedAt: string
}

export interface KnowledgeResponse {
  data: KnowledgeArticle[]
  count: number
}

export function useKnowledge() {
  return useSuspenseQuery({
    queryKey: ["knowledge"],
    queryFn: () =>
      fetchApi<KnowledgeResponse>(() => http.get(API.knowledge.list).json()),
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
