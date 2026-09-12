#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = "2024"

[dependencies]
anyhow = "1"
---
//! One file, its manifest at the top. Run it with:
//!     cargo +nightly -Zscript hello.rs
//! or `chmod +x hello.rs && ./hello.rs` — the shebang does the rest.

fn double(n: i32) -> i32 {
    n * 2
}

fn main() -> anyhow::Result<()> {
    println!("hello from a single-file script, {}", double(21)); // 42
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubles() {
        assert_eq!(super::double(2), 4);
    }
}
