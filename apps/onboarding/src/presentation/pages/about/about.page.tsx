import { motion, type Variants } from "motion/react"

import { AboutDisclaimer } from "@/presentation/components/about/about-disclaimer"
import { AboutProject } from "@/presentation/components/about/about-project"
import { AboutArchitect } from "@/presentation/components/about/about-architect"

export function AboutPage() {
  const container: Variants = {
    hidden: { opacity: 0 },
    show: { opacity: 1, transition: { staggerChildren: 0.1 } },
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
      className="w-full max-w-5xl space-y-4 sm:space-y-6"
    >
      {/* Disclaimer — compact, not dominating */}
      <AboutDisclaimer variants={item} />

      {/* Architect hero — full width */}
      <AboutArchitect variants={item} />

      {/* Project + architecture — below */}
      <AboutProject variants={item} />
    </motion.div>
  )
}
