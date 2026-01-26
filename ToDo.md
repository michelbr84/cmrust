# ToDo — CM Rust Modern UI (Windows Executable)

Roadmap completa para transformar o jogo em um executável Windows com interface moderna 4K, simulação estilo CM01/02, e sem dependências de mídia externa (apenas emojis/code).

## 1. Estrutura do Projeto e Fluxo de Telas (Máquina de Estados)
- [ ] **Inicialização do Projeto GUI**
  - [ ] Escolher/Configurar framework (Sugestão: Tauri + React/Vanilla JS para UI moderna e fácil estilização).
  - [ ] Configurar janela principal: Resolução resizável, suporte a High DPI/4K.
  - [ ] Configurar ícone e metadados do executável Windows.
- [ ] **Gerenciador de Estados (State Machine)**
  - [ ] Implementar sistema de troca de telas (`CurrentScreen` enum/state).
  - [ ] Telas necessárias: `LanguageSelect`, `StartMenu`, `NewGame`, `Options`, `LoadGame`, `MainGameHub`.

## 2. Tela 1: Seleção de Idioma (25 Bandeiras)
- [ ] **Carregamento de Dados**
  - [ ] Ler `/JSON/flags.json` na inicialização.
  - [ ] Parsear lista de 25 países com `flag` (emoji) e `language.code`.
- [ ] **Renderização (Grid 5x5)**
  - [ ] Criar layout Grid 5x5 centralizado.
  - [ ] Renderizar botões com o emoji da bandeira (tamanho grande) + nome (opcional).
  - [ ] Implementar Hover effects (modernos/glasmorphism).
- [ ] **Lógica**
  - [ ] `OnClick` -> Salvar `selected_language_code`.
  - [ ] Disparar carregamento do idioma (passo 3).
  - [ ] Transição para tela `StartMenu`.

## 3. Sistema de Localização (Fallback)
- [ ] **Carregador de Strings**
  - [ ] Receber `selected_language_code`.
  - [ ] Tentar carregar `/JSON/<code>/start.json` (ex: `pt-BR`).
  - [ ] Fallback: Tentar `/JSON/<base>/start.json` (ex: `pt`).
  - [ ] Default: Usar `en` se falhar.
  - [ ] Armazenar strings em memória para uso na UI.

## 4. Tela 2: Menu Inicial (Start Menu)
- [ ] **Dados**
  - [ ] Usar strings carregadas do `start.json`.
  - [ ] Campos: Título do jogo, Labels dos botões (Iniciar, Continuar, Opções, Sair).
- [ ] **Interface (UI)**
  - [ ] Título grande centralizado ("Futebol Simulador 2026").
  - [ ] 4 Botões grandes verticais.
  - [ ] Estilo "Premium": Bordas suaves, sombras, tipografia limpa, sem imagens de fundo (apenas cores/gradientes).
- [ ] **Ações**
  - [ ] **Iniciar Jogo**: Navegar para `NewGameScreen`.
  - [ ] **Continuar Jogo**: Executar lógica de Load (Passo 5).
  - [ ] **Opções**: Navegar para `OptionsScreen`.
  - [ ] **Sair**: Fechar a aplicação.

## 5. Funcionalidade: Continuar Jogo
- [ ] **Opção A (Simples)**
  - [ ] Verificar existência de save mais recente.
  - [ ] Se existir: Carregar e ir para o jogo.
  - [ ] Se não: Exibir Toast/Modal "Nenhum save encontrado".
- [ ] **Opção B (Interface - Futuro)**
  - [ ] Listar slots de save com metadados.

## 6. Tela 3: Novo Jogo (New Game Form)
- [ ] **Interface**
  - [ ] Formulário de criação de treinador (Nome, Sobrenome).
  - [ ] Seleção de Nacionalidade (Dropdown usando dados do `flags.json`).
  - [ ] Seleção de Clube Inicial (Dados do `clubs.json` ou engine).
- [ ] **Dados**
  - [ ] Carregar `new_game.json` do idioma selecionado.
- [ ] **Ação**
  - [ ] Botão "Confirmar": Inicializar engine rust, criar save inicial, ir para o jogo.

## 7. Tela 4: Opções (Mínimo Viável)
- [ ] **Configurações**
  - [ ] Selector de Idioma (reutilizar lógica de flags ou dropdown).
  - [ ] Modo de Janela (Windowed/Fullscreen).
  - [ ] Escala de UI (1x, 1.5x, 2x - importante para 4K).
- [ ] Persistência de configurações.

## 8. Integração Backend (Rust)
- [ ] Conectar UI (Frontend) com lógica de jogo (`cm_engine`/`cm_core`).
- [ ] Comandos Invoke para: Criar Jogo, Carregar Jogo, Salvar, Avançar Dia.

## 9. Polimento Visual (Look & Feel)
- [ ] Garantir tipografia consistente (System fonts ou Google Fonts modernas).
- [ ] Paleta de cores sóbria e elegante (Dark mode default?).
- [ ] Responsividade do layout.
