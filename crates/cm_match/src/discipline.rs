//! Match discipline tracking (stub).

use cm_core::ids::PlayerId;
use std::collections::HashMap;

/// Match discipline tracker.
#[derive(Default)]
pub struct DisciplineTracker {
    yellow_cards: HashMap<PlayerId, u8>,
    red_cards: Vec<PlayerId>,
}

impl DisciplineTracker {
    /// Create new tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Issue yellow card.
    pub fn yellow_card(&mut self, player: PlayerId) -> bool {
        let count = self.yellow_cards.entry(player.clone()).or_insert(0);
        *count += 1;

        // Second yellow = red
        if *count >= 2 {
            self.red_cards.push(player);
            true
        } else {
            false
        }
    }

    /// Issue red card.
    pub fn red_card(&mut self, player: PlayerId) {
        self.red_cards.push(player);
    }

    /// Check if player is sent off.
    pub fn is_sent_off(&self, player: &PlayerId) -> bool {
        self.red_cards.contains(player)
    }

    /// Get sent off count.
    pub fn sent_off_count(&self) -> usize {
        self.red_cards.len()
    }
}
