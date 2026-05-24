const SEG = {
  login: "login",
  app: "app",
  tenant: "tenant",
  knowledge: "knowledge",
} as const

export const segments = SEG

export const paths = {
  home: "/",
  login: `/${SEG.login}`,
  app: {
    root: `/${SEG.app}`,
    tenant: `/${SEG.app}/${SEG.tenant}`,
    knowledge: `/${SEG.app}/${SEG.knowledge}`,
  },
} as const

export type AppPaths = typeof paths
