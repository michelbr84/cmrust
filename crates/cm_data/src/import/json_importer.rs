//! JSON world importer.

use std::collections::HashMap;
use std::path::Path;

use chrono::NaiveDate;
use serde::Deserialize;

use cm_core::economy::{Budget, Money, Wage};
use cm_core::ids::*;
use cm_core::world::*;

use crate::errors::DataError;

/// JSON importer for world data.
pub struct JsonWorldImporter {
    data_dir: String,
}

impl JsonWorldImporter {
    /// Create a new importer.
    pub fn new(data_dir: impl Into<String>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Load the complete world.
    pub fn load_world(&self) -> Result<World, DataError> {
        let mut world = World::new();

        // Load nations
        self.load_nations(&mut world)?;

        // Load clubs
        self.load_clubs(&mut world)?;

        // Load players
        self.load_players(&mut world)?;

        // Load competitions
        self.load_competitions(&mut world)?;

        Ok(world)
    }

    fn load_nations(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("nations.json");
        if !path.exists() {
            // Create default nations
            let nations = vec![
                ("ENG", "England", "Europe"),
                ("ESP", "Spain", "Europe"),
                ("GER", "Germany", "Europe"),
                ("ITA", "Italy", "Europe"),
                ("FRA", "France", "Europe"),
                ("POR", "Portugal", "Europe"),
                ("BRA", "Brazil", "South America"),
                ("ARG", "Argentina", "South America"),
            ];

            for (id, name, continent) in nations {
                let mut nation = Nation::new(id, name);
                nation.continent = continent.to_string();
                nation.reputation = 80;
                world.nations.insert(NationId::new(id), nation);
            }
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawNation> = serde_json::from_str(&content)?;

        for n in raw {
            let nation = Nation {
                id: NationId::new(&n.id),
                name: n.name,
                short_name: n.short_name.unwrap_or_default(),
                continent: n.continent.unwrap_or_default(),
                reputation: n.reputation.unwrap_or(50),
                youth_rating: n.youth_rating.unwrap_or(50),
            };
            world.nations.insert(nation.id.clone(), nation);
        }

        Ok(())
    }

    fn load_clubs(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("clubs.json");
        if !path.exists() {
            // Create default clubs
            let clubs = vec![
                ("LIV", "Liverpool", "ENG", 90, 50_000_000),
                ("ARS", "Arsenal", "ENG", 88, 40_000_000),
                ("MUN", "Manchester United", "ENG", 91, 60_000_000),
                ("CHE", "Chelsea", "ENG", 87, 45_000_000),
                ("NEW", "Newcastle United", "ENG", 75, 15_000_000),
                ("LEE", "Leeds United", "ENG", 78, 20_000_000),
            ];

            for (id, name, nation, rep, budget) in clubs {
                let mut club = Club::new(id, name, NationId::new(nation));
                club.short_name = id.to_string();
                club.reputation = rep;
                club.budget = Budget::new(
                    Money::from_major(budget),
                    Money::from_major(budget / 2),
                    Money::from_major(500_000),
                );
                world.clubs.insert(ClubId::new(id), club);
            }
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawClub> = serde_json::from_str(&content)?;

        for c in raw {
            let budget = Budget::new(
                Money::from_major(c.balance.unwrap_or(1_000_000)),
                Money::from_major(c.transfer_budget.unwrap_or(500_000)),
                Money::from_major(c.wage_budget.unwrap_or(100_000)),
            );

            let club = Club {
                id: ClubId::new(&c.id),
                name: c.name,
                short_name: c.short_name.unwrap_or_default(),
                nation_id: NationId::new(c.nation_id.unwrap_or_default()),
                stadium_id: c.stadium_id.map(StadiumId::new),
                reputation: c.reputation.unwrap_or(50),
                budget,
                board: Board::default(),
                tactics: Tactics::default(),
                player_ids: Vec::new(),
                staff_ids: Vec::new(),
                primary_color: c.primary_color.unwrap_or_else(|| "#FF0000".into()),
                secondary_color: c.secondary_color.unwrap_or_else(|| "#FFFFFF".into()),
            };
            world.clubs.insert(club.id.clone(), club);
        }

        Ok(())
    }

    fn load_players(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("players.json");
        if !path.exists() {
            // Create some default players for each club
            let mut player_id = 1;
            for club_id in world.clubs.keys().cloned().collect::<Vec<_>>() {
                // Create 15 players per club
                let positions = vec![
                    Position::Goalkeeper,
                    Position::DefenderLeft,
                    Position::DefenderCenter,
                    Position::DefenderCenter,
                    Position::DefenderRight,
                    Position::MidfielderLeft,
                    Position::MidfielderCenter,
                    Position::MidfielderCenter,
                    Position::MidfielderRight,
                    Position::MidfielderAttacking,
                    Position::ForwardLeft,
                    Position::ForwardCenter,
                    Position::ForwardCenter,
                    Position::ForwardRight,
                    Position::Goalkeeper,
                ];

                for (i, pos) in positions.iter().enumerate() {
                    let id = format!("P{:04}", player_id);
                    let mut player = Player::new(
                        &id,
                        format!("Player{}", player_id),
                        format!("Name{}", player_id),
                        NationId::new("ENG"),
                        NaiveDate::from_ymd_opt(1980 + (i as i32 % 15), 1, 1).unwrap(),
                        *pos,
                    );
                    player.club_id = Some(club_id.clone());
                    player.value = Money::from_major(100_000 + (i as i64 * 50_000));

                    // Set some basic attributes
                    player.attributes.technical.passing = 50 + (i as u8 % 30);
                    player.attributes.technical.finishing = 40 + (i as u8 % 40);
                    player.attributes.physical.pace = 50 + (i as u8 % 30);
                    player.attributes.physical.stamina = 60 + (i as u8 % 20);
                    player.attributes.mental.decisions = 50 + (i as u8 % 30);

                    world.players.insert(PlayerId::new(&id), player);

                    if let Some(club) = world.clubs.get_mut(&club_id) {
                        club.player_ids.push(PlayerId::new(&id));
                    }

                    player_id += 1;
                }
            }
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawPlayer> = serde_json::from_str(&content)?;

        for p in raw {
            let birth_date = NaiveDate::parse_from_str(&p.birth_date, "%Y-%m-%d")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1990, 1, 1).unwrap());

            let position = parse_position(&p.position);

            let mut player = Player::new(
                &p.id,
                &p.first_name,
                &p.last_name,
                NationId::new(&p.nationality),
                birth_date,
                position,
            );

            if let Some(club) = &p.club_id {
                player.club_id = Some(ClubId::new(club));
                if let Some(c) = world.clubs.get_mut(&ClubId::new(club)) {
                    c.player_ids.push(PlayerId::new(&p.id));
                }
            }

            player.value = Money::from_major(p.value.unwrap_or(100_000));

            world.players.insert(PlayerId::new(&p.id), player);
        }

        Ok(())
    }

    fn load_competitions(&self, world: &mut World) -> Result<(), DataError> {
        let path = Path::new(&self.data_dir).join("competitions.json");
        if !path.exists() {
            // Create default Premier League
            let mut league = Competition::new("EPL", "English Premier League", CompetitionType::League);
            league.short_name = "Premier League".into();
            league.nation_id = Some(NationId::new("ENG"));
            league.reputation = 95;

            // Add all English clubs
            for (club_id, club) in &world.clubs {
                if club.nation_id.as_str() == "ENG" {
                    league.add_team(club_id.clone());
                }
            }

            world.competitions.insert(CompetitionId::new("EPL"), league);
            return Ok(());
        }

        let content = std::fs::read_to_string(&path)?;
        let raw: Vec<RawCompetition> = serde_json::from_str(&content)?;

        for c in raw {
            let comp_type = match c.competition_type.as_deref() {
                Some("cup") => CompetitionType::Cup,
                Some("international") => CompetitionType::International,
                _ => CompetitionType::League,
            };

            let mut comp = Competition::new(&c.id, &c.name, comp_type);
            comp.short_name = c.short_name.unwrap_or_default();
            comp.nation_id = c.nation_id.as_ref().map(|s| NationId::new(s));
            comp.reputation = c.reputation.unwrap_or(50);

            for team_id in c.teams.unwrap_or_default() {
                comp.add_team(ClubId::new(&team_id));
            }

            world.competitions.insert(CompetitionId::new(&c.id), comp);
        }

        Ok(())
    }
}

fn parse_position(s: &str) -> Position {
    match s.to_uppercase().as_str() {
        "GK" => Position::Goalkeeper,
        "DC" | "CB" => Position::DefenderCenter,
        "DL" | "LB" => Position::DefenderLeft,
        "DR" | "RB" => Position::DefenderRight,
        "MC" | "CM" => Position::MidfielderCenter,
        "ML" | "LM" => Position::MidfielderLeft,
        "MR" | "RM" => Position::MidfielderRight,
        "DM" | "DMC" => Position::MidfielderDefensive,
        "AM" | "AMC" => Position::MidfielderAttacking,
        "FC" | "ST" | "CF" => Position::ForwardCenter,
        "FL" | "LW" => Position::ForwardLeft,
        "FR" | "RW" => Position::ForwardRight,
        _ => Position::MidfielderCenter,
    }
}

// Raw JSON structures for deserialization
#[derive(Deserialize)]
struct RawNation {
    id: String,
    name: String,
    short_name: Option<String>,
    continent: Option<String>,
    reputation: Option<u8>,
    youth_rating: Option<u8>,
}

#[derive(Deserialize)]
struct RawClub {
    id: String,
    name: String,
    short_name: Option<String>,
    nation_id: Option<String>,
    stadium_id: Option<String>,
    reputation: Option<u8>,
    balance: Option<i64>,
    transfer_budget: Option<i64>,
    wage_budget: Option<i64>,
    primary_color: Option<String>,
    secondary_color: Option<String>,
}

#[derive(Deserialize)]
struct RawPlayer {
    id: String,
    first_name: String,
    last_name: String,
    nationality: String,
    birth_date: String,
    position: String,
    club_id: Option<String>,
    value: Option<i64>,
}

#[derive(Deserialize)]
struct RawCompetition {
    id: String,
    name: String,
    short_name: Option<String>,
    nation_id: Option<String>,
    competition_type: Option<String>,
    reputation: Option<u8>,
    teams: Option<Vec<String>>,
}
