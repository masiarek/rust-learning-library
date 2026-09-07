//! Kata: write the safe version of each of the three, and name what the
//! `unsafe` one has to prove in order to be faster.

use std::borrow::Borrow;
use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};

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

/// One borrow per element, no promised capacity, no `unsafe`.
fn safe_join<S: Borrow<str>>(parts: &[S], sep: &str) -> String {
    let mut out = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(part.borrow());
    }
    out
}

/// Builds a new `String` instead of shifting bytes inside the old one, so an
/// unwind out of `f` drops a half-built value nobody can observe.
fn safe_retain(s: &String, mut f: impl FnMut(char) -> bool) -> String {
    s.chars().filter(|c| f(*c)).collect()
}

/// The capacity question asked out loud, instead of assumed.
fn safe_repeat(s: &str, n: usize) -> Option<String> {
    let capacity = s.len().checked_mul(n)?;
    let mut out = String::with_capacity(capacity);
    for _ in 0..n {
        out.push_str(s);
    }
    Some(out)
}

fn main() {
    println!("join — a Borrow that grows between the two calls");
    let a = [Shifty::new("x", "x"), Shifty::new("a", "aaaaaaaaaaaa")];
    println!("  std::join   {:?}", caught(|| a.join("-")));
    let b = [Shifty::new("x", "x"), Shifty::new("a", "aaaaaaaaaaaa")];
    println!("  safe_join   {:?}", safe_join(&b, "-"));
    println!("  the difference: safe_join borrows once and believes the answer;");
    println!("  join borrows twice, so it must survive two different answers.");

    println!();
    println!("retain — a predicate that panics");
    let s = String::from("0\u{e8}0");
    let mut seen = 0;
    let out = caught(|| {
        safe_retain(&s, |_| {
            seen += 1;
            match seen {
                1 => false,
                2 => true,
                _ => panic!("the predicate gave up"),
            }
        })
    });
    println!("  safe_retain {:?}", out);
    println!("  original    {:?}  {:02X?}", s, s.as_bytes());
    println!("  the difference: nothing was written into `s`, so an unwind cannot");
    println!("  leave it holding half of a moved character.");

    println!();
    println!("repeat — a capacity that would wrap");
    let n = usize::MAX / 2 + 1;
    println!("  safe_repeat(\"ab\", 3)  {:?}", safe_repeat("ab", 3));
    println!("  safe_repeat(\"ab\", n)  {:?}", safe_repeat("ab", n));
    println!("  the difference: `?` on a `checked_mul` is the same guard std spells");
    println!("  `.expect(\"capacity overflow\")`; the bug was that neither was there.");
}
