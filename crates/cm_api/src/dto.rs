//! Data transfer objects for API.

use serde::{Deserialize, Serialize};

/// Club response DTO.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClubDto {
    pub id: String,
    pub name: String,
    pub reputation: u8,
}

/// Player response DTO.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDto {
    pub id: String,
    pub name: String,
    pub position: String,
    pub club_id: Option<String>,
}

/// Game state response DTO.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateDto {
    pub date: String,
    pub manager: String,
    pub club_id: String,
}
