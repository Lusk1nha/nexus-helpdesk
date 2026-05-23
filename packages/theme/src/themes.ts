export type ThemeId = "midnight" | "dawn" | "dracula" | "nord" | "catppuccin"

export interface Theme {
  id: ThemeId
  name: string
  description: string
  isDark: boolean
  /** Accent hex for theme preview swatch. */
  accentHex: string
}

/**
 * Central registry of available themes.
 * To add a new theme: add an entry here + define CSS vars in theme.css under [data-theme="<id>"].
 */
export const themes: Theme[] = [
  {
    id: "midnight",
    name: "Midnight",
    description: "GitHub dark — the default",
    isDark: true,
    accentHex: "#58a6ff",
  },
  {
    id: "dawn",
    name: "Dawn",
    description: "Clean minimal light",
    isDark: false,
    accentHex: "#0969da",
  },
  {
    id: "dracula",
    name: "Dracula",
    description: "Classic vampire palette",
    isDark: true,
    accentHex: "#bd93f9",
  },
  {
    id: "nord",
    name: "Nord",
    description: "Arctic blue tones",
    isDark: true,
    accentHex: "#88c0d0",
  },
  {
    id: "catppuccin",
    name: "Catppuccin",
    description: "Soothing pastel mocha",
    isDark: true,
    accentHex: "#cba6f7",
  },
]

export const defaultTheme: ThemeId = "midnight"
