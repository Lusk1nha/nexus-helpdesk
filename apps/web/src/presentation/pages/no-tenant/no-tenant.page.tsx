import { ArrowSquareOutIcon, BuildingsIcon } from "@phosphor-icons/react"

/**
 * Shown when the user reaches the app without a tenant subdomain
 * (e.g. plain localhost:5173 in production, or a typo in the URL).
 * Directs them to onboarding to register or find their company URL.
 */
export function NoTenantPage() {
  return (
    <div className="flex min-h-dvh flex-col items-center justify-center bg-(--bg) p-6">
      <div className="w-full max-w-sm space-y-6 text-center">
        <div className="flex justify-center">
          <div className="flex h-12 w-12 items-center justify-center rounded-sm border border-(--border) bg-(--surface)">
            <BuildingsIcon className="h-5 w-5 text-(--muted)" />
          </div>
        </div>

        <div className="space-y-2">
          <p className="font-mono text-xs text-(--muted)">
            <span className="text-(--success)">$</span> nexus resolve-tenant
          </p>
          <h1 className="font-mono text-base font-semibold text-(--fg)">
            Nenhuma empresa encontrada
          </h1>
          <p className="font-mono text-xs text-(--muted)">
            Esta URL não corresponde a nenhuma empresa.
            <br />
            Acesse pelo endereço da sua organização:
          </p>
          <p className="font-mono text-sm text-(--accent)">
            {"{sua-empresa}"}.nexus.com
          </p>
        </div>

        <div className="space-y-2">
          <a
            href={`${import.meta.env.VITE_ONBOARDING_URL ?? "http://onboarding.nexus.localhost"}`}
            className="flex items-center justify-center gap-2 rounded-sm border border-(--accent) bg-transparent px-4 py-2 font-mono text-xs text-(--accent) transition-colors hover:bg-(--accent) hover:text-(--accent-fg)"
          >
            <ArrowSquareOutIcon className="h-3.5 w-3.5" />
            Criar uma conta
          </a>
          <p className="font-mono text-[10px] text-(--border)">
            Já tem conta? Peça o link para o administrador da sua empresa.
          </p>
        </div>
      </div>
    </div>
  )
}
