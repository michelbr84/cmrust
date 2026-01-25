//! # CM Engine
//!
//! Game loop orchestration and systems.

pub mod config;
pub mod errors;
pub mod game;
pub mod inbox;
pub mod state;
pub mod systems;

pub use config::GameConfig;
pub use game::Game;
pub use state::GameState;
