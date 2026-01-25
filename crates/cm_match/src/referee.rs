//! Referee simulation (stub).

/// Referee strictness affects card probability.
pub fn card_probability(strictness: u8, foul_severity: u8) -> (f32, f32) {
    let base_yellow = 0.1 + (strictness as f32 / 200.0);
    let base_red = 0.01 + (strictness as f32 / 500.0);

    let severity_mult = foul_severity as f32 / 50.0;

    (base_yellow * severity_mult, base_red * severity_mult)
}

/// Check if foul results in a card.
pub fn check_card(strictness: u8, foul_severity: u8, roll: f32) -> Option<CardType> {
    let (yellow_prob, red_prob) = card_probability(strictness, foul_severity);

    if roll < red_prob {
        Some(CardType::Red)
    } else if roll < yellow_prob {
        Some(CardType::Yellow)
    } else {
        None
    }
}

/// Card type.
pub enum CardType {
    Yellow,
    Red,
}
