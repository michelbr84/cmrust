//! Output formatting.

/// Print a success message.
#[allow(dead_code)]
pub fn success(msg: &str) {
    println!("✓ {}", msg);
}

/// Print an info message.
#[allow(dead_code)]
pub fn info(msg: &str) {
    println!("ℹ {}", msg);
}

/// Print an error message.
#[allow(dead_code)]
pub fn error(msg: &str) {
    eprintln!("✗ {}", msg);
}
