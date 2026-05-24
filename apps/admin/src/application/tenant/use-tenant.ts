import { API } from "@nexus/api"
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export interface Tenant {
  id: string
  name: string
  description?: string
  slug: string
  theme: string
  createdAt: string
}

export function useTenant() {
  return useQuery({
    queryKey: ["tenant"],
    queryFn: () =>
      fetchApi<Tenant>(() => http.get(API.identity.tenant).json()),
  })
}

export interface UpdateTenantInput {
  name?: string
  description?: string
  theme?: string
}

export function useUpdateTenant() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (input: UpdateTenantInput) =>
      fetchApi<Tenant>(() =>
        http.patch(API.identity.tenant, { json: input }).json()
      ),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["tenant"] })
    },
  })
}
