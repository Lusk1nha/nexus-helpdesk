import { API } from "@nexus/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

import type { CreateTicketInput } from "@/domain/tickets/ticket"
import { fetchApi, http } from "@/infrastructure/http/client"

export function useCreateTicket() {
  const qc = useQueryClient()

  return useMutation({
    mutationFn: (input: CreateTicketInput) =>
      fetchApi<{ ticketId: string; status: string; message: string }>(() =>
        http.post(API.tickets.create, { json: input }).json()
      ),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["tickets"] })
    },
  })
}
