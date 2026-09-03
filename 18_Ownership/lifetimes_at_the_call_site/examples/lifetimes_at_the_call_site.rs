//! What a lifetime does to the CALLER. Passing `&a` and `&b` into one function
//! locks both for as long as you keep the result — unless the signature says
//! which one the result came from.
//!
//!   rustc --edition 2024 lifetimes_at_the_call_site.rs -o /tmp/lacs && /tmp/lacs

/// Two lifetimes. The return type names only `'a`, so `y` is released at the call.
fn tally_of<'a, 'b>(tally: &'a str, _scratch: &'b str) -> &'a str {
    tally
}

/// One lifetime for both. The caller must now keep BOTH alive, whatever the body does.
fn either<'a>(tally: &'a str, _scratch: &'a str) -> &'a str {
    tally
}

fn main() {
    println!("──── 1. Two lifetimes: only the returned one stays locked");
    let tally = String::from("Ada 5, Ben 3");
    let mut scratch = String::from("scratch");
    let chosen = tally_of(&tally, &scratch);
    scratch.push_str(" — mutated while `chosen` is still alive");
    println!("  chosen  = {chosen}");
    println!("  scratch = {scratch}");
    println!("  The push_str happened while `chosen` was still in use. `scratch`");
    println!("  was released the moment the call returned, because the return type");
    println!("  names `'a` and nothing else — so no result can point into it.");

    println!();
    println!("──── 2. One lifetime: both stay locked");
    let tally2 = String::from("Ada 5, Ben 3");
    let scratch2 = String::from("scratch");
    let chosen2 = either(&tally2, &scratch2);
    println!("  chosen2 = {chosen2}");
    println!("  Same body, same call, one character different in the signature —");
    println!("  and now `scratch2.push_str(..)` on the line above would be E0502.");
    println!("  The body returns `tally` in BOTH functions. The compiler never looked.");

    println!();
    println!("──── 3. The signature is the whole contract");
    println!("  `either` always returns its first argument. Its signature does not");
    println!("  say so, so every caller pays for a result that might have come from");
    println!("  the second. Widening a lifetime in a signature costs the CALLER");
    println!("  something the body never uses — which is why `'a` on everything is");
    println!("  not the safe default it looks like.");
}
