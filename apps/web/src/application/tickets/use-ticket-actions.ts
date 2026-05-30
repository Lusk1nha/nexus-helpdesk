import { API } from "@nexus/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

import type { Ticket } from "@/domain/tickets/ticket"
import { fetchApi, http } from "@/infrastructure/http/client"

function invalidateTicket(qc: ReturnType<typeof useQueryClient>, id: string) {
  qc.invalidateQueries({ queryKey: ["ticket", id] })
  qc.invalidateQueries({ queryKey: ["tickets"] })
}

export function useApproveAi(ticketId: string) {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: () =>
      fetchApi<Ticket>(() => http.post(API.tickets.approveAi(ticketId)).json()),
    onSuccess: () => invalidateTicket(qc, ticketId),
  })
}

export function useRejectAi(ticketId: string) {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: () =>
      fetchApi<Ticket>(() => http.post(API.tickets.rejectAi(ticketId)).json()),
    onSuccess: () => invalidateTicket(qc, ticketId),
  })
}

export function useAssignTicket(ticketId: string) {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: () =>
      fetchApi<Ticket>(() => http.post(API.tickets.assign(ticketId)).json()),
    onSuccess: () => invalidateTicket(qc, ticketId),
  })
}

export function useUpdateTicketStatus(ticketId: string) {
  const qc = useQueryClient()
  return useMutation({
    mutationFn: (status: string) =>
      fetchApi<Ticket>(() =>
        http
          .patch(API.tickets.updateStatus(ticketId), { json: { status } })
          .json()
      ),
    onSuccess: () => invalidateTicket(qc, ticketId),
  })
}
