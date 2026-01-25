//! Competition entity.

use serde::{Deserialize, Serialize};
use crate::ids::{CompetitionId, NationId, ClubId};
use super::{Fixtures, Table};

/// Competition type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompetitionType {
    League,
    Cup,
    SuperCup,
    International,
}

impl Default for CompetitionType {
    fn default() -> Self {
        Self::League
    }
}

/// A football competition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: CompetitionId,
    pub name: String,
    pub short_name: String,
    pub nation_id: Option<NationId>,
    pub competition_type: CompetitionType,
    pub reputation: u8,
    pub teams: Vec<ClubId>,
    pub fixtures: Fixtures,
    pub table: Table,
    pub current_round: u8,
    pub total_rounds: u8,
}

impl Competition {
    /// Create a new competition.
    pub fn new(
        id: impl Into<CompetitionId>,
        name: impl Into<String>,
        competition_type: CompetitionType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            short_name: String::new(),
            nation_id: None,
            competition_type,
            reputation: 50,
            teams: Vec::new(),
            fixtures: Fixtures::new(),
            table: Table::new(),
            current_round: 0,
            total_rounds: 0,
        }
    }

    /// Check if league.
    pub fn is_league(&self) -> bool {
        self.competition_type == CompetitionType::League
    }

    /// Check if cup.
    pub fn is_cup(&self) -> bool {
        self.competition_type == CompetitionType::Cup
    }

    /// Add a team.
    pub fn add_team(&mut self, club_id: ClubId) {
        if !self.teams.contains(&club_id) {
            self.teams.push(club_id.clone());
            self.table.add_team(club_id);
        }
    }

    /// Get number of teams.
    pub fn team_count(&self) -> usize {
        self.teams.len()
    }
}
