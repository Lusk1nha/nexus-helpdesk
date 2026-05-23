import { motion } from "motion/react"
import { useEffect, useState } from "react"

interface TitleTypingAnimationProps {
  /** Array de frases que serão digitadas e apagadas em loop */
  texts: string[]
  /** Atraso inicial antes de começar a primeira digitação (ms) */
  delay?: number
  /** Velocidade da digitação (ms por caractere) */
  typeSpeed?: number
  /** Velocidade ao apagar (ms por caractere) */
  deleteSpeed?: number
  /** Tempo de pausa com a frase completa antes de começar a apagar (ms) */
  pauseDuration?: number

  loop?: boolean
}

export function TitleTypingAnimation({
  texts,
  delay = 0,
  typeSpeed = 80, // Um pouco mais rápido para não entediar
  deleteSpeed = 40, // Apaga bem rápido (estilo terminal)
  pauseDuration = 2500, // Tempo suficiente para ler a frase inteira
  loop = true, // Loop infinito por padrão, mas pode ser desativado se quiser que pare após uma rodada
}: TitleTypingAnimationProps) {
  const [displayedText, setDisplayedText] = useState("")
  const [textIndex, setTextIndex] = useState(0)
  const [isDeleting, setIsDeleting] = useState(false)
  const [hasStarted, setHasStarted] = useState(delay === 0)

  useEffect(() => {
    // 1. Lida com o delay inicial antes de tudo começar
    if (!hasStarted) {
      const timeout = setTimeout(() => setHasStarted(true), delay)
      return () => clearTimeout(timeout)
    }

    const currentWord = texts[textIndex]
    let timeoutId: ReturnType<typeof setTimeout>

    // 2. Máquina de estados: Apagando
    if (isDeleting) {
      if (displayedText === "") {
        // Terminou de apagar: vai para a próxima palavra e volta a digitar
        setIsDeleting(false)
        setTextIndex((prev) => (prev + 1) % texts.length)
        timeoutId = setTimeout(() => {}, 200) // Breve pausa antes de digitar
      } else {
        if (!currentWord) return

        // Continua apagando caractere por caractere
        timeoutId = setTimeout(() => {
          setDisplayedText(currentWord?.substring(0, displayedText.length - 1))
        }, deleteSpeed)
      }
    }
    // 3. Máquina de estados: Digitando
    else {
      if (displayedText === currentWord) {
        if (!loop && textIndex === texts.length - 1) {
          return // Finaliza o effect, mantendo o cursor piscando
        }

        // Terminou de digitar: faz a pausa longa antes de começar a apagar
        timeoutId = setTimeout(() => setIsDeleting(true), pauseDuration)
      } else {
        if (!currentWord) return

        // Continua adicionando caractere por caractere
        timeoutId = setTimeout(() => {
          setDisplayedText(currentWord?.substring(0, displayedText.length + 1))
        }, typeSpeed)
      }
    }

    return () => clearTimeout(timeoutId)
  }, [
    displayedText,
    isDeleting,
    textIndex,
    texts,
    typeSpeed,
    deleteSpeed,
    pauseDuration,
    hasStarted,
    delay,
    loop
  ])

  return (
    <>
      {/* Acessibilidade: Leitores de tela leem o primeiro valor ou todos de uma vez */}
      <span className="sr-only">{texts.join(". ")}</span>

      <span aria-hidden="true" className="inline-flex items-baseline">
        {displayedText}
        {/* Cursor piscante do terminal: Usa h-[0.8em] para se adaptar ao tamanho da fonte do P ou do H1 */}
        <motion.span
          animate={{ opacity: [1, 0, 1] }}
          transition={{ duration: 0.8, repeat: Infinity, ease: "linear" }}
          className="ml-1 inline-block h-[0.8em] w-[0.1em] bg-(--accent) sm:w-[0.15em]"
        />
      </span>
    </>
  )
}
