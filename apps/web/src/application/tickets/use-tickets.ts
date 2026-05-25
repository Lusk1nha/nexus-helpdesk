import { API } from "@nexus/api"
import { useQuery } from "@tanstack/react-query"

import type { Ticket, TicketStatus } from "@/domain/tickets/ticket"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useTickets(status?: TicketStatus) {
  return useQuery({
    queryKey: ["tickets", status ?? "all"],
    queryFn: () => {
      const url = status
        ? `${API.tickets.list}?status=${status}`
        : API.tickets.list
      return fetchApi<Ticket[]>(() => http.get(url).json())
    },
    staleTime: 30_000,
  })
}
