import { MonitorIcon } from "@phosphor-icons/react"
import { useState } from "react"

import { useTheme } from "./provider"
import { themes, type ThemeId } from "./themes"

import { cn } from "@nexus/utils"

export function ThemeSwitcher({ className }: { className?: string }) {
  const { theme, setTheme } = useTheme()
  const [open, setOpen] = useState(false)

  return (
    <div className={cn("relative", className)}>
      <button
        onClick={() => setOpen((o) => !o)}
        className={cn(
          "flex h-7 items-center gap-1.5 px-2",
          "font-mono text-xs text-(--muted)",
          "rounded-sm border border-(--border) bg-(--surface)",
          "transition-colors hover:border-(--muted) hover:text-(--fg)"
        )}
        aria-label="Switch theme"
        title="Switch theme"
      >
        <MonitorIcon className="h-3 w-3" />
        <span className="hidden sm:inline">
          {themes.find((t) => t.id === theme)?.name ?? "Theme"}
        </span>
      </button>

      {open && (
        <>
          {/* Backdrop */}
          <div className="fixed inset-0 z-40" onClick={() => setOpen(false)} />

          {/* Dropdown */}
          <div
            className={cn(
              "absolute top-full right-0 z-50 mt-1 w-72 lg:w-96", // Aumentamos a largura para acomodar o grid
              "rounded-sm border border-(--border) bg-(--surface)",
              "overflow-hidden shadow-lg shadow-black/20"
            )}
          >
            {/* Cabeçalho opcional para dar um ar mais organizado */}
            <div className="border-b border-(--border) px-3 py-2">
              <span className="font-mono text-xs font-semibold text-(--fg)">
                Select Theme
              </span>
            </div>

            {/* Container rolável com Grid de 2 colunas */}
            <div className="flex max-h-80 flex-col gap-2 overflow-y-auto p-1">
              {["dark", "light"].map((mode) => {
                const filtered = themes.filter((t) =>
                  mode === "dark" ? t.isDark : !t.isDark
                )
                if (filtered.length === 0) return null

                return (
                  <div key={mode}>
                    <div className="px-2 py-1 text-[10px] font-bold tracking-wider text-(--muted) uppercase">
                      {mode} themes
                    </div>
                    <div className="grid grid-cols-2 gap-1">
                      {filtered
                        .sort((a, b) => a.name.localeCompare(b.name))
                        .map((t) => (
                          <button
                            key={t.id}
                            onClick={() => {
                              setTheme(t.id as ThemeId)
                              setOpen(false)
                            }}
                            className={cn(
                              "flex items-center gap-2 rounded-sm px-2 py-2 text-left font-mono text-xs text-(--muted)",
                              "transition-colors hover:bg-(--surface-2) hover:text-(--fg)",
                              theme === t.id && "bg-(--surface-2) text-(--fg)"
                            )}
                          >
                            <span
                              className="h-3 w-3 shrink-0 rounded-full border border-black/10"
                              style={{ backgroundColor: t.accentHex }}
                            />
                            <span className="truncate">{t.name}</span>
                          </button>
                        ))}
                    </div>
                  </div>
                )
              })}
            </div>
          </div>
        </>
      )}
    </div>
  )
}
