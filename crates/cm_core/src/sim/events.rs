//! Game events.

use serde::{Deserialize, Serialize};
use crate::ids::{ClubId, MatchId, PlayerId};

/// Match events during simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEvent {
    Goal {
        minute: u8,
        scorer: PlayerId,
        assist: Option<PlayerId>,
        club: ClubId,
    },
    OwnGoal {
        minute: u8,
        player: PlayerId,
        club: ClubId,
    },
    YellowCard {
        minute: u8,
        player: PlayerId,
    },
    RedCard {
        minute: u8,
        player: PlayerId,
    },
    Substitution {
        minute: u8,
        player_off: PlayerId,
        player_on: PlayerId,
    },
    Injury {
        minute: u8,
        player: PlayerId,
    },
    PenaltyMissed {
        minute: u8,
        taker: PlayerId,
    },
    PenaltyScored {
        minute: u8,
        taker: PlayerId,
    },
    HalfTime,
    FullTime,
    ExtraTimeStart,
    PenaltyShootout,
}

impl MatchEvent {
    /// Get the minute of the event.
    pub fn minute(&self) -> Option<u8> {
        match self {
            Self::Goal { minute, .. } => Some(*minute),
            Self::OwnGoal { minute, .. } => Some(*minute),
            Self::YellowCard { minute, .. } => Some(*minute),
            Self::RedCard { minute, .. } => Some(*minute),
            Self::Substitution { minute, .. } => Some(*minute),
            Self::Injury { minute, .. } => Some(*minute),
            Self::PenaltyMissed { minute, .. } => Some(*minute),
            Self::PenaltyScored { minute, .. } => Some(*minute),
            _ => None,
        }
    }
}

/// General game events (not match-specific).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    MatchPlayed {
        match_id: MatchId,
        home: ClubId,
        away: ClubId,
        home_goals: u8,
        away_goals: u8,
    },
    TransferCompleted {
        player: PlayerId,
        from: ClubId,
        to: ClubId,
    },
    ContractSigned {
        player: PlayerId,
        club: ClubId,
    },
    InjuryUpdate {
        player: PlayerId,
        days_out: u16,
    },
    SeasonEnd {
        season: String,
    },
    NewSeason {
        season: String,
    },
}
