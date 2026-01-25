//! Work permit system (stub).

use cm_core::ids::NationId;

/// Check if work permit required.
pub fn requires_work_permit(player_nation: &NationId, club_nation: &NationId) -> bool {
    player_nation != club_nation
}

/// Calculate work permit success chance.
pub fn work_permit_chance(player_reputation: u8, international_caps: u16) -> f32 {
    let base = (player_reputation as f32 / 100.0) * 0.5;
    let caps_bonus = (international_caps.min(50) as f32 / 50.0) * 0.3;
    (base + caps_bonus).min(0.95)
}
