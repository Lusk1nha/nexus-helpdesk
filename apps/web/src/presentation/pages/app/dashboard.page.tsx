import { MessageSquare } from "lucide-react"

import { useSession } from "@/application/auth/use-session"

export function DashboardPage() {
  const user = useSession()

  return (
    <div className="flex min-h-[60vh] flex-col items-center justify-center space-y-4 text-center">
      <MessageSquare className="h-10 w-10 text-(--border)" />
      <div>
        <h1 className="font-mono text-base font-semibold text-(--fg)">
          Welcome back,{" "}
          <span className="text-(--accent)">{user?.role}</span>
        </h1>
        <p className="mt-1 font-mono text-xs text-(--muted)">
          Ticket management is coming next.
        </p>
      </div>
      <pre className="text-left font-mono text-[10px] text-(--border)">
        {`tenant_id: ${user?.tenantId ?? "..."}\nuser_id:   ${user?.userId ?? "..."}\nrole:      ${user?.role ?? "..."}`}
      </pre>
    </div>
  )
}
