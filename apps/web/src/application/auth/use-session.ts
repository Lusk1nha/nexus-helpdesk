import { useAuthStore } from '@/infrastructure/store/auth.store'

/** Returns the current authenticated user, or null if not logged in. */
export function useSession() {
  return useAuthStore((s) => s.user)
}

export function useIsAuthenticated() {
  return useAuthStore((s) => s.isAuthenticated())
}

export function useLogout() {
  const clear = useAuthStore((s) => s.clear)
  return clear
}
