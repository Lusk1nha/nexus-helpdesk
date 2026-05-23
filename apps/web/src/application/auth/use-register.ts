import { useMutation } from "@tanstack/react-query"

import type { RegisterInput } from "@/domain/auth/auth.schemas"
import type { RegisterResult } from "@/domain/auth/auth.types"
import { fetchApi, http } from "@/infrastructure/http/client"
import { API } from "@/infrastructure/http/api.routes"

export function useRegister() {
  return useMutation({
    mutationFn: ({ confirmPassword: _, ...input }: RegisterInput) =>
      fetchApi<RegisterResult>(() =>
        http.post(API.identity.register, { json: input }).json()
      ),
  })
}
