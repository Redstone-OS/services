# Init - Redstone OS

Processo init (PID 1) do Redstone OS.

## Funcionalidade

- ✅ Mostra mensagem de boas-vindas
- ✅ Primeiro processo userspace
- ⏳ Inicializar outros serviços (TODO)
- ⏳ Gerenciar processos filhos (TODO)

## Mensagem de Boas-Vindas

```
╔════════════════════════════════════════╗
║   Bem-vindo ao Redstone OS v0.3.5!   ║
╚════════════════════════════════════════╝

Sistema inicializado com sucesso!

Init (PID 1) executando...

> _
```

## Compilação

```bash
cargo build --release --target x86_64-unknown-none
```

O binário será gerado em:
```
target/x86_64-unknown-none/release/init
```

## Instalação no InitFS

Copiar para:
```
dist/initfs/bin/init
```

## TODO

- [ ] Inicializar serviços (logind, networkd)
- [ ] Gerenciar processos filhos
- [ ] Reboot/shutdown
- [ ] Logs do sistema
