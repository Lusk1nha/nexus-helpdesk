// web

import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import tsconfigPaths from "vite-tsconfig-paths"
import { defineConfig } from "vitest/config"

export default defineConfig({
  plugins: [react(), tailwindcss(), tsconfigPaths()],
  server: {
    port: 5173,
    strictPort: true,
    host: "127.0.0.1",
    allowedHosts: [".localhost", ".nexus.test"], // <-- Permite subdomínios wildcard
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./src/test/setup.ts"],
  },
})
