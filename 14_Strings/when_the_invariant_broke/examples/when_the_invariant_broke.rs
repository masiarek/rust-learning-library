//! Three published CVEs in the string library, and the guard that closes each.
//! Every panic is caught, so the program exits 0 and its output can be recorded.

use std::borrow::Borrow;
use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};

/// Run `f`; hand back its panic message instead of letting it kill the process.
fn caught<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let outcome = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(hook);
    outcome.map_err(|e| match e.downcast_ref::<&str>() {
        Some(s) => (*s).to_string(),
        None => e.downcast_ref::<String>().cloned().unwrap_or_default(),
    })
}

/// A `Borrow<str>` that answers one way when asked for a length and another
/// when asked for the bytes. Nothing here is `unsafe`, and nothing here is
/// forbidden: `Borrow` does not promise to be consistent.
struct Shifty {
    calls: Cell<usize>,
    at_length_time: &'static str,
    at_copy_time: &'static str,
}

impl Shifty {
    fn new(at_length_time: &'static str, at_copy_time: &'static str) -> Self {
        Shifty { calls: Cell::new(0), at_length_time, at_copy_time }
    }
}

impl Borrow<str> for Shifty {
    fn borrow(&self) -> &str {
        let n = self.calls.get();
        self.calls.set(n + 1);
        if n == 0 { self.at_length_time } else { self.at_copy_time }
    }
}

fn main() {
    // ---------------------------------------------------------------- 1 ----
    println!("CVE-2018-1000810  str::repeat  — capacity by multiplication");
    let n = usize::MAX / 2 + 1;
    println!("  \"ab\".len() * n, wrapped   = {}", 2usize.wrapping_mul(n));
    println!("  \"ab\".len().checked_mul(n) = {:?}", 2usize.checked_mul(n));
    println!("  \"ab\".repeat(3)            = {:?}", "ab".repeat(3));
    println!("  \"ab\".repeat(n)            = panic {:?}", caught(|| "ab".repeat(n)).unwrap_err());

    // ---------------------------------------------------------------- 2 ----
    println!();
    println!("CVE-2020-36317  String::retain  — a predicate that panics");
    let mut s = String::from("0\u{e8}0");
    let original = s.clone().into_bytes();
    println!("  before        {:?}  {:02X?}  len {}", s, s.as_bytes(), s.len());
    let mut seen = 0;
    let err = caught(|| {
        s.retain(|_| {
            seen += 1;
            match seen {
                1 => false,                              // drop the leading '0'
                2 => true,                               // keep 'è' — it shifts left
                _ => panic!("the predicate gave up"),    // ...and then give up
            }
        })
    });
    println!("  panic         {:?}", err.unwrap_err());
    println!("  after         {:?}  {:02X?}  len {}", s, s.as_bytes(), s.len());
    // What the buffer held at the moment of the panic: the shifted 'è' the new
    // length keeps, followed by the tail `retain` never reached. Before 1.49 the
    // length stayed at 4, so a `String` claimed all four of these bytes.
    let mut mid_shift = s.as_bytes().to_vec();
    mid_shift.extend_from_slice(&original[2..]);
    println!("  the buffer held {:02X?}", mid_shift);
    println!("  as UTF-8      {:?}", std::str::from_utf8(&mid_shift));

    // ---------------------------------------------------------------- 3 ----
    println!();
    println!("CVE-2020-36323  [Borrow<str>]::join  — a Borrow that changes its mind");
    let honest = [Shifty::new("x", "x"), Shifty::new("a", "a")];
    println!("  consistent            {:?}", caught(|| honest.join("-")));
    let grows = [Shifty::new("x", "x"), Shifty::new("a", "aaaaaaaaaaaa")];
    println!("  1 byte, then 12       {:?}", caught(|| grows.join("-")));
    let shrinks = [Shifty::new("x", "x"), Shifty::new("aaaaaaaaaaaa", "a")];
    println!("  12 bytes, then 1      {:?}", caught(|| shrinks.join("-")));
}
