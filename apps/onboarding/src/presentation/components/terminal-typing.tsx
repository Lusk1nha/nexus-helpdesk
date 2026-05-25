import { motion } from "motion/react"
import { useEffect, useState } from "react"

interface TerminalTypingProps {
  text: string
  delay?: number
  speed?: number
}

export function TerminalTyping({
  text,
  delay = 0,
  speed = 15,
}: TerminalTypingProps) {
  const [displayedText, setDisplayedText] = useState("")

  useEffect(() => {
    // 1. Declarar as variáveis no escopo correto para o cleanup
    let timeoutId: ReturnType<typeof setTimeout>
    let intervalId: ReturnType<typeof setInterval>

    // 2. Reseta o texto caso a prop 'text' mude com o componente já montado
    setDisplayedText("")

    timeoutId = setTimeout(() => {
      let i = 0
      intervalId = setInterval(() => {
        setDisplayedText(text.substring(0, i + 1))
        i++
        if (i >= text.length) {
          clearInterval(intervalId)
        }
      }, speed)
    }, delay)

    // 3. Cleanup correto: limpa AMBOS quando o componente desmonta
    return () => {
      clearTimeout(timeoutId)
      clearInterval(intervalId)
    }
  }, [text, delay, speed])

  return (
    <>
      {/* 4. Acessibilidade: Leitores de tela leem tudo de uma vez */}
      <span className="sr-only">{text}</span>

      {/* 5. Visual: Escondemos a animação quebrada dos leitores de tela */}
      <span aria-hidden="true">
        {displayedText}
        {/* Cursor piscante do terminal */}
        <motion.span
          animate={{ opacity: [1, 0, 1] }}
          transition={{ duration: 0.8, repeat: Infinity, ease: "linear" }}
          className="ml-1 inline-block h-3.5 w-2 bg-(--accent) align-baseline"
        />
      </span>
    </>
  )
}
