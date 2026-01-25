//! Agent system (stub).

/// Agent greed level.
pub fn agent_fee_multiplier(greed: u8) -> f32 {
    1.0 + (greed as f32 / 50.0)
}
