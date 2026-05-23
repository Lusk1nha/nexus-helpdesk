import { motion, type Variants } from "motion/react"
import {
  GithubLogoIcon,
  LinkedinLogoIcon,
  EnvelopeSimpleIcon,
  TerminalIcon,
  CoffeeIcon,
  GameControllerIcon,
  ActivityIcon,
  MapPinIcon,
  HeartIcon,
} from "@phosphor-icons/react"

import { Avatar, AvatarImage, AvatarFallback } from "@nexus/ui"
import { TerminalTyping } from "@/presentation/components/terminal-typing"
import { TitleTypingAnimation } from "@/presentation/components/title-typing"

interface AboutArchitectProps {
  variants: Variants
}

export function AboutArchitect({ variants }: AboutArchitectProps) {
  return (
    <motion.div variants={variants} className="flex flex-col gap-4">
      <div className="flex items-center gap-2 border-b border-(--border) pb-2">
        <TerminalIcon className="h-5 w-5 text-(--accent)" />

        <h2 className="font-mono text-xl font-semibold text-(--fg)">
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
        </h2>
      </div>

      <div>
        {/* Header do Autor com Avatar */}
        <div className="mb-4 flex items-center gap-4">
          <Avatar className="size-16 ring-2 ring-(--border)">
            <AvatarImage
              src="https://avatars.githubusercontent.com/u/61957312?v=4"
              alt="Lucas Pedro da Hora"
            />
            <AvatarFallback>LP</AvatarFallback>
          </Avatar>

          <div>
            <h3 className="font-mono text-lg font-bold text-(--fg)">
              Lucas Pedro da Hora
            </h3>
            <p className="mb-1 font-mono text-xs text-(--accent)">
              Senior Full Stack Developer
            </p>
            <p className="flex items-center gap-1 font-mono text-[10px] tracking-wider text-(--muted) uppercase">
              <MapPinIcon className="h-3 w-3" /> São Paulo, SP
            </p>
          </div>
        </div>

        {/* Efeito de Terminal na Bio */}
        <p className="mb-5 min-h-25 text-sm leading-relaxed text-(--muted) sm:min-h-20">
          <TerminalTyping
            delay={600}
            text="With over 4 years of experience building high-performance web systems, I currently work at Hub Brasil, focusing on scalable cloud platforms and LLM integrations. I value clean code, strict typing, and well-defined architectures like DDD and Microservices."
          />
        </p>

        <div className="mb-6 grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <span className="font-mono text-xs tracking-wider text-(--muted) uppercase">
              Tech Stack
            </span>
            <ul className="space-y-1 text-sm text-(--fg)">
              <li>• React & TypeScript</li>
              <li>• Node.js & NestJS</li>
              <li>• Rust (Axum, Tauri)</li>
              <li>• GCP & Docker</li>
            </ul>
          </div>
          <div className="space-y-2">
            <span className="font-mono text-xs tracking-wider text-(--muted) uppercase">
              Beyond Code
            </span>
            <ul className="space-y-2 text-sm text-(--fg)">
              <li className="flex items-center gap-2">
                <ActivityIcon className="h-4 w-4 text-(--muted)" /> 5k/10k Runner
              </li>
              <li className="flex items-center gap-2">
                <CoffeeIcon className="h-4 w-4 text-(--muted)" /> Coffee Enthusiast
              </li>
              <li className="flex items-center gap-2">
                <GameControllerIcon className="h-4 w-4 text-(--muted)" /> Indie Gamer
              </li>
              <li className="flex items-center gap-2">
                <HeartIcon className="h-4 w-4 text-(--muted)" /> Mutual Lover
              </li>
            </ul>
          </div>
        </div>

        {/* Redes Sociais */}
        <div className="flex gap-3">
          <a
            href="https://github.com/Lusk1nha"
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-2 rounded-md border border-(--border) bg-(--surface) px-3 py-1.5 text-sm transition-colors hover:border-(--accent) hover:text-(--accent)"
          >
            <GithubLogoIcon className="h-4 w-4" /> GitHub
          </a>
          <a
            href="https://www.linkedin.com/in/olucaspedro/"
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-2 rounded-md border border-(--border) bg-(--surface) px-3 py-1.5 text-sm transition-colors hover:border-(--accent) hover:text-(--accent)"
          >
            <LinkedinLogoIcon className="h-4 w-4" /> LinkedIn
          </a>
          <a
            href="mailto:lucaspedro517@gmail.com"
            className="flex items-center gap-2 rounded-md border border-(--border) bg-(--surface) px-3 py-1.5 text-sm transition-colors hover:border-(--accent) hover:text-(--accent)"
          >
            <EnvelopeSimpleIcon className="h-4 w-4" /> Email
          </a>
        </div>
      </div>
    </motion.div>
  )
}