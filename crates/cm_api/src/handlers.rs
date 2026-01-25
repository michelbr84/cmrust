//! API handlers (stubs).

use axum::{extract::Path, Json};

use crate::dto::{ClubDto, GameStateDto, PlayerDto};

/// Get club by ID.
pub async fn get_club(Path(id): Path<String>) -> Json<ClubDto> {
    Json(ClubDto {
        id,
        name: "Club".to_string(),
        reputation: 50,
    })
}

/// Get player by ID.
pub async fn get_player(Path(id): Path<String>) -> Json<PlayerDto> {
    Json(PlayerDto {
        id,
        name: "Player".to_string(),
        position: "MC".to_string(),
        club_id: None,
    })
}

/// Get game state.
pub async fn get_state() -> Json<GameStateDto> {
    Json(GameStateDto {
        date: "2001-07-01".to_string(),
        manager: "Manager".to_string(),
        club_id: "LIV".to_string(),
    })
}
