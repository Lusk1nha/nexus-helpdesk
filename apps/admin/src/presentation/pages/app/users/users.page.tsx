import { UsersIcon } from "@phosphor-icons/react"
import { AnimatePresence, motion } from "motion/react"
import { useState } from "react"

import { cn } from "@nexus/utils"

import { useUsers, type TenantUser } from "@/application/users/use-users"
import { fmtDate } from "@/lib/format-date"

type RoleFilter = "all" | TenantUser["role"]

const ROLE_STYLES: Record<
  TenantUser["role"],
  { badge: string; dot: string; bg: string }
> = {
  admin: {
    badge: "bg-(--accent)/10 text-(--accent) border border-(--accent)/20",
    dot: "bg-(--accent)",
    bg: "bg-(--accent)/5",
  },
  agent: {
    badge: "bg-(--success)/10 text-(--success) border border-(--success)/20",
    dot: "bg-(--success)",
    bg: "bg-(--success)/5",
  },
  customer: {
    badge: "bg-(--surface-2) text-(--muted) border border-(--border)",
    dot: "bg-(--muted)",
    bg: "bg-(--surface-2)",
  },
}

const FILTERS: { value: RoleFilter; label: string }[] = [
  { value: "all", label: "All" },
  { value: "admin", label: "Admin" },
  { value: "agent", label: "Agent" },
  { value: "customer", label: "Customer" },
]

function UserAvatar({ email, role }: { email: string; role: TenantUser["role"] }) {
  const initial = email[0]?.toUpperCase() ?? "?"
  const s = ROLE_STYLES[role]
  return (
    <div
      className={cn(
        "flex h-8 w-8 shrink-0 items-center justify-center rounded-sm",
        s.bg
      )}
    >
      <span className="font-mono text-xs font-semibold text-(--fg)">{initial}</span>
    </div>
  )
}

export function UsersPage() {
  const { data: users, isLoading, error } = useUsers()
  const [filter, setFilter] = useState<RoleFilter>("all")
  const [search, setSearch] = useState("")

  const filtered = users?.filter((u) => {
    const matchRole = filter === "all" || u.role === filter
    const matchSearch = u.email.toLowerCase().includes(search.toLowerCase())
    return matchRole && matchSearch
  })

  const countFor = (role: RoleFilter) =>
    role === "all"
      ? (users?.length ?? 0)
      : (users?.filter((u) => u.role === role).length ?? 0)

  const STAT_ITEMS = [
    { label: "total", value: users?.length ?? 0, color: "text-(--fg)" },
    { label: "admins", value: countFor("admin"), color: "text-(--accent)" },
    { label: "agents", value: countFor("agent"), color: "text-(--success)" },
    { label: "customers", value: countFor("customer"), color: "text-(--muted)" },
  ]

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="mx-auto max-w-3xl space-y-5"
    >
      {/* Page header */}
      <div className="flex items-center gap-2.5">
        <div className="flex h-7 w-7 items-center justify-center rounded-sm bg-(--accent)/10">
          <UsersIcon className="h-3.5 w-3.5 text-(--accent)" />
        </div>
        <div>
          <h1 className="font-mono text-sm font-semibold text-(--fg)">users</h1>
          <p className="font-mono text-[10px] text-(--muted)">Tenant members and their roles</p>
        </div>
      </div>

      {/* Stats row */}
      {!isLoading && users && (
        <div className="grid grid-cols-4 overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
          {STAT_ITEMS.map(({ label, value, color }, i) => (
            <div
              key={label}
              className={cn(
                "flex flex-col items-center px-3 py-3 sm:px-5",
                i !== 0 && "border-l border-(--border)"
              )}
            >
              <span className={cn("font-mono text-base font-semibold sm:text-lg", color)}>
                {value}
              </span>
              <span className="mt-0.5 font-mono text-[10px] text-(--muted)">{label}</span>
            </div>
          ))}
        </div>
      )}

      {/* Users list */}
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Filters + search */}
        <div className="relative flex flex-col gap-3 border-b border-(--border) px-5 py-3 sm:flex-row sm:items-center sm:justify-between">
          <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-(--accent)/30 via-(--accent)/15 to-transparent" />
          <div className="flex items-center gap-0.5">
            {FILTERS.map(({ value, label }) => (
              <button
                key={value}
                onClick={() => setFilter(value)}
                className={cn(
                  "flex items-center gap-1.5 rounded-sm px-2.5 py-1.5 font-mono text-xs transition-colors",
                  filter === value
                    ? "bg-(--accent)/10 text-(--fg)"
                    : "text-(--muted) hover:bg-(--surface-2) hover:text-(--fg)"
                )}
              >
                {label}
                <span className="rounded-sm bg-(--surface-2) px-1 py-0.5 text-[10px] text-(--border)">
                  {countFor(value)}
                </span>
              </button>
            ))}
          </div>

          <input
            type="text"
            placeholder="search by email..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className={cn(
              "h-7 w-full rounded-sm border border-(--border) bg-(--bg) px-3",
              "font-mono text-xs text-(--fg) placeholder:text-(--border)",
              "outline-none focus:border-(--accent) transition-colors",
              "sm:w-48"
            )}
          />
        </div>

        {isLoading ? (
          <div className="divide-y divide-(--border)">
            {[1, 2, 3, 4].map((i) => (
              <div key={i} className="flex items-center gap-3 px-5 py-3.5">
                <div className="h-8 w-8 animate-pulse rounded-sm bg-(--surface-2)" />
                <div className="flex-1 space-y-1.5">
                  <div className="h-3 w-36 animate-pulse rounded-sm bg-(--surface-2)" />
                  <div className="h-2 w-24 animate-pulse rounded-sm bg-(--surface-2)" />
                </div>
                <div className="h-5 w-16 animate-pulse rounded-sm bg-(--surface-2)" />
              </div>
            ))}
          </div>
        ) : error ? (
          <div className="px-5 py-8 text-center">
            <p className="font-mono text-xs text-(--destructive)">failed to load users</p>
          </div>
        ) : !filtered || filtered.length === 0 ? (
          <div className="flex flex-col items-center gap-3 px-5 py-14 text-center">
            <div className="flex h-12 w-12 items-center justify-center rounded-sm border border-(--border) bg-(--surface-2)">
              <UsersIcon className="h-5 w-5 text-(--border)" />
            </div>
            <p className="font-mono text-xs font-medium text-(--fg)">no users found</p>
          </div>
        ) : (
          <motion.ul
            initial="hidden"
            animate="show"
            variants={{ hidden: {}, show: { transition: { staggerChildren: 0.04 } } }}
            className="divide-y divide-(--border)"
          >
            <AnimatePresence>
              {filtered.map((user) => {
                const s = ROLE_STYLES[user.role]
                return (
                  <motion.li
                    key={user.id}
                    initial={{ opacity: 0, x: -6 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0 }}
                    transition={{ duration: 0.15 }}
                    className="flex items-center gap-3 px-5 py-3.5 transition-colors hover:bg-(--surface-2)/40"
                  >
                    <UserAvatar email={user.email} role={user.role} />

                    <div className="min-w-0 flex-1">
                      <p className="truncate font-mono text-xs font-medium text-(--fg)">
                        {user.email}
                      </p>
                      <p className="font-mono text-[10px] text-(--border) mt-0.5 truncate">
                        {user.id}
                      </p>
                    </div>

                    <div className="flex shrink-0 items-center gap-3">
                      <span
                        className={cn(
                          "inline-flex items-center gap-1.5 rounded-sm px-2 py-0.5 font-mono text-[10px] font-medium",
                          s.badge
                        )}
                      >
                        <span className={cn("h-1.5 w-1.5 shrink-0 rounded-full", s.dot)} />
                        {user.role}
                      </span>
                      <span className="hidden font-mono text-[10px] text-(--border) sm:block">
                        {fmtDate(user.createdAt)}
                      </span>
                    </div>
                  </motion.li>
                )
              })}
            </AnimatePresence>
          </motion.ul>
        )}
      </div>
    </motion.div>
  )
}
