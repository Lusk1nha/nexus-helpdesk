import { motion } from "motion/react"

import { cn } from "@nexus/utils"

function SkeletonBox({ className }: { className: string }) {
  return (
    <div
      className={cn("animate-pulse rounded-sm bg-(--surface-2)", className)}
    />
  )
}

export function PageLoader() {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.15 }}
      className="mx-auto max-w-3xl space-y-5"
    >
      {/* Page header */}
      <div className="flex items-center gap-2.5">
        <SkeletonBox className="h-7 w-7 shrink-0" />
        <div className="space-y-1.5">
          <SkeletonBox className="h-3 w-28" />
          <SkeletonBox className="h-2 w-44" />
        </div>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-4 overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {Array.from({ length: 4 }).map((_, i) => (
          <div
            key={i}
            className={cn(
              "flex flex-col items-center gap-1.5 px-5 py-3",
              i !== 0 && "border-l border-(--border)"
            )}
          >
            <SkeletonBox className="h-5 w-8" />
            <SkeletonBox className="h-2 w-12" />
          </div>
        ))}
      </div>

      {/* Main card */}
      <div className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)">
        {/* Card header */}
        <div className="flex items-center justify-between border-b border-(--border) px-5 py-3">
          <div className="flex items-center gap-2">
            <SkeletonBox className="h-2.5 w-20" />
            <SkeletonBox className="h-4 w-12" />
            <SkeletonBox className="h-4 w-16" />
            <SkeletonBox className="h-4 w-20" />
          </div>
          <SkeletonBox className="h-6 w-28" />
        </div>

        {/* List rows */}
        {Array.from({ length: 5 }).map((_, i) => (
          <div
            key={i}
            className={cn(
              "flex items-center gap-3 px-5 py-4",
              i !== 0 && "border-t border-(--border)"
            )}
          >
            <SkeletonBox className="h-8 w-8 shrink-0" />
            <div className="flex-1 space-y-2">
              <SkeletonBox
                className={`h-2.5 ${["w-2/5", "w-1/2", "w-3/5", "w-2/5", "w-1/3"][i]}`}
              />
              <SkeletonBox
                className={`h-2 ${["w-1/4", "w-1/3", "w-1/4", "w-2/5", "w-1/4"][i]}`}
              />
            </div>
            <SkeletonBox className="h-5 w-16 shrink-0" />
          </div>
        ))}
      </div>
    </motion.div>
  )
}
