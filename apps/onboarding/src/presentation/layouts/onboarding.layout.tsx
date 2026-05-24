import { Link, Outlet } from "react-router"
import { ThemeSwitcher } from "@nexus/theme"
import { GithubLogoIcon } from "@phosphor-icons/react"

export function OnboardingLayout() {
  return (
    <div className="relative flex min-h-dvh flex-col bg-(--bg)">
      {/* Top accent strip */}
      <div className="h-0.5 w-full bg-(--accent)" />

      {/* Background: grid lines with radial vignette */}
      <div
        className="pointer-events-none absolute inset-0"
        style={{
          backgroundImage: `
            linear-gradient(var(--border) 1px, transparent 1px),
            linear-gradient(90deg, var(--border) 1px, transparent 1px)
          `,
          backgroundSize: "40px 40px",
          opacity: 0.35,
        }}
      />
      <div
        className="pointer-events-none absolute inset-0"
        style={{
          background:
            "radial-gradient(ellipse 80% 60% at 50% 0%, transparent 40%, var(--bg) 100%)",
        }}
      />

      {/* Header */}
      <header className="relative z-20 flex items-center justify-between border-b border-(--border)/40 px-6 py-4">
        <Link
          to="/"
          className="flex items-center gap-2 font-mono text-sm transition-opacity hover:opacity-75"
        >
          <span className="font-semibold text-(--accent)">◈</span>
          <span className="text-(--fg) font-medium">nexus</span>
          <span className="text-(--border)">/</span>
          <span className="text-(--muted)">onboarding</span>
        </Link>

        <div className="flex items-center gap-3">
          <nav className="hidden sm:flex items-center gap-1">
            <Link
              to="/about"
              className="font-mono text-xs text-(--muted) px-3 py-1.5 rounded-sm hover:bg-(--surface) hover:text-(--fg) transition-colors"
            >
              about
            </Link>
            <a
              href="https://github.com/Lusk1nha/nexus-helpdesk"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-1.5 font-mono text-xs text-(--muted) px-3 py-1.5 rounded-sm hover:bg-(--surface) hover:text-(--fg) transition-colors"
            >
              <GithubLogoIcon className="h-3.5 w-3.5" />
              github
            </a>
          </nav>
          <div className="h-4 w-px bg-(--border) hidden sm:block" />
          <ThemeSwitcher />
        </div>
      </header>

      {/* Content */}
      <main className="relative z-10 flex flex-1 items-center justify-center px-4 py-8 sm:px-6 sm:py-12">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="relative z-10 border-t border-(--border)/40 px-6 py-4">
        <div className="flex flex-col sm:flex-row items-center justify-between gap-2">
          <p className="font-mono text-[11px] text-(--border)">
            nexus_engine v1.0 ·{" "}
            <span className="text-(--accent)">multi-tenant</span> · ai-powered ·
            realtime
          </p>
          <p className="font-mono text-[11px] text-(--border)">
            built by{" "}
            <a
              href="https://github.com/Lusk1nha"
              target="_blank"
              rel="noopener noreferrer"
              className="text-(--muted) hover:text-(--fg) transition-colors"
            >
              Lucas Pedro da Hora
            </a>
          </p>
        </div>
      </footer>
    </div>
  )
}
