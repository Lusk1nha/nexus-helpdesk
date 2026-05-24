export function fmtDate(value?: string | null): string {
  if (!value) return "—"
  // Normalize Postgres timestamps: "2026-05-24 13:38:58.946279 +00:00:00"
  // → "2026-05-24T13:38:58.946279+00:00"
  const normalized = value
    .trim()
    .replace(
      /^(\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}:\d{2}(?:\.\d+)?) ([+-]\d{2}:\d{2})(?::\d{2})?$/,
      "$1T$2$3"
    )
  const d = new Date(normalized)
  if (isNaN(d.getTime())) return "—"
  return d.toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  })
}
