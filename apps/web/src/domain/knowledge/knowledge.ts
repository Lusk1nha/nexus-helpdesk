export interface KnowledgeDocument {
  id: string
  title: string
  contentPreview: string
  documentType: string
  sourceTicketId: string
  indexedAt: number
  indexedBy: string
}

export interface CreateKnowledgeInput {
  title: string
  content: string
}
