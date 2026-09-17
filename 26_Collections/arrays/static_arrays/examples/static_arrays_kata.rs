//! Kata solution: a static lookup table, checked for order while it compiles.
//!
//!   rustc --edition 2024 static_arrays_kata.rs -o /tmp/sak && /tmp/sak

/// Sorted by code, so a lookup can binary-search it.
static REASONS: [(u16, &str); 6] = [
    (200, "OK"),
    (201, "Created"),
    (301, "Moved Permanently"),
    (404, "Not Found"),
    (418, "I'm a teapot"),
    (500, "Internal Server Error"),
];

/// `binary_search_by_key` is not a `const fn`, so the order check is a loop.
const fn strictly_sorted(table: &[(u16, &str)]) -> bool {
    let mut i = 1;
    while i < table.len() {
        if table[i - 1].0 >= table[i].0 {
            return false;
        }
        i += 1;
    }
    true
}

// Evaluated while compiling: swap two rows above and the build fails.
const _: () = assert!(strictly_sorted(&REASONS), "REASONS must be sorted by code");

/// The answer borrows from the table, not from `code`, so it can be `'static`.
fn reason(code: u16) -> Option<&'static str> {
    REASONS
        .binary_search_by_key(&code, |&(c, _)| c)
        .ok()
        .map(|i| REASONS[i].1)
}

fn main() {
    println!("1. Lookups");
    for code in [200, 404, 418, 302] {
        println!("   reason({code}) = {:?}", reason(code));
    }

    println!();
    println!("2. Why the return type may say 'static");
    let found: &'static str = reason(301).unwrap();
    println!("   {found:?} is a borrow of REASONS, which lives for the whole run;");
    println!("   `code` is a u16 copied in, so there is no input borrow to tie it to");

    println!();
    println!("3. The order check ran before main did");
    println!("   strictly_sorted(&REASONS) = {}", strictly_sorted(&REASONS));
    println!("   with rows 404 and 301 swapped, rustc stops with E0080:");
    println!("   evaluation panicked: REASONS must be sorted by code");
}
