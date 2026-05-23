import { MessageSquare } from 'lucide-react'

import { useSession } from '@/application/auth/use-session'

export function DashboardPage() {
  const user = useSession()

  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] text-center space-y-4">
      <MessageSquare className="h-10 w-10 text-[var(--border)]" />
      <div>
        <h1 className="text-base font-mono font-semibold text-[var(--fg)]">
          Welcome back, <span className="text-[var(--accent)]">{user?.role}</span>
        </h1>
        <p className="text-xs font-mono text-[var(--muted)] mt-1">
          Ticket management is coming next.
        </p>
      </div>
      <pre className="text-[10px] font-mono text-[var(--border)] text-left">
        {`tenant_id: ${user?.tenantId ?? '...'}\nuser_id:   ${user?.userId ?? '...'}\nrole:      ${user?.role ?? '...'}`}
      </pre>
    </div>
  )
}
