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
          'flex items-center gap-1.5 px-2 h-7',
          'text-xs font-mono text-[var(--muted)]',
          'border border-[var(--border)] rounded-sm bg-[var(--surface)]',
          'hover:text-[var(--fg)] hover:border-[var(--muted)] transition-colors',
        )}
        aria-label="Switch theme"
        title="Switch theme"
      >
        <Monitor className="h-3 w-3" />
        <span className="hidden sm:inline">{themes.find((t) => t.id === theme)?.name ?? 'Theme'}</span>
      </button>

      {open && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 z-40"
            onClick={() => setOpen(false)}
          />

          {/* Dropdown */}
          <div className={cn(
            'absolute right-0 top-full mt-1 z-50 min-w-[180px]',
            'border border-[var(--border)] rounded-sm bg-[var(--surface)]',
            'shadow-lg shadow-black/20 overflow-hidden',
          )}>
            {themes.map((t) => (
              <button
                key={t.id}
                onClick={() => { setTheme(t.id as ThemeId); setOpen(false) }}
                className={cn(
                  'flex items-center gap-2.5 w-full px-3 py-2 text-left',
                  'text-xs font-mono text-[var(--muted)]',
                  'hover:bg-[var(--surface-2)] hover:text-[var(--fg)] transition-colors',
                  theme === t.id && 'text-[var(--fg)] bg-[var(--surface-2)]',
                )}
              >
                {/* Color swatch */}
                <span
                  className="h-3 w-3 rounded-full border border-white/10 shrink-0"
                  style={{ backgroundColor: t.accentHex }}
                />
                <span>{t.name}</span>
                {theme === t.id && (
                  <span className="ml-auto text-[var(--accent)]">✓</span>
                )}
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  )
}
