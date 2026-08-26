//! Kata solution: a function that allocates only when it has something to change.
//!
//!   rustc --edition 2024 clone_on_write_kata.rs -o /tmp/cowk && /tmp/cowk

use std::borrow::Cow;

/// Return `s` unchanged when it already starts with `prefix`, otherwise a new
/// string with the prefix in front. The `'a` is the whole trick: the borrowed
/// arm hands back the caller's own bytes, so it must not outlive them.
fn ensure_prefix<'a>(s: &'a str, prefix: &str) -> Cow<'a, str> {
    if s.starts_with(prefix) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{prefix}{s}"))
    }
}

fn arm(c: &Cow<'_, str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "Borrowed",
        Cow::Owned(_) => "Owned",
    }
}

fn main() {
    let rows = ["BV2261 Ada", "Ben", "BV2261 Cara", "Dan"];

    println!("1. Prefix every row, paying only for the ones that lack it");
    let mut allocated = 0;
    for row in rows {
        let out = ensure_prefix(row, "BV2261 ");
        if matches!(out, Cow::Owned(_)) {
            allocated += 1;
        }
        println!("   {:<14} -> {:<22} {}", format!("{row:?}"), format!("{out:?}"), arm(&out));
    }
    println!("   allocated {} of {} rows", allocated, rows.len());

    println!();
    println!("2. Prove the untouched row was not copied");
    let row = "BV2261 Ada";
    let out = ensure_prefix(row, "BV2261 ");
    println!("   points at the caller's own bytes? {}", std::ptr::eq(row.as_ptr(), out.as_ptr()));

    println!();
    println!("3. What into_owned() costs on each arm");
    let borrowed = ensure_prefix("BV2261 Cara", "BV2261 ");
    let owned = ensure_prefix("Dan", "BV2261 ");
    println!("   {:<8} -> into_owned() allocates now      {:?}", arm(&borrowed), borrowed.into_owned());
    println!("   {:<8} -> into_owned() hands the buffer over {:?}", arm(&owned), owned.into_owned());

    println!();
    println!("4. The same rows through a plain String signature, for comparison");
    fn ensure_prefix_owned(s: &str, prefix: &str) -> String {
        if s.starts_with(prefix) { s.to_owned() } else { format!("{prefix}{s}") }
    }
    let _: Vec<String> = rows.iter().map(|r| ensure_prefix_owned(r, "BV2261 ")).collect();
    println!("   allocated {} of {} rows — correct, and it pays every time", rows.len(), rows.len());
}
