export type TicketStatus =
  | "open"
  | "processing_ai"
  | "awaiting_agent_approval"
  | "resolved"
  | "closed"

export type TicketPriority = "low" | "normal" | "high"

export interface Ticket {
  id: string
  tenantId: string
  customerId: string
  title: string
  description: string
  status: TicketStatus
  priority: TicketPriority
  category: string | null
  assigneeId: string | null
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
  priority?: TicketPriority
  category?: string
}
