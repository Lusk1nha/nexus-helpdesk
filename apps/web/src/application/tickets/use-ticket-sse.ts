import { useEffect } from "react"
import { useQueryClient } from "@tanstack/react-query"

import { useAuthStore } from "@nexus/auth"
import { env } from "@/env"
import { API } from "@nexus/api"
import { useToast } from "@/presentation/components/toast/toast-context"

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

    es.addEventListener("assignee_changed", () => {
      qc.invalidateQueries({ queryKey: ["ticket", ticketId] })
      qc.invalidateQueries({ queryKey: ["tickets"] })
    })

    return () => es.close()
  }, [ticketId, accessToken, qc])
}

export function useSystemSse() {
  const qc = useQueryClient()
  const accessToken = useAuthStore((s) => s.accessToken)
  const role = useAuthStore((s) => s.user?.role)
  const { toast } = useToast()

  useEffect(() => {
    if (!accessToken) return

    // Toasts for system events are only meaningful to support staff.
    const isStaff = role === "agent" || role === "admin"

    const url = `${env.apiUrl}/${API.realtime.system}?token=${accessToken}`
    const es = new EventSource(url)

    es.addEventListener("ticket_created", (e) => {
      qc.invalidateQueries({ queryKey: ["tickets"] })
      if (!isStaff) return
      const title = safeParse(e).title as string | undefined
      toast({
        title: "New ticket",
        description: title ? `"${title}" was just opened` : undefined,
        variant: "info",
      })
    })

    es.addEventListener("ticket_status_changed", (e) => {
      qc.invalidateQueries({ queryKey: ["tickets"] })
      if (!isStaff) return
      const status = safeParse(e).new_status as string | undefined
      if (status === "awaiting_agent_approval") {
        toast({
          title: "AI response ready",
          description: "A ticket is awaiting your review.",
          variant: "warning",
        })
      }
    })

    return () => es.close()
  }, [accessToken, qc, role, toast])
}

function safeParse(e: Event): Record<string, unknown> {
  try {
    const data = (e as MessageEvent).data
    return data ? JSON.parse(data) : {}
  } catch {
    return {}
  }
}
