import { motion } from "motion/react"
import { Link } from "react-router"
import { ArrowLeftIcon } from "@phosphor-icons/react"

import { buttonVariants } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { paths } from "@/presentation/router/paths"

export function NotFoundPage() {
  return (
    <motion.div
      initial={{ opacity: 0, y: 16 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="flex w-full max-w-md flex-col items-center text-center"
    >
      <p className="mb-2 font-mono text-xs text-(--muted)">
        <span className="text-(--destructive)">$</span> route not_found
      </p>

      <h1 className="mb-2 font-mono text-5xl font-bold tracking-tight text-(--fg)">
        404
      </h1>

      <p className="mb-8 font-mono text-sm text-(--muted)">
        the page you were looking for could not be resolved.
      </p>

      <Link
        to={paths.home}
        className={cn(
          buttonVariants({ variant: "outline" }),
          "font-mono text-sm"
        )}
      >
        <ArrowLeftIcon className="mr-2 h-4 w-4" /> back to home
      </Link>
    </motion.div>
  )
}
