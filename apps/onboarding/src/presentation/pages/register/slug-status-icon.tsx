import {
  CheckCircleIcon,
  SpinnerGapIcon,
  WarningCircleIcon,
} from "@phosphor-icons/react"

import { cn } from "@nexus/utils"

interface Props {
  /** True while the API request is in flight. */
  loading: boolean
  /** True when the server confirms the slug is free. */
  available: boolean | undefined
  /** True when local validation failed before the request fired. */
  invalid: boolean
  className?: string
}

/**
 * Visual indicator rendered inside the slug `<Input />`.
 *
 *   - spinner → request in flight
 *   - check   → slug is free
 *   - warning → slug is taken / format invalid
 *   - nothing → empty input
 */
export function SlugStatusIcon({
  loading,
  available,
  invalid,
  className,
}: Props) {
  if (loading) {
    return (
      <SpinnerGapIcon
        aria-label="Checking availability"
        className={cn("h-4 w-4 animate-spin text-(--muted)", className)}
      />
    )
  }

  if (invalid || available === false) {
    return (
      <WarningCircleIcon
        aria-label="Slug is not available"
        weight="fill"
        className={cn("h-4 w-4 text-(--destructive)", className)}
      />
    )
  }

  if (available === true) {
    return (
      <CheckCircleIcon
        aria-label="Slug is available"
        weight="fill"
        className={cn("h-4 w-4 text-(--success)", className)}
      />
    )
  }

  return null
}
