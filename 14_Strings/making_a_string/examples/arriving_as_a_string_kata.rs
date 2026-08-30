//! Kata solution: five ways text arrives — capacity, digits, a format, bytes,
//! and bytes that were not UTF-8 after all.
//!
//!   rustc --edition 2024 arriving_as_a_string_kata.rs -o /tmp/aask && /tmp/aask

use std::num::ParseIntError;

/// Parse without panicking. `&str` in, `Result` out — the caller decides.
fn read_score(raw: &str) -> Result<i32, ParseIntError> {
    raw.trim().parse::<i32>()
}

/// One line out of three unrelated types.
fn summary(seat: i32, share: f64, decided: bool) -> String {
    format!("seat {seat}: {share:.2}% turnout, decided = {decided}")
}

fn main() {
    println!("1. Pre-paying for the growth");
    let mut planned = String::with_capacity(100);
    println!("   String::with_capacity(100)  len {:>3}  capacity {:>3}", planned.len(), planned.capacity());
    for c in "ABCDEFGHIJ".chars() {
        planned.push(c);
    }
    println!("   after 10 pushes             len {:>3}  capacity {:>3}", planned.len(), planned.capacity());
    assert_eq!(planned.capacity(), 100, "ten bytes into a hundred: no reallocation");
    println!("   assert_eq!(capacity, 100) held — the buffer was bought once, up front.");
    println!("   (An empty String::new() would have reallocated on the way to 10.)");

    println!("\n2. Digits out of a raw field");
    for raw in [" 42 \n", "42", "4 2", ""] {
        let shown = format!("{raw:?}");
        match read_score(raw) {
            Ok(n) => println!("   read_score({shown:<9}) -> Ok({n})"),
            Err(e) => println!("   read_score({shown:<9}) -> Err({e})"),
        }
    }
    println!("   The trim is doing real work — `parse` itself refuses the spaces:");
    println!("   \" 42 \\n\".parse::<i32>()        -> {:?}", " 42 \n".parse::<i32>());
    println!("   The Result is the whole point — a bad field is data, not a crash.");

    println!("\n3. One String out of three types");
    println!("   {:?}", summary(3, 61.8375, true));
    println!("   {{:.2}} rounds for display only; the f64 itself is untouched.");

    println!("\n4. Bytes that really are UTF-8");
    let bytes: Vec<u8> = vec![0xE2, 0x98, 0x85, b' ', b'A', b'd', b'a'];
    match String::from_utf8(bytes.clone()) {
        Ok(s) => println!("   String::from_utf8({} bytes) -> Ok({s:?})", bytes.len()),
        Err(e) => println!("   String::from_utf8 -> Err({e})"),
    }
    println!("   No copy: from_utf8 takes the Vec<u8> and hands back the same allocation.");

    println!("\n5. Bytes that are not");
    let broken: Vec<u8> = vec![b'A', b'd', 0xFF, b'a', 0x9F];
    match String::from_utf8(broken.clone()) {
        Ok(s) => println!("   from_utf8       -> Ok({s:?})"),
        Err(e) => {
            println!("   from_utf8       -> Err: {e}");
            println!("                      first bad byte at index {}", e.utf8_error().valid_up_to());
            println!("                      and the Vec is handed back unharmed: {} bytes",
                e.into_bytes().len());
        }
    }
    let lossy = String::from_utf8_lossy(&broken);
    println!("   from_utf8_lossy -> {lossy:?}   <- each bad byte became U+FFFD");
    println!("   {} bytes in, {} chars out, {} of them the replacement character.",
        broken.len(),
        lossy.chars().count(),
        lossy.chars().filter(|&c| c == '\u{FFFD}').count());
    println!("   Lossy never fails, which is the risk: it silences the error rather than");
    println!("   reporting it. Use it for display; use from_utf8 when the bytes mattered.");
}
