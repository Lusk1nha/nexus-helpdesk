import { useEffect, useState } from "react"

/**
 * Returns `value` after it has been stable for `delayMs`. Use to throttle
 * expensive queries while the user types (e.g. slug availability check).
 */
export function useDebouncedValue<T>(value: T, delayMs: number): T {
  const [debounced, setDebounced] = useState(value)

  useEffect(() => {
    const id = setTimeout(() => setDebounced(value), delayMs)
    return () => clearTimeout(id)
  }, [value, delayMs])

  return debounced
}
