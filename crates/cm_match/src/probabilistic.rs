//! Probabilistic match simulation.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::model::{MatchInput, MatchResult};

/// Simulate a match tick-by-tick.
pub fn simulate_match(input: &MatchInput) -> MatchResult {
    let mut rng = match input.seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_entropy(),
    };

    let mut home_goals = 0u8;
    let mut away_goals = 0u8;
    let mut highlights = Vec::new();

    for minute in 1..=input.minutes {
        // Calculate attack chances based on team strengths
        let home_edge = input.home.attack.saturating_sub(input.away.midfield);
        let away_edge = input.away.attack.saturating_sub(input.home.midfield);

        // Base chance per minute (adjusted by team strength)
        let base_home = (0.010 + (home_edge as f32) * 0.0008).clamp(0.005, 0.040);
        let base_away = (0.010 + (away_edge as f32) * 0.0008).clamp(0.005, 0.040);

        // Roll for attack opportunities
        let home_roll: f32 = rng.gen();
        let away_roll: f32 = rng.gen();

        // Finishing vs Defense
        let home_finish = (input.home.finishing as f32 + rng.gen_range(0.0..6.0))
            - (input.away.defense as f32 * 0.35);
        let away_finish = (input.away.finishing as f32 + rng.gen_range(0.0..6.0))
            - (input.home.defense as f32 * 0.35);

        // Home goal chance
        if home_roll < base_home && home_finish > 6.5 {
            home_goals += 1;
            highlights.push(format!("{}' GOAL! Home team scores!", minute));
        }

        // Away goal chance
        if away_roll < base_away && away_finish > 6.5 {
            away_goals += 1;
            highlights.push(format!("{}' GOAL! Away team scores!", minute));
        }

        // Match events at key times
        if minute == 45 {
            highlights.push(format!("{}' Half-time whistle.", minute));
        }
        if minute == 90 {
            highlights.push(format!("{}' Full-time whistle.", minute));
        }
    }

    MatchResult {
        home_id: input.home_id.clone(),
        away_id: input.away_id.clone(),
        home_goals,
        away_goals,
        highlights,
    }
}

/// Simulate with extra time.
pub fn simulate_with_extra_time(input: &MatchInput) -> MatchResult {
    let mut result = simulate_match(input);

    // If draw in cup, add extra time
    if result.is_draw() {
        let mut extra_input = input.clone();
        extra_input.minutes = 30;
        extra_input.seed = input.seed.map(|s| s.wrapping_add(1));

        let extra_result = simulate_match(&extra_input);
        result.home_goals += extra_result.home_goals;
        result.away_goals += extra_result.away_goals;
        result.highlights.extend(extra_result.highlights);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TeamStrength;
    use cm_core::ids::ClubId;

    #[test]
    fn test_simulate_match_deterministic() {
        let input = MatchInput {
            home_id: ClubId::new("LIV"),
            away_id: ClubId::new("ARS"),
            home: TeamStrength {
                attack: 80,
                midfield: 75,
                defense: 78,
                finishing: 82,
                morale: 70,
                fitness: 85,
            },
            away: TeamStrength {
                attack: 75,
                midfield: 72,
                defense: 76,
                finishing: 78,
                morale: 65,
                fitness: 80,
            },
            minutes: 90,
            seed: Some(42),
        };

        let result1 = simulate_match(&input);
        let result2 = simulate_match(&input);

        // Same seed should give same result
        assert_eq!(result1.home_goals, result2.home_goals);
        assert_eq!(result1.away_goals, result2.away_goals);
    }
}
