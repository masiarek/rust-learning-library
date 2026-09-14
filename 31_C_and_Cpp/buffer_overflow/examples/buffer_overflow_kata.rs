//! Kata solution: four ways to write into a buffer too small, and the one the
//! compiler rejects before it runs.
//!
//!   rustc --edition 2024 buffer_overflow_kata.rs -o /tmp/bok && /tmp/bok
use std::panic::{catch_unwind, AssertUnwindSafe, set_hook};

fn main() {
    let src = b"lambda";        // 6 bytes
    set_hook(Box::new(|_| {})); // hush the panic banners; we report outcomes ourselves

    println!("THE C SHAPE");
    println!("  char buf[4]; strcpy(buf, \"lambda\");");
    println!("  Six bytes plus a NUL written into four. The write runs off the");
    println!("  end into whatever follows buf -- a saved register, the canary,");
    println!("  the return address. Nothing in the language stops it; what you");
    println!("  see is decided by what the compiler bolted on (a stack canary, a");
    println!("  FORTIFY _chk call) or, with those off, by luck.");
    println!();

    println!("1. THE RUNTIME INDEX -- a bounds check on every write");
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let mut b = [0u8; 4];
        for (i, &byte) in src.iter().enumerate() {
            b[i] = byte;        // panics at i == 4
        }
    }))
    .is_err();
    println!("  buf[i] = byte for i in 0..6  ->  {}",
             if panicked { "panic at i == 4, nothing past the end is written" } else { "ok" });
    println!();

    println!("2. THE BULK COPY -- lengths must match, so it cannot even start");
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let mut b = [0u8; 4];
        b.copy_from_slice(src); // 6 into 4
    }))
    .is_err();
    println!("  buf.copy_from_slice(6 bytes)  ->  {}",
             if panicked { "panic, buffer untouched" } else { "ok" });
    println!();

    println!("3. THE ONE THE COMPILER CATCHES OUTRIGHT");
    println!("  buf[4] = 0;  with buf: [u8; 4]");
    println!("  A constant index into a fixed-size array: rustc rejects it at");
    println!("  compile time -- \"this operation will panic at runtime\", lint");
    println!("  unconditional_panic. No run needed; the length is in the type.");
    println!();

    println!("4. THE CHECKED WRITE -- copy what fits, and know how much");
    let mut buf = [0u8; 4];
    let n = src.len().min(buf.len());
    buf[..n].copy_from_slice(&src[..n]);
    println!("  copied {n} of {}: {:?}", src.len(), std::str::from_utf8(&buf[..n]).unwrap());
    println!("  The bound C asks you to remember is the only thing the API lets");
    println!("  you express -- the length is not optional here.");

    assert_eq!(&buf, b"lamb");
}
