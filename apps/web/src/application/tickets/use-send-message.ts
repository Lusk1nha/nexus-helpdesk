import { API } from "@nexus/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

import type { TicketMessage } from "@/domain/tickets/ticket"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useSendMessage(ticketId: string) {
  const qc = useQueryClient()

  return useMutation({
    mutationFn: (content: string) =>
      fetchApi<TicketMessage>(() =>
        http.post(API.tickets.messages(ticketId), { json: { content } }).json()
      ),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["ticket-messages", ticketId] })
    },
  })
}
