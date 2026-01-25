//! Game rules configuration.

use serde::{Deserialize, Serialize};

/// Game rules and configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRules {
    /// Maximum squad size.
    pub max_squad_size: usize,
    /// Maximum foreign players.
    pub max_foreign_players: usize,
    /// Minimum starting age.
    pub min_player_age: u8,
    /// Retirement age.
    pub retirement_age: u8,
    /// Match duration in minutes.
    pub match_duration: u8,
    /// Extra time duration (each half).
    pub extra_time_duration: u8,
    /// Maximum substitutions per match.
    pub max_substitutions: u8,
    /// Transfer window enabled.
    pub transfer_windows_enabled: bool,
    /// Points for a win.
    pub points_win: u8,
    /// Points for a draw.
    pub points_draw: u8,
    /// Points for a loss.
    pub points_loss: u8,
    /// Enable FFP rules.
    pub ffp_enabled: bool,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            max_squad_size: 25,
            max_foreign_players: 17,
            min_player_age: 16,
            retirement_age: 40,
            match_duration: 90,
            extra_time_duration: 15,
            max_substitutions: 3,
            transfer_windows_enabled: true,
            points_win: 3,
            points_draw: 1,
            points_loss: 0,
            ffp_enabled: false,
        }
    }
}
