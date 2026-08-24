//! Kata solution: three arrivals, three types — then break both promises
//! on purpose and read the refusals.
//!
//!   rustc --edition 2024 six_kinds_of_string_kata.rs -o /tmp/sixk && /tmp/sixk

use std::ffi::{CString, OsStr};

fn main() {
    println!("Round 1 — route each arrival to its type");
    println!("   text you built yourself      -> String    (UTF-8 is yours to promise)");
    println!("   a filename from the OS       -> OsString  (the OS promised nothing)");
    println!("   a string headed into C code  -> CString   (C stops at the first NUL)");

    println!("\nRound 2 — break the UTF-8 promise");
    use std::os::unix::ffi::OsStrExt;
    let filename = OsStr::from_bytes(&[b'c', b'a', b's', b'e', 0xF5, b'.', b'y', b'a', b'm', b'l']);
    println!("   a real, legal Unix filename: {filename:?}");
    match filename.to_str() {
        Some(s) => println!("   to_str() -> Some({s:?})"),
        None => println!("   to_str() -> None      <- not UTF-8; String would have to lie"),
    }
    println!("   to_string_lossy() -> {:?}   <- the byte is gone, and says so", filename.to_string_lossy());

    println!("\nRound 3 — break the NUL promise");
    match CString::new("tally\0sheet") {
        Ok(c) => println!("   unexpectedly fine: {c:?}"),
        Err(e) => {
            println!("   CString::new(\"tally\\0sheet\") -> Err");
            println!("   the error names the byte: {e}");
            println!("   C would have read only {:?} — Rust refuses instead", "tally");
        }
    }

    println!("\nRound 4 — and the promise that always holds");
    let c = CString::new("tally").unwrap();
    let back = c.to_str();
    println!("   a CString of plain ASCII is also valid UTF-8: to_str() = {back:?}");
    println!("   every narrowing that checks out hands you the &str view for free");
}
