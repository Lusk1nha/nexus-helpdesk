import { motion, type Variants } from "motion/react"

import { AboutDisclaimer } from "@/presentation/components/about/about-disclaimer"
import { AboutProject } from "@/presentation/components/about/about-project"
import { AboutArchitect } from "@/presentation/components/about/about-architect"

export function AboutPage() {
  const container: Variants = {
    hidden: { opacity: 0 },
    show: {
      opacity: 1,
      transition: { staggerChildren: 0.1 },
    },
  }

  const item: Variants = {
    hidden: { opacity: 0, y: 20 },
    show: { opacity: 1, y: 0, transition: { duration: 0.4, ease: "easeOut" } },
  }

  return (
    <motion.div
      variants={container}
      initial="hidden"
      animate="show"
      className="w-full max-w-4xl space-y-8"
    >
      <AboutDisclaimer variants={item} />

      <div className="grid gap-8 md:grid-cols-2">
        <AboutProject variants={item} />
        <AboutArchitect variants={item} />
      </div>
    </motion.div>
  )
}
