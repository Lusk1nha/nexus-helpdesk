import { API } from "@nexus/api"
import { useQuery } from "@tanstack/react-query"

import type { TicketMessage } from "@/domain/tickets/ticket"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useTicketMessages(ticketId: string) {
  return useQuery({
    queryKey: ["ticket-messages", ticketId],
    queryFn: () =>
      fetchApi<TicketMessage[]>(() =>
        http.get(API.tickets.messages(ticketId)).json()
      ),
    enabled: !!ticketId,
    staleTime: 5_000,
  })
}
