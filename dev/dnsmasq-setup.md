# Wildcard DNS local com dnsmasq (opcional)

Sem isso: você precisa adicionar uma linha no `/etc/hosts` por slug de tenant.  
Com isso: `*.nexus.localhost` resolve automaticamente para 127.0.0.1 — qualquer slug funciona sem configuração extra.

## Manjaro / Arch

NetworkManager já inclui dnsmasq. São dois passos:

### 1. Habilitar dnsmasq no NetworkManager

Crie o arquivo `/etc/NetworkManager/conf.d/dnsmasq.conf`:
```ini
[main]
dns=dnsmasq
```

### 2. Adicionar a regra wildcard

Crie `/etc/NetworkManager/dnsmasq.d/nexus-local.conf`:
```
address=/nexus.localhost/127.0.0.1
```

### 3. Reiniciar

```bash
sudo systemctl restart NetworkManager
```

Pronto. `acme.nexus.localhost`, `demo.nexus.localhost`, qualquer subdomínio — tudo resolve para 127.0.0.1 automaticamente.

> **Nota:** com dnsmasq ativo você pode remover as entradas de `nexus.localhost` do `/etc/hosts` se tiver adicionado antes.
