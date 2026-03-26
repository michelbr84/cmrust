# FutMestre

Jogo de gerenciamento de futebol inspirado nos clássicos **Championship Manager 01/02** e **Elifoot 98**. Construído em Rust com interface desktop via Tauri.

Gerencie seu clube, escale o time, defina táticas, contrate jogadores e leve seu time da quarta divisão ao topo do campeonato.

## Inicio Rapido

```bash
# Compilar o projeto
cargo build --workspace

# Simular uma partida
cargo run -p cm_cli -- simulate-match --home LIV --away ARS --seed 42

# Criar um novo jogo
cargo run -p cm_cli -- new-game --club LIV --manager "Seu Nome"

# Avançar dias
cargo run -p cm_cli -- advance-day --days 7

# Interface de terminal
cargo run -p cm_tui

# Servidor HTTP
cargo run -p cm_server
```

## Pre-requisitos

- [Rust 1.75+](https://www.rust-lang.org/tools/install)
- [Node.js 18+](https://nodejs.org/) (apenas para a GUI Tauri)

## Arquitetura

Workspace Rust com 15 crates seguindo Domain-Driven Design:

```
crates/
├── cm_utils        # Utilitarios (RNG, hashing, tempo, filesystem)
├── cm_telemetry    # Logging, tracing e metricas
├── cm_core         # Modelos de dominio (Jogador, Clube, Competicao, Mundo)
├── cm_data         # Camada de dados (importador JSON, SQLite, repositorios)
├── cm_match        # Motor de partida (simulacao minuto-a-minuto, eventos)
├── cm_ai           # Sistemas de IA (escalacao, transferencias, tatica, diretoria)
├── cm_finance      # Simulacao financeira (salarios, patrocinio, FFP)
├── cm_transfers    # Mercado de transferencias (avaliacao, negociacao, contratos)
├── cm_save         # Sistema de save (gzip + verificacao SHA256)
├── cm_engine       # Game loop com 13+ sistemas (moral, treino, lesoes)
├── cm_cli          # Comandos CLI (novo-jogo, avancar-dia, simular-partida)
├── cm_tui          # Interface de terminal (Ratatui)
├── cm_api          # API REST (DTOs e rotas)
├── cm_server       # Servidor HTTP (Axum)
└── cm_gui          # Interface desktop (Tauri + Glassmorphism 4K)
```

### Dependencias entre Crates

```
cm_utils --> cm_core --> cm_data --> cm_match
                |                       |
                +--> cm_ai <------------+
                +--> cm_finance         |
                +--> cm_transfers       |
                         |              |
                    cm_engine <---------+
                    /   |   \
              cm_cli  cm_tui  cm_api --> cm_server
                              cm_gui
```

## Funcionalidades

### Implementadas
- **Motor de Partida**: Simulacao probabilistica minuto-a-minuto com RNG semeado
- **Modelo de Mundo**: Nacoes, Clubes, Jogadores (40+ atributos), Staff, Competicoes, Estadios
- **IA**: Escalacao automatica, decisoes de transferencia, analise de elenco, coletivas de imprensa
- **Financas**: Salarios semanais, bilheteria, patrocinio por reputacao, premiacao
- **Transferencias**: Propostas, negociacao, avaliacao de jogadores, janelas de transferencia, emprestimos
- **Salvamento**: Saves comprimidos (.cmsave) com verificacao de integridade SHA256
- **CLI**: Comandos para simulacao, criacao de jogo e avanco de dias
- **Game Loop**: Processamento diario com moral, treino, lesoes e financas
- **GUI Desktop**: Interface Tauri com design Glassmorphism e suporte a 4K

### Em Desenvolvimento
- Motor de partida com cartoes, lesoes e substituicoes durante o jogo
- Sistema de 4 divisoes com promocao e rebaixamento
- Copa nacional (mata-mata)
- Treinamento com evolucao de atributos
- Categorias de base
- Dados de clubes brasileiros
- Testes E2E e cobertura > 70%

## Comandos de Desenvolvimento

```bash
# Compilar
cargo build --workspace              # Debug
cargo build --workspace --release    # Release

# Testes
cargo test --workspace               # Todos os testes
cargo test -p cm_match               # Testes de um crate especifico

# Qualidade
cargo fmt --all                      # Formatar codigo
cargo clippy --workspace --all-targets -- -D warnings  # Linting

# Makefile (atalhos)
make all                             # fmt + clippy + test
make ci                              # fmt-check + clippy + test
make run-tui                         # Iniciar TUI
make run-server                      # Iniciar servidor HTTP
make simulate-match                  # Simular partida de exemplo
```

## Estrutura de Pastas

```
FutMestre/
├── Cargo.toml          # Manifesto do workspace
├── Cargo.lock          # Versoes travadas das dependencias
├── Makefile            # Atalhos de desenvolvimento
├── justfile            # Alternativa ao Make (just)
├── rustfmt.toml        # Configuracao de formatacao
├── clippy.toml         # Configuracao de linting
├── deny.toml           # Auditoria de dependencias
├── roadmap.md          # Plano de desenvolvimento
├── CLAUDE.md           # Instrucoes para Claude Code
├── crates/             # Todos os 15 crates Rust
├── assets/
│   └── data/           # Dados do jogo (JSON)
│       ├── clubs.json
│       ├── competitions.json
│       ├── nations.json
│       ├── stadiums.json
│       ├── calendar.json
│       ├── referees.json
│       ├── staff.json
│       └── tactics_presets.json
├── saves/              # Saves do jogador
├── benches/            # Benchmarks de performance
├── tests/              # Testes de integracao
├── scripts/            # Scripts auxiliares
└── docs/               # Documentacao adicional
```

## Modelo de Dados

### Jogador (40+ atributos)
- **Tecnicos**: cruzamento, drible, finalizacao, marcacao, passe, desarme, cabeceio
- **Mentais**: agressividade, antecipacao, compostura, decisao, lideranca, visao
- **Fisicos**: velocidade, resistencia, forca, aceleracao, agilidade
- **Goleiro**: defesa, reflexos, comando de area, chute longo
- **Derivados**: overall_rating() calculado por posicao com pesos

### Clube
- Elenco, staff, orcamento (transferencias + salarios), estadio
- Taticas: formacao, mentalidade, pressao, ritmo
- Reputacao e confianca da diretoria

### Competicao
- Tipos: liga (pontos corridos) e copa (mata-mata)
- Fixtures gerados automaticamente, tabela de classificacao

## Motor de Partida

Simulacao probabilistica por minuto (1-90):
1. Calcula vantagem de ataque: `ataque_casa - meio_campo_fora`
2. Chance base de gol: 0.5% a 4% por minuto (ajustado por forca do time)
3. Roll de RNG para oportunidade de ataque
4. Calculo finalizacao vs defesa
5. Gol se finalizacao > limiar

Suporta RNG semeado para resultados deterministicos e reproduziveis.

## Roadmap

Consulte [roadmap.md](roadmap.md) para o plano completo de desenvolvimento com 13 fases.

**Prioridades atuais:**
1. Motor de partida realista (cartoes, lesoes, substituicoes)
2. Sistema de 4 divisoes com promocao/rebaixamento
3. Interface GUI jogavel completa
4. Dados de clubes brasileiros

## Licenca

MIT License — veja [LICENSE](LICENSE) para detalhes.

---

*Desenvolvido com dedicacao para os fas de jogos de gerenciamento de futebol.*
"# FutMestre" 
"# FutMestre" 
