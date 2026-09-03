//! `r: &'a T` makes three claims at once. This program exercises the third —
//! the one that decides what may be *assigned into* `r` — and then shows the
//! single place the same substitution is refused: behind a `&mut`.
//!
//!   rustc --edition 2024 what_a_reference_claims.rs -o /tmp/wrc && /tmp/wrc

fn longer_of<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("──── 1. One name, two different lifetimes, and it still compiles");
    let forever: &'static str = "a string literal, alive for the whole program";
    let borrowed = String::from("a local String");
    println!("  longer_of(forever, &borrowed) = {}", longer_of(forever, &borrowed));
    println!("  `'a` was asked to cover both. The compiler did not widen the local —");
    println!("  it NARROWED the literal, because a longer lifetime is usable wherever");
    println!("  a shorter one is wanted. That direction is the whole rule.");

    println!();
    println!("──── 2. What may be assigned INTO a reference");
    let outer = String::from("outer — declared first, so it outlives `inner`");
    let mut r: &str = &outer;
    println!("  r = {r}");
    {
        let inner = String::from("inner — declared inside the block");
        r = &inner;
        println!("  r = {r}");
        println!("     ^ legal, because `r` is not read below this block.");
    }
    r = &outer;
    println!("  r = {r}");
    println!("     ^ always legal: `outer` outlives every region `r` is used in.");
    println!("  Keep the `r = &inner` line and read `r` after the brace and it is");
    println!("  E0597 — the value assigned in must outlive every LATER use of `r`.");

    println!();
    println!("──── 3. The direction reverses behind `&mut`");
    let mut slot: &str = "initially a literal";
    println!("  slot = {slot}");
    reassign(&mut slot, &outer);
    println!("  slot = {slot}");
    println!("  `&mut &'a str` is INVARIANT in 'a: the compiler will not quietly");
    println!("  shorten `'a` here the way it did in section 1, because writing");
    println!("  through the pointer could leave the caller holding a shorter");
    println!("  reference than its own type promises. This call is fine only");
    println!("  because `outer` genuinely outlives `slot`'s last use.");
}

fn reassign<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}
