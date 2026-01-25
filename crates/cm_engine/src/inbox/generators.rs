//! Message generators (stub).

/// Generate welcome message.
pub fn welcome_message(manager: &str, club: &str) -> String {
    format!("Welcome {} to {}! Your journey begins now.", manager, club)
}
