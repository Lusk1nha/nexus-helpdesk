import { API } from "@nexus/api"
import { useQuery } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export interface TenantUser {
  id: string
  email: string
  role: "admin" | "agent" | "customer"
  createdAt: string
}

export function useUsers() {
  return useQuery({
    queryKey: ["users"],
    queryFn: () =>
      fetchApi<TenantUser[]>(() => http.get(API.identity.users).json()),
  })
}
