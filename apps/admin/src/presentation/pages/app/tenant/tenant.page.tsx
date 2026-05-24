import { BuildingsIcon } from "@phosphor-icons/react"
import { motion } from "motion/react"

import { OrganizationForm } from "./organization-form"
import { ThemePicker } from "./theme-picker"

export function TenantPage() {
  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="mx-auto max-w-2xl space-y-6"
    >
      <div className="flex items-center gap-2.5">
        <div className="flex h-7 w-7 items-center justify-center rounded-sm bg-(--accent)/10">
          <BuildingsIcon className="h-3.5 w-3.5 text-(--accent)" />
        </div>
        <div>
          <h1 className="font-mono text-sm font-semibold text-(--fg)">tenant settings</h1>
          <p className="font-mono text-[10px] text-(--muted)">
            Organization details and workspace configuration
          </p>
        </div>
      </div>

      <OrganizationForm />
      <ThemePicker />
    </motion.div>
  )
}
