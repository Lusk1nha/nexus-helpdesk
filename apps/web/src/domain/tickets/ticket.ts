export type TicketStatus =
  | "open"
  | "processing_ai"
  | "awaiting_agent_approval"
  | "resolved"
  | "closed"

export interface Ticket {
  id: string
  tenantId: string
  customerId: string
  title: string
  description: string
  status: TicketStatus
  createdAt: string
  updatedAt: string
}

export type MessageSenderType = "customer" | "agent" | "ai" | "system"

export interface TicketMessage {
  id: string
  ticketId: string
  senderId: string | null
  senderType: MessageSenderType
  content: string
  isInternalNote: boolean
  createdAt: string
}

export interface CreateTicketInput {
  title: string
  description: string
}
