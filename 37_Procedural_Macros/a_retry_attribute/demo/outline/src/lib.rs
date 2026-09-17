//! The attribute exactly as the course outline writes it, with no `sleep` key.
//! Nothing calls `fetch`: this crate is here to be expanded, not run.

use retry::retry;

#[retry(times = 3, delay_ms = 100)]
pub fn fetch(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
