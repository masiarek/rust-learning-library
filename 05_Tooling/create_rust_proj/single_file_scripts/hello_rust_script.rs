#!/usr/bin/env rust-script
//! The same program in rust-script's syntax: the manifest is a fenced `cargo`
//! block inside the crate doc comment. Run it with:
//!     rust-script hello_rust_script.rs
//! or `chmod +x hello_rust_script.rs && ./hello_rust_script.rs`.
//!
//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! ```

fn double(n: i32) -> i32 {
    n * 2
}

fn main() -> anyhow::Result<()> {
    println!("hello from rust-script, {}", double(21)); // 42
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubles() {
        assert_eq!(super::double(2), 4);
    }
}
