import { API } from "@nexus/api"
import type { Role } from "@nexus/auth"
import { useQuery } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export interface TenantMember {
  userId: string
  email: string
  fullName: string
  role: Role
  isActive: boolean
  joinedAt: string
}

/** Lists the members of the current tenant. Admin-only on the backend. */
export function useTeam() {
  return useQuery({
    queryKey: ["team"],
    queryFn: () =>
      fetchApi<TenantMember[]>(() => http.get(API.identity.users).json()),
    staleTime: 60_000,
  })
}
