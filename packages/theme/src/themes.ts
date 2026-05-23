export type ThemeId =
  | "midnight"
  | "dawn"
  | "dracula"
  | "nord"
  | "catppuccin"
  | "rose-pine"
  | "solarized-light"
  | "cyberpunk"
  | "forest"
  | "tokyo-night"
  | "oled-black"
  | "synthwave"
export interface Theme {
  id: ThemeId
  name: string
  description: string
  isDark: boolean
  /** Accent hex for theme preview swatch. */
  accentHex: string
}

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
  {
    id: "rose-pine",
    name: "Rosé Pine",
    description: "Warm, soft, and dusty dark",
    isDark: true,
    accentHex: "#c4a7e7",
  },
  {
    id: "solarized-light",
    name: "Solarized Light",
    description: "Warm reading-friendly light",
    isDark: false,
    accentHex: "#268bd2",
  },
  {
    id: "cyberpunk",
    name: "Cyberpunk",
    description: "High contrast neon nights",
    isDark: true,
    accentHex: "#fdf500",
  },
  {
    id: "forest",
    name: "Forest",
    description: "Calm deep green nature",
    isDark: true,
    accentHex: "#4ade80",
  },
  {
    id: "tokyo-night",
    name: "Tokyo Night",
    description: "Lights of downtown Tokyo",
    isDark: true,
    accentHex: "#7aa2f7",
  },
  {
    id: "oled-black",
    name: "OLED Black",
    description: "Pure black for maximum contrast",
    isDark: true,
    accentHex: "#ffffff",
  },
  {
    id: "synthwave",
    name: "Synthwave",
    description: "Neon 80s retro-futurism",
    isDark: true,
    accentHex: "#ff7edb",
  },
]

export const defaultTheme: ThemeId = "midnight"
