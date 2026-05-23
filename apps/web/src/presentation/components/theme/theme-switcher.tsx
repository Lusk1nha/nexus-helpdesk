import { Monitor } from 'lucide-react'
import { useState } from 'react'

import { cn } from '@/lib/utils'
import { themes, type ThemeId } from '@/presentation/theme/themes'
import { useTheme } from '@/presentation/providers/theme.provider'

export function ThemeSwitcher({ className }: { className?: string }) {
  const { theme, setTheme } = useTheme()
  const [open, setOpen] = useState(false)

  return (
    <div className={cn('relative', className)}>
      <button
        onClick={() => setOpen((o) => !o)}
        className={cn(
          'flex h-7 items-center gap-1.5 px-2',
          'font-mono text-xs text-[var(--muted)]',
          'rounded-sm border border-[var(--border)] bg-[var(--surface)]',
          'transition-colors hover:border-[var(--muted)] hover:text-[var(--fg)]',
        )}
        aria-label="Switch theme"
        title="Switch theme"
      >
        <Monitor className="h-3 w-3" />
        <span className="hidden sm:inline">
          {themes.find((t) => t.id === theme)?.name ?? 'Theme'}
        </span>
      </button>

      {open && (
        <>
          {/* Backdrop */}
          <div className="fixed inset-0 z-40" onClick={() => setOpen(false)} />

          {/* Dropdown */}
          <div
            className={cn(
              'absolute top-full right-0 z-50 mt-1 min-w-[180px]',
              'rounded-sm border border-[var(--border)] bg-[var(--surface)]',
              'overflow-hidden shadow-lg shadow-black/20',
            )}
          >
            {themes.map((t) => (
              <button
                key={t.id}
                onClick={() => {
                  setTheme(t.id as ThemeId)
                  setOpen(false)
                }}
                className={cn(
                  'flex w-full items-center gap-2.5 px-3 py-2 text-left',
                  'font-mono text-xs text-[var(--muted)]',
                  'transition-colors hover:bg-[var(--surface-2)] hover:text-[var(--fg)]',
                  theme === t.id && 'bg-[var(--surface-2)] text-[var(--fg)]',
                )}
              >
                {/* Color swatch */}
                <span
                  className="h-3 w-3 shrink-0 rounded-full border border-white/10"
                  style={{ backgroundColor: t.accentHex }}
                />
                <span>{t.name}</span>
                {theme === t.id && <span className="ml-auto text-[var(--accent)]">✓</span>}
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  )
}
