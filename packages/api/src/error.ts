export interface ApiErrorPayload {
  error: {
    code: string
    message: string
  }
}

export interface NexusApiError extends Error {
  name: "NexusApiError"
  code: string
  status: number
}

export function isNexusApiError(error: unknown): error is NexusApiError {
  return error instanceof Error && error.name === "NexusApiError"
}
