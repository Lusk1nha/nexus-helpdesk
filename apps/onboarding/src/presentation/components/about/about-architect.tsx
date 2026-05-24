import { motion, type Variants } from "motion/react"
import {
  GithubLogoIcon,
  LinkedinLogoIcon,
  EnvelopeSimpleIcon,
  MapPinIcon,
  ActivityIcon,
  CoffeeIcon,
  GameControllerIcon,
  HeartIcon,
} from "@phosphor-icons/react"

import { Avatar, AvatarImage, AvatarFallback } from "@nexus/ui"
import { TerminalTyping } from "@/presentation/components/terminal-typing"
import { TitleTypingAnimation } from "@/presentation/components/title-typing"

interface AboutArchitectProps {
  variants: Variants
}

const STACK_BADGES = [
  "React 19",
  "TypeScript",
  "Rust",
  "Axum",
  "Node.js",
  "NestJS",
  "GCP",
  "Docker",
  "PostgreSQL",
  "Ollama",
]

const INTERESTS = [
  { icon: ActivityIcon, label: "5k/10k Runner" },
  { icon: CoffeeIcon, label: "Coffee Enthusiast" },
  { icon: GameControllerIcon, label: "Indie Gamer" },
  { icon: HeartIcon, label: "Lover" },
]

const SOCIAL_LINKS = [
  {
    href: "https://github.com/Lusk1nha",
    icon: GithubLogoIcon,
    label: "GitHub",
  },
  {
    href: "https://www.linkedin.com/in/olucaspedro/",
    icon: LinkedinLogoIcon,
    label: "LinkedIn",
  },
  {
    href: "mailto:lucaspedro517@gmail.com",
    icon: EnvelopeSimpleIcon,
    label: "Email",
  },
]

export function AboutArchitect({ variants }: AboutArchitectProps) {
  return (
    <motion.div
      variants={variants}
      className="overflow-hidden rounded-sm border border-(--border) bg-(--surface)"
    >
      {/* Top accent strip */}
      <div className="h-0.5 w-full bg-(--accent)" />

      <div className="p-5 sm:p-6 lg:p-8">
        <div className="flex flex-col gap-6 sm:gap-8 md:flex-row md:gap-8 lg:gap-12">
          {/* Left: identity */}
          <div className="flex flex-col items-start gap-4 md:w-48 md:shrink-0 lg:w-56">
            <Avatar className="size-20 ring-2 ring-(--border) ring-offset-2 ring-offset-(--bg)">
              <AvatarImage
                src="https://avatars.githubusercontent.com/u/61957312?v=4"
                alt="Lucas Pedro da Hora"
              />
              <AvatarFallback className="font-mono text-lg">LP</AvatarFallback>
            </Avatar>

            <div>
              <h2 className="font-mono text-lg leading-tight font-bold text-(--fg)">
                Lucas Pedro da Hora
              </h2>
              <p className="mt-0.5 font-mono text-xs text-(--accent)">
                Senior Full Stack Developer
              </p>
              <p className="mt-1 flex items-center gap-1 font-mono text-[10px] text-(--muted)">
                <MapPinIcon className="h-3 w-3" />
                São Paulo, SP
              </p>
            </div>

            {/* Social */}
            <div className="flex flex-wrap gap-2">
              {SOCIAL_LINKS.map(({ href, icon: Icon, label }) => (
                <a
                  key={label}
                  href={href}
                  target="_blank"
                  rel="noreferrer"
                  className="flex items-center gap-1.5 rounded-sm border border-(--border) bg-(--surface-2) px-2.5 py-1.5 font-mono text-xs text-(--muted) transition-colors hover:border-(--accent)/50 hover:text-(--accent)"
                >
                  <Icon className="h-3.5 w-3.5" />
                  {label}
                </a>
              ))}
            </div>
          </div>

          {/* Right: content */}
          <div className="flex-1 space-y-6">
            {/* Typing title */}
            <div className="flex items-center gap-2">
              <span className="mr-1 font-mono text-xs text-(--accent)">$</span>
              <span className="font-mono text-base font-semibold text-(--fg)">
                <TitleTypingAnimation
                  delay={300}
                  typeSpeed={70}
                  texts={[
                    "The Architect_",
                    "Senior Developer_",
                    "sysadmin@nexus_",
                    "root@localhost_",
                    "Lucas Pedro_",
                  ]}
                />
              </span>
            </div>

            {/* Bio */}
            <p className="min-h-16 text-sm leading-relaxed text-(--muted)">
              <TerminalTyping
                delay={600}
                text="With over 4 years of experience building high-performance web systems, I currently work at Hub Brasil, focusing on scalable cloud platforms and LLM integrations. I value clean code, strict typing, and well-defined architectures like DDD and Microservices."
              />
            </p>

            {/* Tech stack badges */}
            <div>
              <p className="mb-2 font-mono text-[10px] tracking-widest text-(--muted) uppercase">
                Tech Stack
              </p>
              <div className="flex flex-wrap gap-1.5">
                {STACK_BADGES.map((tech) => (
                  <span
                    key={tech}
                    className="rounded-sm border border-(--border) bg-(--surface-2) px-2 py-1 font-mono text-[11px] text-(--fg) transition-colors hover:border-(--accent)/40 hover:text-(--accent)"
                  >
                    {tech}
                  </span>
                ))}
              </div>
            </div>

            {/* Interests */}
            <div>
              <p className="mb-2 font-mono text-[10px] tracking-widest text-(--muted) uppercase">
                Beyond Code
              </p>
              <div className="flex flex-wrap gap-4">
                {INTERESTS.map(({ icon: Icon, label }) => (
                  <div
                    key={label}
                    className="flex items-center gap-1.5 font-mono text-xs text-(--muted)"
                  >
                    <Icon className="h-3.5 w-3.5 text-(--border)" />
                    {label}
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </motion.div>
  )
}
