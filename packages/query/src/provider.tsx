import {
  QueryClient,
  QueryClientProvider,
  type QueryClientConfig,
} from "@tanstack/react-query"
import { useState } from "react"

interface QueryProviderProps {
  children: React.ReactNode
  config?: QueryClientConfig
}

export function QueryProvider({ children, config }: QueryProviderProps) {
  const [client] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            staleTime: 30_000,
            retry: 1,
          },
        },
        ...config,
      })
  )

  return <QueryClientProvider client={client}>{children}</QueryClientProvider>
}
