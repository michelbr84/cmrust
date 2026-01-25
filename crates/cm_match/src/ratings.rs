//! Player match ratings (stub).

/// Calculate player match rating.
pub fn calculate_rating(
    goals: u8,
    assists: u8,
    passes_completed: u16,
    tackles_won: u8,
    saves: u8,
    mistakes: u8,
) -> f32 {
    let mut rating = 6.0; // Base rating

    rating += goals as f32 * 0.8;
    rating += assists as f32 * 0.5;
    rating += (passes_completed as f32 / 20.0).min(1.0);
    rating += tackles_won as f32 * 0.2;
    rating += saves as f32 * 0.3;
    rating -= mistakes as f32 * 0.5;

    rating.clamp(1.0, 10.0)
}

/// Calculate man of the match.
pub fn determine_motm(ratings: &[(String, f32)]) -> Option<String> {
    ratings
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(name, _)| name.clone())
}
