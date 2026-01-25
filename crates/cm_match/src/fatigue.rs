//! Fatigue simulation (stub).

/// Calculate fatigue for a player during a match.
pub fn calculate_match_fatigue(
    starting_fitness: u8,
    minutes_played: u8,
    stamina: u8,
    intensity: u8,
) -> u8 {
    let base_drain = (minutes_played as f32 / 90.0) * 30.0;
    let stamina_mod = 1.0 - (stamina as f32 / 100.0) * 0.5;
    let intensity_mod = intensity as f32 / 50.0;

    let fatigue = (base_drain * stamina_mod * intensity_mod) as u8;
    starting_fitness.saturating_sub(fatigue)
}

/// Recovery between matches.
pub fn calculate_recovery(current_fitness: u8, days_rest: u8, natural_fitness: u8) -> u8 {
    let recovery_per_day = 5 + (natural_fitness / 20);
    let total_recovery = recovery_per_day.saturating_mul(days_rest);
    current_fitness.saturating_add(total_recovery).min(100)
}
