//! Set pieces simulation (stub).

/// Set piece type.
pub enum SetPieceType {
    Corner,
    FreeKick,
    Penalty,
    ThrowIn,
}

/// Calculate set piece goal chance.
pub fn set_piece_goal_chance(set_piece: SetPieceType, attacking_strength: u8) -> f32 {
    let base = match set_piece {
        SetPieceType::Penalty => 0.75,
        SetPieceType::Corner => 0.03,
        SetPieceType::FreeKick => 0.05,
        SetPieceType::ThrowIn => 0.01,
    };

    base * (1.0 + (attacking_strength as f32 - 50.0) / 200.0)
}
