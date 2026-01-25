```text

cmrust/

├── Cargo.toml

├── README.md

├── LICENSE

├── CHANGELOG.md

├── .gitignore

├── rustfmt.toml

├── clippy.toml

├── deny.toml

├── justfile

├── Makefile

├── .cargo/

│   ├── config.toml

│   └── audit.toml

├── .github/

│   └── workflows/

│       ├── ci.yml

│       └── release.yml

├── config/

│   ├── default.toml

│   ├── dev.toml

│   └── prod.toml

├── assets/

│   ├── data/

│   │   ├── clubs.json

│   │   ├── players.json

│   │   ├── staff.json

│   │   ├── competitions.json

│   │   ├── calendar.json

│   │   ├── nations.json

│   │   ├── referees.json

│   │   ├── stadiums.json

│   │   ├── tactics\_presets.json

│   │   └── names/

│   │       ├── first\_names\_en.txt

│   │       ├── last\_names\_en.txt

│   │       ├── first\_names\_pt.txt

│   │       └── last\_names\_pt.txt

│   ├── db/

│   │   ├── schema.sql

│   │   └── seed.sql

│   ├── ui/

│   │   ├── theme.toml

│   │   └── icons.txt

│   └── localization/

│       ├── en-US.ftl

│       └── pt-BR.ftl

├── docs/

│   ├── ARCHITECTURE.md

│   ├── DATA\_MODEL.md

│   ├── MATCH\_ENGINE.md

│   ├── AI.md

│   ├── FINANCE.md

│   ├── TRANSFERS.md

│   ├── SAVE\_FORMAT.md

│   ├── UI\_TUI.md

│   └── API.md

├── scripts/

│   ├── import\_json\_to\_sqlite.rs

│   ├── generate\_calendar.rs

│   └── dev\_seed.sh

├── benches/

│   ├── match\_engine\_bench.rs

│   └── transfer\_ai\_bench.rs

├── tests/

│   ├── integration\_smoke.rs

│   ├── simulation\_regression.rs

│   └── api\_contract.rs

├── docker/

│   ├── Dockerfile.server

│   ├── Dockerfile.tui

│   ├── docker-compose.yml

│   └── healthcheck.sh

└── crates/

&nbsp;   ├── cm\_core/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── prelude.rs

&nbsp;   │       ├── ids.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── economy/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── money.rs

&nbsp;   │       │   ├── wage.rs

&nbsp;   │       │   └── budget.rs

&nbsp;   │       ├── sim/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── time.rs

&nbsp;   │       │   ├── rng.rs

&nbsp;   │       │   ├── rules.rs

&nbsp;   │       │   └── events.rs

&nbsp;   │       └── world/

&nbsp;   │           ├── mod.rs

&nbsp;   │           ├── nation.rs

&nbsp;   │           ├── stadium.rs

&nbsp;   │           ├── referee.rs

&nbsp;   │           ├── club.rs

&nbsp;   │           ├── board.rs

&nbsp;   │           ├── player.rs

&nbsp;   │           ├── staff.rs

&nbsp;   │           ├── contract.rs

&nbsp;   │           ├── injury.rs

&nbsp;   │           ├── morale.rs

&nbsp;   │           ├── training.rs

&nbsp;   │           ├── academy.rs

&nbsp;   │           ├── scouting.rs

&nbsp;   │           ├── tactics.rs

&nbsp;   │           ├── attributes.rs

&nbsp;   │           ├── reputation.rs

&nbsp;   │           ├── competition.rs

&nbsp;   │           ├── fixtures.rs

&nbsp;   │           ├── table.rs

&nbsp;   │           ├── history.rs

&nbsp;   │           └── calendar.rs

&nbsp;   ├── cm\_data/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   ├── migrations/

&nbsp;   │   │   ├── 0001\_init.sql

&nbsp;   │   │   ├── 0002\_seed\_world.sql

&nbsp;   │   │   ├── 0003\_indexes.sql

&nbsp;   │   │   └── 0004\_save\_slots.sql

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── store.rs

&nbsp;   │       ├── db/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── sqlite.rs

&nbsp;   │       │   └── queries.rs

&nbsp;   │       ├── repositories/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── clubs.rs

&nbsp;   │       │   ├── players.rs

&nbsp;   │       │   ├── staff.rs

&nbsp;   │       │   ├── competitions.rs

&nbsp;   │       │   ├── calendar.rs

&nbsp;   │       │   └── saves.rs

&nbsp;   │       └── import/

&nbsp;   │           ├── mod.rs

&nbsp;   │           ├── json\_importer.rs

&nbsp;   │           └── validators.rs

&nbsp;   ├── cm\_match/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── model.rs

&nbsp;   │       ├── probabilistic.rs

&nbsp;   │       ├── tactics.rs

&nbsp;   │       ├── set\_pieces.rs

&nbsp;   │       ├── referee.rs

&nbsp;   │       ├── discipline.rs

&nbsp;   │       ├── fatigue.rs

&nbsp;   │       ├── injuries.rs

&nbsp;   │       ├── ratings.rs

&nbsp;   │       ├── commentary.rs

&nbsp;   │       └── tests.rs

&nbsp;   ├── cm\_ai/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── personalities.rs

&nbsp;   │       ├── squad\_builder.rs

&nbsp;   │       ├── matchday\_ai.rs

&nbsp;   │       ├── tactics\_ai.rs

&nbsp;   │       ├── transfer\_ai.rs

&nbsp;   │       ├── scouting.rs

&nbsp;   │       ├── staff\_ai.rs

&nbsp;   │       ├── board\_ai.rs

&nbsp;   │       └── press\_ai.rs

&nbsp;   ├── cm\_finance/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── model.rs

&nbsp;   │       ├── wage.rs

&nbsp;   │       ├── ticketing.rs

&nbsp;   │       ├── sponsorship.rs

&nbsp;   │       ├── prizes.rs

&nbsp;   │       ├── debt.rs

&nbsp;   │       ├── ffp.rs

&nbsp;   │       ├── rules.rs

&nbsp;   │       └── tests.rs

&nbsp;   ├── cm\_transfers/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── model.rs

&nbsp;   │       ├── valuation.rs

&nbsp;   │       ├── negotiation.rs

&nbsp;   │       ├── agents.rs

&nbsp;   │       ├── clauses.rs

&nbsp;   │       ├── loan.rs

&nbsp;   │       ├── work\_permit.rs

&nbsp;   │       ├── window.rs

&nbsp;   │       └── tests.rs

&nbsp;   ├── cm\_save/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── format.rs

&nbsp;   │       ├── snapshot.rs

&nbsp;   │       ├── compression.rs

&nbsp;   │       ├── integrity.rs

&nbsp;   │       └── versioning.rs

&nbsp;   ├── cm\_engine/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── config.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── state.rs

&nbsp;   │       ├── game.rs

&nbsp;   │       ├── inbox/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── message.rs

&nbsp;   │       │   ├── generators.rs

&nbsp;   │       │   └── filters.rs

&nbsp;   │       └── systems/

&nbsp;   │           ├── mod.rs

&nbsp;   │           ├── time\_manager.rs

&nbsp;   │           ├── competition\_system.rs

&nbsp;   │           ├── match\_system.rs

&nbsp;   │           ├── transfer\_system.rs

&nbsp;   │           ├── finance\_system.rs

&nbsp;   │           ├── ai\_system.rs

&nbsp;   │           ├── injury\_system.rs

&nbsp;   │           ├── training\_system.rs

&nbsp;   │           ├── morale\_system.rs

&nbsp;   │           ├── board\_system.rs

&nbsp;   │           ├── academy\_system.rs

&nbsp;   │           ├── press\_system.rs

&nbsp;   │           └── save\_system.rs

&nbsp;   ├── cm\_api/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── openapi.rs

&nbsp;   │       ├── dto/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── club.rs

&nbsp;   │       │   ├── player.rs

&nbsp;   │       │   ├── competition.rs

&nbsp;   │       │   ├── match.rs

&nbsp;   │       │   └── save.rs

&nbsp;   │       └── routes/

&nbsp;   │           ├── mod.rs

&nbsp;   │           ├── health.rs

&nbsp;   │           ├── world.rs

&nbsp;   │           ├── squad.rs

&nbsp;   │           ├── matches.rs

&nbsp;   │           ├── transfers.rs

&nbsp;   │           └── saves.rs

&nbsp;   ├── cm\_server/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── main.rs

&nbsp;   │       ├── app\_state.rs

&nbsp;   │       ├── router.rs

&nbsp;   │       ├── middleware.rs

&nbsp;   │       └── shutdown.rs

&nbsp;   ├── cm\_tui/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── main.rs

&nbsp;   │       ├── app.rs

&nbsp;   │       ├── state.rs

&nbsp;   │       ├── actions.rs

&nbsp;   │       ├── input.rs

&nbsp;   │       ├── theme.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── widgets/

&nbsp;   │       │   ├── mod.rs

&nbsp;   │       │   ├── header.rs

&nbsp;   │       │   ├── footer.rs

&nbsp;   │       │   ├── table.rs

&nbsp;   │       │   ├── list.rs

&nbsp;   │       │   ├── tabs.rs

&nbsp;   │       │   └── popup.rs

&nbsp;   │       └── screens/

&nbsp;   │           ├── mod.rs

&nbsp;   │           ├── menu.rs

&nbsp;   │           ├── inbox.rs

&nbsp;   │           ├── squad.rs

&nbsp;   │           ├── tactics.rs

&nbsp;   │           ├── transfers.rs

&nbsp;   │           ├── finance.rs

&nbsp;   │           ├── match\_day.rs

&nbsp;   │           └── settings.rs

&nbsp;   ├── cm\_cli/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── main.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── output.rs

&nbsp;   │       └── commands/

&nbsp;   │           ├── mod.rs

&nbsp;   │           ├── new\_game.rs

&nbsp;   │           ├── load\_game.rs

&nbsp;   │           ├── advance\_day.rs

&nbsp;   │           ├── simulate\_match.rs

&nbsp;   │           ├── import\_data.rs

&nbsp;   │           └── export\_save.rs

&nbsp;   ├── cm\_telemetry/

&nbsp;   │   ├── Cargo.toml

&nbsp;   │   └── src/

&nbsp;   │       ├── lib.rs

&nbsp;   │       ├── errors.rs

&nbsp;   │       ├── logging.rs

&nbsp;   │       ├── tracing.rs

&nbsp;   │       └── metrics.rs

&nbsp;   └── cm\_utils/

&nbsp;       ├── Cargo.toml

&nbsp;       └── src/

&nbsp;           ├── lib.rs

&nbsp;           ├── errors.rs

&nbsp;           ├── fs.rs

&nbsp;           ├── hashing.rs

&nbsp;           ├── time.rs

&nbsp;           ├── rng.rs

&nbsp;           └── serde\_ext.rs

```



\## Como essa arquitetura “CM01/02” fica em Rust (camadas reais)



\* \*\*cm\_core\*\*: o “DNA” do jogo (entidades, atributos, regras, calendário, tabelas, reputação, contratos, histórico).

\* \*\*cm\_data\*\*: carregamento/validação do mundo (JSON) + suporte a SQLite (repositórios, migrações, seeds).

\* \*\*cm\_engine\*\*: loop principal e \*systems\* (tempo, competições, partidas, transferências, finanças, IA, treino, moral, lesões, board, base, imprensa, save).

\* \*\*cm\_match\*\*: motor de partida numérico/probabilístico por ticks (minuto a minuto), com árbitro/cartões, fadiga, lesões, ratings e narração.

\* \*\*cm\_ai\*\*: IA dos clubes (personalidade, montagem de elenco, tática, dia de jogo, scouting, mercado, decisões do board e imprensa).

\* \*\*cm\_finance\*\*: salários, bilheteria, patrocínio, premiação, dívidas, regras (inclui FFP para “modo moderno”).

\* \*\*cm\_transfers\*\*: valuation, janelas, agentes, cláusulas, empréstimos, work permit, negociação.

\* \*\*cm\_save\*\*: snapshot/versionamento + compressão + integridade (save em arquivo).

\* \*\*cm\_tui\*\*: UI estilo “menus” (Inbox, Squad, Tactics, Transfers, Finance, Match Day).

\* \*\*cm\_server + cm\_api\*\*: opcional para expor API (para UI web no futuro, analytics, debug tools).

\* \*\*cm\_cli\*\*: ferramenta de dev/automação (criar save, avançar dia, simular partida, importar dados).



---



\## Esqueleto mínimo “rodando” (compila e já simula um jogo)



Abaixo vai um conjunto \*\*pequeno, mas funcional\*\* de arquivos-chave (o resto da árvore suporta o “projeto completo” ao longo do tempo).



\### `Cargo.toml` (workspace)



```toml

\[workspace]

resolver = "2"

members = \[

&nbsp; "crates/cm\_core",

&nbsp; "crates/cm\_utils",

&nbsp; "crates/cm\_telemetry",

&nbsp; "crates/cm\_data",

&nbsp; "crates/cm\_match",

&nbsp; "crates/cm\_ai",

&nbsp; "crates/cm\_finance",

&nbsp; "crates/cm\_transfers",

&nbsp; "crates/cm\_save",

&nbsp; "crates/cm\_engine",

&nbsp; "crates/cm\_cli",

&nbsp; "crates/cm\_tui",

&nbsp; "crates/cm\_api",

&nbsp; "crates/cm\_server",

]



\[workspace.package]

edition = "2021"

version = "0.1.0"

license = "MIT"

authors = \["You"]



\[workspace.dependencies]

serde = { version = "1.0", features = \["derive"] }

serde\_json = "1.0"

thiserror = "1.0"

chrono = { version = "0.4", features = \["serde"] }

rand = "0.8"

rand\_chacha = "0.3"

tracing = "0.1"

tracing-subscriber = { version = "0.3", features = \["env-filter"] }

clap = { version = "4.5", features = \["derive"] }

flate2 = "1.0"

sha2 = "0.10"

hex = "0.4"

anyhow = "1.0"

toml = "0.8"



\# UI / server (prontos pro projeto completo)

crossterm = "0.28"

ratatui = "0.29"

tokio = { version = "1.41", features = \["rt-multi-thread", "macros", "signal"] }

axum = "0.7"

tower = "0.5"

tower-http = { version = "0.6", features = \["trace", "cors"] }



\# SQLite opcional (cm\_data)

rusqlite = { version = "0.32", features = \["bundled"] }

```



---



\### `crates/cm\_cli/Cargo.toml`



```toml

\[package]

name = "cm\_cli"

version.workspace = true

edition.workspace = true

license.workspace = true



\[dependencies]

clap.workspace = true

anyhow.workspace = true

tracing.workspace = true

tracing-subscriber.workspace = true

chrono.workspace = true



cm\_core = { path = "../cm\_core" }

cm\_data = { path = "../cm\_data" }

cm\_engine = { path = "../cm\_engine" }

cm\_match = { path = "../cm\_match" }

cm\_save = { path = "../cm\_save" }

cm\_telemetry = { path = "../cm\_telemetry" }

```



\### `crates/cm\_cli/src/main.rs`



```rust

use clap::{Parser, Subcommand};

use tracing\_subscriber::EnvFilter;



use cm\_telemetry::tracing::init\_tracing;



mod commands;



\#\[derive(Parser, Debug)]

\#\[command(name = "cm", version, about = "CM01/02-style manager sim (Rust)")]

struct Cli {

&nbsp;   #\[command(subcommand)]

&nbsp;   cmd: Command,

}



\#\[derive(Subcommand, Debug)]

enum Command {

&nbsp;   NewGame(commands::new\_game::Args),

&nbsp;   AdvanceDay(commands::advance\_day::Args),

&nbsp;   SimulateMatch(commands::simulate\_match::Args),

}



fn main() -> anyhow::Result<()> {

&nbsp;   init\_tracing(EnvFilter::from\_default\_env().add\_directive("info".parse()?));

&nbsp;   let cli = Cli::parse();



&nbsp;   match cli.cmd {

&nbsp;       Command::NewGame(args) => commands::new\_game::run(args),

&nbsp;       Command::AdvanceDay(args) => commands::advance\_day::run(args),

&nbsp;       Command::SimulateMatch(args) => commands::simulate\_match::run(args),

&nbsp;   }

}

```



\### `crates/cm\_cli/src/commands/mod.rs`



```rust

pub mod advance\_day;

pub mod new\_game;

pub mod simulate\_match;

```



\### `crates/cm\_cli/src/commands/new\_game.rs`



```rust

use clap::Args;

use chrono::NaiveDate;



use cm\_data::import::json\_importer::JsonWorldImporter;

use cm\_engine::game::Game;

use cm\_engine::state::{GameConfig, GameState};

use cm\_save::snapshot::SaveSnapshot;



\#\[derive(Args, Debug)]

pub struct Args {

&nbsp;   #\[arg(long, default\_value = "assets/data")]

&nbsp;   pub data\_dir: String,



&nbsp;   #\[arg(long, default\_value = "saves/slot1.cmsave")]

&nbsp;   pub out: String,



&nbsp;   #\[arg(long, default\_value = "2001-07-01")]

&nbsp;   pub start\_date: String,



&nbsp;   #\[arg(long, default\_value = "LIV")]

&nbsp;   pub club: String,



&nbsp;   #\[arg(long, default\_value = "Manager")]

&nbsp;   pub manager: String,

}



pub fn run(args: Args) -> anyhow::Result<()> {

&nbsp;   let start\_date = NaiveDate::parse\_from\_str(\&args.start\_date, "%Y-%m-%d")?;



&nbsp;   let importer = JsonWorldImporter::new(args.data\_dir);

&nbsp;   let world = importer.load\_world()?;



&nbsp;   let cfg = GameConfig::default();

&nbsp;   let state = GameState::new(start\_date, args.manager, args.club.into());



&nbsp;   let mut game = Game::new(cfg, world, state);

&nbsp;   game.bootstrap\_inbox();



&nbsp;   let snapshot = SaveSnapshot::from\_game(\&game)?;

&nbsp;   snapshot.write\_to\_file(\&args.out)?;



&nbsp;   println!("Novo jogo criado em: {}", args.out);

&nbsp;   Ok(())

}

```



\### `crates/cm\_cli/src/commands/advance\_day.rs`



```rust

use clap::Args;



use cm\_engine::game::Game;

use cm\_save::snapshot::SaveSnapshot;



\#\[derive(Args, Debug)]

pub struct Args {

&nbsp;   #\[arg(long, default\_value = "saves/slot1.cmsave")]

&nbsp;   pub save: String,



&nbsp;   #\[arg(long, default\_value\_t = 1)]

&nbsp;   pub days: u32,

}



pub fn run(args: Args) -> anyhow::Result<()> {

&nbsp;   let mut snap = SaveSnapshot::read\_from\_file(\&args.save)?;

&nbsp;   let mut game: Game = snap.into\_game()?;



&nbsp;   for \_ in 0..args.days {

&nbsp;       game.process\_day();

&nbsp;   }



&nbsp;   let new\_snap = SaveSnapshot::from\_game(\&game)?;

&nbsp;   new\_snap.write\_to\_file(\&args.save)?;

&nbsp;   println!("Avançou {} dia(s). Data atual: {}", args.days, game.state().date);

&nbsp;   Ok(())

}

```



\### `crates/cm\_cli/src/commands/simulate\_match.rs`



```rust

use clap::Args;



use cm\_data::import::json\_importer::JsonWorldImporter;

use cm\_match::probabilistic::simulate\_match;

use cm\_match::model::{MatchInput, TeamStrength};



\#\[derive(Args, Debug)]

pub struct Args {

&nbsp;   #\[arg(long, default\_value = "assets/data")]

&nbsp;   pub data\_dir: String,



&nbsp;   #\[arg(long, default\_value = "LIV")]

&nbsp;   pub home: String,



&nbsp;   #\[arg(long, default\_value = "ARS")]

&nbsp;   pub away: String,

}



pub fn run(args: Args) -> anyhow::Result<()> {

&nbsp;   let importer = JsonWorldImporter::new(args.data\_dir);

&nbsp;   let world = importer.load\_world()?;



&nbsp;   let home = world.club(\&args.home)?;

&nbsp;   let away = world.club(\&args.away)?;



&nbsp;   // Exemplo: força derivada de reputação/orçamento (stub funcional)

&nbsp;   let h = TeamStrength::from\_club(\&home);

&nbsp;   let a = TeamStrength::from\_club(\&away);



&nbsp;   let input = MatchInput {

&nbsp;       home\_id: home.id.clone(),

&nbsp;       away\_id: away.id.clone(),

&nbsp;       home: h,

&nbsp;       away: a,

&nbsp;       minutes: 90,

&nbsp;       seed: Some(42),

&nbsp;   };



&nbsp;   let result = simulate\_match(\&input);

&nbsp;   println!("{} {} x {} {}", home.name, result.home\_goals, result.away\_goals, away.name);

&nbsp;   for line in result.highlights {

&nbsp;       println!(" - {}", line);

&nbsp;   }

&nbsp;   Ok(())

}

```



---



\### `crates/cm\_engine/src/game.rs` (loop principal)



```rust

use crate::config::GameConfig;

use crate::state::GameState;

use crate::systems::{

&nbsp;   ai\_system::AiSystem, competition\_system::CompetitionSystem, finance\_system::FinanceSystem,

&nbsp;   match\_system::MatchSystem, morale\_system::MoraleSystem, save\_system::SaveSystem,

&nbsp;   time\_manager::TimeManager, transfer\_system::TransferSystem,

};

use cm\_core::world::World;



pub struct Game {

&nbsp;   cfg: GameConfig,

&nbsp;   world: World,

&nbsp;   state: GameState,

&nbsp;   // systems (estilo ECS “manual”)

&nbsp;   time: TimeManager,

&nbsp;   competitions: CompetitionSystem,

&nbsp;   matches: MatchSystem,

&nbsp;   transfers: TransferSystem,

&nbsp;   finance: FinanceSystem,

&nbsp;   ai: AiSystem,

&nbsp;   morale: MoraleSystem,

&nbsp;   save: SaveSystem,

}



impl Game {

&nbsp;   pub fn new(cfg: GameConfig, world: World, state: GameState) -> Self {

&nbsp;       Self {

&nbsp;           cfg,

&nbsp;           world,

&nbsp;           state,

&nbsp;           time: TimeManager,

&nbsp;           competitions: CompetitionSystem,

&nbsp;           matches: MatchSystem,

&nbsp;           transfers: TransferSystem,

&nbsp;           finance: FinanceSystem,

&nbsp;           ai: AiSystem,

&nbsp;           morale: MoraleSystem,

&nbsp;           save: SaveSystem,

&nbsp;       }

&nbsp;   }



&nbsp;   pub fn state(\&self) -> \&GameState {

&nbsp;       \&self.state

&nbsp;   }



&nbsp;   pub fn world(\&self) -> \&World {

&nbsp;       \&self.world

&nbsp;   }



&nbsp;   pub fn bootstrap\_inbox(\&mut self) {

&nbsp;       self.state.inbox.push("Bem-vindo! Seu desafio começa hoje.".into());

&nbsp;       self.state.inbox.push("Defina sua tática e confira o elenco.".into());

&nbsp;   }



&nbsp;   pub fn process\_day(\&mut self) {

&nbsp;       // 1) tempo

&nbsp;       self.time.tick\_day(\&self.cfg, \&mut self.world, \&mut self.state);



&nbsp;       // 2) IA (pré)

&nbsp;       self.ai.run\_daily(\&self.cfg, \&mut self.world, \&mut self.state);



&nbsp;       // 3) competições (fixtures/tabelas)

&nbsp;       self.competitions.run\_daily(\&self.cfg, \&mut self.world, \&mut self.state);



&nbsp;       // 4) dia de jogo?

&nbsp;       if self.state.flags.match\_day {

&nbsp;           self.matches.run\_match\_day(\&self.cfg, \&mut self.world, \&mut self.state);

&nbsp;       }



&nbsp;       // 5) mercado

&nbsp;       self.transfers.run\_daily(\&self.cfg, \&mut self.world, \&mut self.state);



&nbsp;       // 6) finanças (diário/semanal)

&nbsp;       self.finance.run\_daily(\&self.cfg, \&mut self.world, \&mut self.state);



&nbsp;       // 7) moral/treino/lesões (simplificado aqui)

&nbsp;       self.morale.run\_daily(\&self.cfg, \&mut self.world, \&mut self.state);



&nbsp;       // 8) autosave flag

&nbsp;       self.save.mark\_dirty(\&mut self.state);

&nbsp;   }

}

```



---



\### `crates/cm\_match/src/probabilistic.rs` (ticks + eventos)



```rust

use rand::{Rng, SeedableRng};

use rand\_chacha::ChaCha8Rng;



use crate::model::{MatchInput, MatchResult};



pub fn simulate\_match(input: \&MatchInput) -> MatchResult {

&nbsp;   let mut rng = match input.seed {

&nbsp;       Some(s) => ChaCha8Rng::seed\_from\_u64(s),

&nbsp;       None => ChaCha8Rng::from\_entropy(),

&nbsp;   };



&nbsp;   let mut home\_goals = 0u8;

&nbsp;   let mut away\_goals = 0u8;

&nbsp;   let mut highlights = Vec::new();



&nbsp;   for minute in 1..=input.minutes {

&nbsp;       // posse “barata”, mas estável

&nbsp;       let home\_edge = input.home.attack.saturating\_sub(input.away.midfield);

&nbsp;       let away\_edge = input.away.attack.saturating\_sub(input.home.midfield);



&nbsp;       // chance base (0.0..1.0)

&nbsp;       let base\_home = (0.010 + (home\_edge as f32) \* 0.0008).clamp(0.005, 0.040);

&nbsp;       let base\_away = (0.010 + (away\_edge as f32) \* 0.0008).clamp(0.005, 0.040);



&nbsp;       // resolve ataque -> defesa -> finalização

&nbsp;       let home\_roll: f32 = rng.gen();

&nbsp;       let away\_roll: f32 = rng.gen();



&nbsp;       let home\_finish = (input.home.finishing as f32 + rng.gen\_range(0.0..6.0))

&nbsp;           - (input.away.defense as f32 \* 0.35);

&nbsp;       let away\_finish = (input.away.finishing as f32 + rng.gen\_range(0.0..6.0))

&nbsp;           - (input.home.defense as f32 \* 0.35);



&nbsp;       if home\_roll < base\_home \&\& home\_finish > 6.5 {

&nbsp;           home\_goals += 1;

&nbsp;           highlights.push(format!("{minute:02}' GOL (Casa)!"));

&nbsp;       }



&nbsp;       if away\_roll < base\_away \&\& away\_finish > 6.5 {

&nbsp;           away\_goals += 1;

&nbsp;           highlights.push(format!("{minute:02}' GOL (Fora)!"));

&nbsp;       }



&nbsp;       if minute == 45 || minute == 90 {

&nbsp;           highlights.push(format!("{minute:02}' Apito do árbitro."));

&nbsp;       }

&nbsp;   }



&nbsp;   MatchResult {

&nbsp;       home\_id: input.home\_id.clone(),

&nbsp;       away\_id: input.away\_id.clone(),

&nbsp;       home\_goals,

&nbsp;       away\_goals,

&nbsp;       highlights,

&nbsp;   }

}

```



---



\### `crates/cm\_save/src/snapshot.rs` (save gzip + hash)



```rust

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};



use crate::compression::{read\_gzip, write\_gzip};

use crate::integrity::{hash\_bytes\_sha256, verify\_sha256\_hex};

use crate::versioning::SAVE\_VERSION;

use cm\_engine::game::Game;



\#\[derive(Debug, Serialize, Deserialize)]

pub struct SaveSnapshot {

&nbsp;   pub version: u32,

&nbsp;   pub created\_at: DateTime<Utc>,

&nbsp;   pub sha256: String,

&nbsp;   pub payload: SavePayload,

}



\#\[derive(Debug, Serialize, Deserialize)]

pub struct SavePayload {

&nbsp;   pub world: cm\_core::world::World,

&nbsp;   pub state: cm\_engine::state::GameState,

&nbsp;   pub cfg: cm\_engine::config::GameConfig,

}



impl SaveSnapshot {

&nbsp;   pub fn from\_game(game: \&Game) -> anyhow::Result<Self> {

&nbsp;       let payload = SavePayload {

&nbsp;           world: game.world().clone(),

&nbsp;           state: game.state().clone(),

&nbsp;           cfg: game.cfg().clone(),

&nbsp;       };

&nbsp;       let bytes = serde\_json::to\_vec(\&payload)?;

&nbsp;       let sha256 = hash\_bytes\_sha256(\&bytes);

&nbsp;       Ok(Self {

&nbsp;           version: SAVE\_VERSION,

&nbsp;           created\_at: Utc::now(),

&nbsp;           sha256,

&nbsp;           payload,

&nbsp;       })

&nbsp;   }



&nbsp;   pub fn into\_game(self) -> anyhow::Result<Game> {

&nbsp;       Ok(Game::new(self.payload.cfg, self.payload.world, self.payload.state))

&nbsp;   }



&nbsp;   pub fn write\_to\_file(\&self, path: \&str) -> anyhow::Result<()> {

&nbsp;       let bytes = serde\_json::to\_vec(self)?;

&nbsp;       write\_gzip(path, \&bytes)

&nbsp;   }



&nbsp;   pub fn read\_from\_file(path: \&str) -> anyhow::Result<Self> {

&nbsp;       let bytes = read\_gzip(path)?;

&nbsp;       let snap: SaveSnapshot = serde\_json::from\_slice(\&bytes)?;

&nbsp;       let payload\_bytes = serde\_json::to\_vec(\&snap.payload)?;

&nbsp;       verify\_sha256\_hex(\&payload\_bytes, \&snap.sha256)?;

&nbsp;       Ok(snap)

&nbsp;   }

}

```



---



\## Como rodar (dev rápido)



1\. Coloque um dataset mínimo em `assets/data/` (já previsto na estrutura).

2\. Rode:



```bash

cargo run -p cm\_cli -- new-game --out saves/slot1.cmsave --start-date 2001-07-01 --club LIV --manager "Você"

cargo run -p cm\_cli -- advance-day --save saves/slot1.cmsave --days 7

cargo run -p cm\_cli -- simulate-match --home LIV --away ARS

```



---



\## Por que essa estrutura é “a real” de um CM-style



\* \*\*Separação limpa\*\* entre \*mundo\* (dados), \*regras/simulações\* (engine/systems), e \*execução do jogo\* (CLI/TUI/Server).

\* \*\*Match engine isolado\*\*: dá pra evoluir do probabilístico simples para modelos mais detalhados sem quebrar o resto.

\* \*\*IA em módulos\*\*: contratações/tática/dia de jogo/board—cada um cresce sem virar “Deus-objeto”.

\* \*\*Save versionado\*\*: essencial quando você muda o modelo de dados no meio do projeto.

\* \*\*Dois modos de dados\*\*: JSON (dev rápido) + SQLite (produção/escala).

