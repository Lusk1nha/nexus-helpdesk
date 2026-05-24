# Dev — Subdomain setup local

Por padrão, os apps rodam em **portas separadas** (`localhost:5173/5174/5175`) e
não precisam de nenhuma configuração extra. Isso funciona para desenvolver cada
app de forma isolada.

Se você quiser testar o comportamento **real de subdomínios** (cookie
compartilhado, URLs por tenant), siga os passos abaixo.

---

## Modo portas (padrão, sem configuração)

| App        | URL                     |
| ---------- | ----------------------- |
| web        | `http://localhost:5173` |
| onboarding | `http://localhost:5174` |
| admin      | `http://localhost:5175` |
| API        | `http://localhost:8080` |

`.env` do backend:

```
FRONTEND_URL=http://localhost:5173,http://localhost:5174,http://localhost:5175
# COOKIE_DOMAIN não definido
```

**Limitação:** o refresh cookie fica restrito a cada origem. Funciona perfeitamente
para desenvolvimento isolado, mas web e admin não compartilham sessão.

---

## Modo subdomínios (mais próximo da produção)

### 1. Instale o Caddy

```bash
# Arch/Manjaro
sudo pacman -S caddy

# Ubuntu/Debian
sudo apt install caddy

# macOS
brew install caddy
```

### 2. Adicione entradas no `/etc/hosts`

```
127.0.0.1  onboarding.nexus.localhost
127.0.0.1  admin.nexus.localhost
127.0.0.1  acme.nexus.localhost
127.0.0.1  demo.nexus.localhost
```

> Adicione uma linha por slug de tenant que você for testar.  
> **Não precisa de dnsmasq** — só essas entradas fixas já bastam.

### 3. Configure o backend

No arquivo `.env` (raiz do workspace):

```
FRONTEND_URL=.nexus.localhost
COOKIE_DOMAIN=.nexus.localhost
COOKIE_SECURE=false
```

### 4. Suba tudo

```bash
# Terminal 1 — API
cargo run -p api_gateway

# Terminal 2 — Web
pnpm --filter web dev

# Terminal 3 — Onboarding
pnpm --filter onboarding dev

# Terminal 4 — Admin
pnpm --filter admin dev

# Terminal 5 — Caddy (do diretório raiz)
caddy run --config dev/Caddyfile
```

### Resultado

| App               | URL                                 |
| ----------------- | ----------------------------------- |
| web (tenant acme) | `http://acme.nexus.localhost`       |
| web (tenant demo) | `http://demo.nexus.localhost`       |
| onboarding        | `http://onboarding.nexus.localhost` |
| admin             | `http://admin.nexus.localhost`      |

O cookie `nexus_refresh` fica disponível em **todos** os subdomínios
`.nexus.localhost`, então web e admin compartilham a sessão.

---

## Eliminar as entradas do `/etc/hosts` com dnsmasq

O Caddyfile já usa wildcard (`*.nexus.localhost`) — você **nunca precisa mexer
nele por tenant**. A única coisa que precisa por tenant são as entradas no
`/etc/hosts`, porque esse arquivo não suporta wildcard.

Para resolver isso de vez, veja [dnsmasq-setup.md](dnsmasq-setup.md).  
No Manjaro são dois arquivos e um restart do NetworkManager — nada mais.

---

## Produção

No deploy real (ex: Fly.io, Render, VPS):

1. Configure um **wildcard DNS**: `*.nexus.com → IP do servidor`
2. Use um **wildcard TLS cert** — o Let's Encrypt suporta via DNS challenge:
   ```
   certbot certonly --dns-cloudflare -d "*.nexus.com" -d "nexus.com"
   ```
3. No `.env` de produção:
   ```
   FRONTEND_URL=.nexus.com
   COOKIE_DOMAIN=.nexus.com
   COOKIE_SECURE=true
   ```

Caddy em produção também pode emitir o wildcard cert automaticamente se você
tiver o plugin DNS provider configurado.
