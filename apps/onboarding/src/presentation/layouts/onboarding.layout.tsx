import { Outlet } from "react-router"
import { ThemeSwitcher } from "@nexus/theme"

export function OnboardingLayout() {
  return (
    <div className="relative flex min-h-dvh flex-col bg-(--bg)">
      {/* Dot grid background (mesmo do web) */}
      <div
        className="pointer-events-none absolute inset-0"
        style={{
          backgroundImage:
            "radial-gradient(circle, var(--border) 1px, transparent 1px)",
          backgroundSize: "28px 28px",
          opacity: 0.5,
        }}
      />

      {/* Top bar minimalista */}
      <header className="relative z-20 flex items-center justify-between border-b border-(--border)/50 px-6 py-4">
        <div className="flex items-center gap-2 font-mono text-sm">
          <span className="font-semibold text-(--accent)">◈</span>
          <span className="text-(--muted)">nexus</span>
          <span className="text-(--border)">/</span>
          <span className="text-(--fg)">onboarding</span>
        </div>
        <ThemeSwitcher />
      </header>

      {/* Content */}
      <main className="relative z-10 flex flex-1 items-center justify-center p-6 py-12">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="relative z-10 flex flex-col items-center justify-center gap-1.5 border-t border-(--border)/50 px-6 py-4">
        <p className="text-center font-mono text-xs text-(--muted)">
          Empowering support teams with local AI
        </p>
      </footer>
    </div>
  )
}
