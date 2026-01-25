//! Output formatting.

/// Print a success message.
pub fn success(msg: &str) {
    println!("✓ {}", msg);
}

/// Print an info message.
pub fn info(msg: &str) {
    println!("ℹ {}", msg);
}

/// Print an error message.
pub fn error(msg: &str) {
    eprintln!("✗ {}", msg);
}
