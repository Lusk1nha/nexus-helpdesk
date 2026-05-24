import { FloppyDiskIcon, PaintBrushIcon, CheckIcon } from "@phosphor-icons/react"
import { themes, type ThemeId } from "@nexus/theme"
import { useEffect, useState } from "react"

import { Button, FormError } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useTenant, useUpdateTenant } from "@/application/tenant/use-tenant"

export function ThemePicker() {
  const { data: tenant } = useTenant()
  const update = useUpdateTenant()

  const [selected, setSelected] = useState<ThemeId>("midnight")
  const [isDirty, setIsDirty] = useState(false)

  useEffect(() => {
    if (tenant) {
      setSelected((tenant.theme as ThemeId) ?? "midnight")
      setIsDirty(false)
    }
  }, [tenant])

  const handlePick = (id: ThemeId) => {
    setSelected(id)
    setIsDirty(id !== ((tenant?.theme as ThemeId) ?? "midnight"))
  }

  const handleSave = async () => {
    await update.mutateAsync({ theme: selected })
    setIsDirty(false)
  }

  return (
    <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
      <div className="relative border-b border-(--border) px-5 py-4">
        <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-(--accent)/40 via-(--accent)/20 to-transparent" />
        <div className="flex items-center gap-2">
          <PaintBrushIcon className="h-3.5 w-3.5 text-(--accent)" />
          <p className="font-mono text-[10px] font-semibold uppercase tracking-widest text-(--muted)">
            workspace theme
          </p>
        </div>
        <p className="mt-1 font-mono text-xs text-(--fg)">
          Default theme for your agents' workspace
        </p>
      </div>

      <div className="space-y-5 px-5 py-5">
        <FormError error={update.error} fallbackMessage="Failed to update theme." />

        <div className="grid grid-cols-3 gap-3 sm:grid-cols-5">
          {themes.map((t) => {
            const isSelected = selected === t.id
            return (
              <button
                key={t.id}
                type="button"
                onClick={() => handlePick(t.id)}
                title={t.name}
                className={cn(
                  "group relative flex flex-col items-center gap-2 rounded-sm border p-3 transition-all duration-150",
                  isSelected
                    ? "border-(--accent)/60 bg-(--accent)/8 shadow-sm"
                    : "border-(--border) hover:border-(--muted)/60 hover:bg-(--surface-2)"
                )}
              >
                <span
                  className="h-7 w-7 rounded-full border-2 border-(--border) shadow-sm transition-transform duration-150 group-hover:scale-110"
                  style={{ backgroundColor: t.accentHex }}
                />
                <span
                  className={cn(
                    "font-mono text-[10px] leading-tight text-center transition-colors",
                    isSelected ? "text-(--fg)" : "text-(--muted) group-hover:text-(--fg)"
                  )}
                >
                  {t.name}
                </span>
                {isSelected && (
                  <span className="absolute top-1.5 right-1.5 flex h-3.5 w-3.5 items-center justify-center rounded-full bg-(--accent)">
                    <CheckIcon className="h-2 w-2 text-white" weight="bold" />
                  </span>
                )}
              </button>
            )
          })}
        </div>

        <div className="flex items-center justify-between border-t border-(--border) pt-4">
          {update.isSuccess && (
            <p className="font-mono text-xs text-(--success)">✓ theme applied</p>
          )}
          <div className="ml-auto">
            <Button type="button" size="sm" disabled={!isDirty} loading={update.isPending} onClick={handleSave}>
              <FloppyDiskIcon className="h-3.5 w-3.5" />
              {update.isPending ? "Applying..." : "Apply theme"}
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}
