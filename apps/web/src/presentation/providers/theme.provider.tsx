import { createContext, useContext, useEffect, useState } from 'react'

import { defaultTheme, type ThemeId } from '@/presentation/theme/themes'

const STORAGE_KEY = 'nexus:theme'

interface ThemeContextValue {
  theme: ThemeId
  setTheme: (id: ThemeId) => void
}

const ThemeContext = createContext<ThemeContextValue | null>(null)

function getStoredTheme(): ThemeId {
  try {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored) return stored as ThemeId
  } catch {}
  return defaultTheme
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setThemeState] = useState<ThemeId>(getStoredTheme)

  useEffect(() => {
    document.documentElement.dataset.theme = theme
    localStorage.setItem(STORAGE_KEY, theme)
  }, [theme])

  // Apply stored theme on first render (before any state update)
  useEffect(() => {
    document.documentElement.dataset.theme = getStoredTheme()
  }, [])

  const setTheme = (id: ThemeId) => {
    setThemeState(id)
  }

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  )
}

export function useTheme() {
  const ctx = useContext(ThemeContext)
  if (!ctx) throw new Error('useTheme must be used inside <ThemeProvider>')
  return ctx
}
