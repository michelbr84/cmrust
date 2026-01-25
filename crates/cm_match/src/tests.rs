//! Match engine tests.

#[cfg(test)]
mod tests {
    use crate::model::{MatchInput, TeamStrength};
    use crate::probabilistic::simulate_match;
    use cm_core::ids::ClubId;

    #[test]
    fn test_basic_match() {
        let input = MatchInput {
            home_id: ClubId::new("HOME"),
            away_id: ClubId::new("AWAY"),
            home: TeamStrength::default(),
            away: TeamStrength::default(),
            minutes: 90,
            seed: Some(123),
        };

        let result = simulate_match(&input);
        assert!(!result.highlights.is_empty());
    }

    #[test]
    fn test_stronger_team_advantage() {
        let mut home_wins = 0;
        let mut away_wins = 0;

        for seed in 0..100 {
            let input = MatchInput {
                home_id: ClubId::new("STRONG"),
                away_id: ClubId::new("WEAK"),
                home: TeamStrength {
                    attack: 90,
                    midfield: 85,
                    defense: 88,
                    finishing: 90,
                    morale: 80,
                    fitness: 90,
                },
                away: TeamStrength {
                    attack: 50,
                    midfield: 45,
                    defense: 48,
                    finishing: 50,
                    morale: 50,
                    fitness: 60,
                },
                minutes: 90,
                seed: Some(seed),
            };

            let result = simulate_match(&input);
            if result.is_home_win() {
                home_wins += 1;
            } else if result.is_away_win() {
                away_wins += 1;
            }
        }

        // Strong team should win more often
        assert!(home_wins > away_wins);
    }
}
