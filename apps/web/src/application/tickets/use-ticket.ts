import { API } from "@nexus/api"
import { useQuery } from "@tanstack/react-query"

import type { Ticket } from "@/domain/tickets/ticket"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useTicket(id: string) {
  return useQuery({
    queryKey: ["ticket", id],
    queryFn: () => fetchApi<Ticket>(() => http.get(API.tickets.get(id)).json()),
    enabled: !!id,
    staleTime: 10_000,
  })
}
