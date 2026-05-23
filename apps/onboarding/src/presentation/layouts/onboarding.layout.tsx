import { Link, Outlet } from "react-router"
import { ThemeSwitcher } from "@nexus/theme"

export function OnboardingLayout() {
  return (
    <div className="relative flex min-h-dvh flex-col bg-(--bg)">
      {/* Background: Grid de pontos identico ao app web */}
      <div
        className="pointer-events-none absolute inset-0"
        style={{
          backgroundImage:
            "radial-gradient(circle, var(--border) 1px, transparent 1px)",
          backgroundSize: "28px 28px",
          opacity: 0.5,
        }}
      />

      {/* Top Bar / Header */}
      <header className="relative z-20 flex items-center justify-between border-b border-(--border)/50 px-6 py-4">
        <Link
          to="/"
          className="flex items-center gap-2 font-mono text-sm transition-opacity hover:opacity-80"
          title="Return to home"
        >
          <span className="font-semibold text-(--accent)">◈</span>
          <span className="text-(--muted)">nexus</span>
          <span className="text-(--border)">/</span>
          <span className="text-(--fg)">onboarding</span>
        </Link>
        <ThemeSwitcher />
      </header>

      {/* Content Injection Layer */}
      <main className="relative z-10 flex flex-1 items-center justify-center p-6 py-12">
        <Outlet />
      </main>

      {/* Corporate Footer com Créditos */}
      <footer className="relative z-10 flex flex-col items-center justify-center gap-1.5 border-t border-(--border)/50 px-6 py-4">
        <p className="text-center font-mono text-xs text-(--muted)">
          nexus_engine v1.0 —{" "}
          <span className="text-(--accent)">multi-tenant</span> · ai-powered ·
          realtime
        </p>
        <p className="text-center font-mono text-xs text-(--muted)">
          built and owned by{" "}
          <a
            href="https://github.com/Lusk1nha"
            target="_blank"
            rel="noopener noreferrer"
            className="text-(--fg) transition-colors hover:text-(--accent) hover:underline hover:underline-offset-2"
          >
            Lucas Pedro da Hora
          </a>
        </p>
      </footer>
    </div>
  )
}
