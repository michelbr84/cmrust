//! Match commentary generation (stub).

/// Generate commentary for a goal.
pub fn goal_commentary(minute: u8, scorer: &str, home: bool) -> String {
    let location = if home { "home" } else { "away" };
    format!("{}' GOAL! {} scores for the {} side!", minute, scorer, location)
}

/// Generate commentary for a save.
pub fn save_commentary(minute: u8, keeper: &str) -> String {
    format!("{}' Great save by {}!", minute, keeper)
}

/// Generate commentary for a card.
pub fn card_commentary(minute: u8, player: &str, yellow: bool) -> String {
    let card_type = if yellow { "yellow" } else { "red" };
    format!("{}' {} receives a {} card!", minute, player, card_type)
}

/// Generate half-time commentary.
pub fn halftime_commentary(home_goals: u8, away_goals: u8) -> String {
    format!(
        "Half-time: The score is {} - {}.",
        home_goals, away_goals
    )
}

/// Generate full-time commentary.
pub fn fulltime_commentary(home_goals: u8, away_goals: u8, home_name: &str, away_name: &str) -> String {
    let result = if home_goals > away_goals {
        format!("{} wins!", home_name)
    } else if away_goals > home_goals {
        format!("{} wins!", away_name)
    } else {
        "It's a draw!".to_string()
    };

    format!(
        "Full-time: {} {} - {} {}. {}",
        home_name, home_goals, away_goals, away_name, result
    )
}
