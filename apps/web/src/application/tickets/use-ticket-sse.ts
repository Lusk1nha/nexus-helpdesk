import { useEffect } from "react"
import { useQueryClient } from "@tanstack/react-query"

import { useAuthStore } from "@nexus/auth"
import { env } from "@/env"
import { API } from "@nexus/api"

export function useTicketSse(ticketId: string) {
  const qc = useQueryClient()
  const accessToken = useAuthStore((s) => s.accessToken)

  useEffect(() => {
    if (!ticketId || !accessToken) return

    const url = `${env.apiUrl}/${API.realtime.ticket(ticketId)}?token=${accessToken}`
    const es = new EventSource(url)

    es.addEventListener("message_added", () => {
      qc.invalidateQueries({ queryKey: ["ticket-messages", ticketId] })
    })

    es.addEventListener("status_changed", () => {
      qc.invalidateQueries({ queryKey: ["ticket", ticketId] })
      qc.invalidateQueries({ queryKey: ["tickets"] })
    })

    return () => es.close()
  }, [ticketId, accessToken, qc])
}

export function useSystemSse() {
  const qc = useQueryClient()
  const accessToken = useAuthStore((s) => s.accessToken)

  useEffect(() => {
    if (!accessToken) return

    const url = `${env.apiUrl}/${API.realtime.system}?token=${accessToken}`
    const es = new EventSource(url)

    es.addEventListener("ticket_created", () => {
      qc.invalidateQueries({ queryKey: ["tickets"] })
    })

    es.addEventListener("ticket_status_changed", () => {
      qc.invalidateQueries({ queryKey: ["tickets"] })
    })

    return () => es.close()
  }, [accessToken, qc])
}
