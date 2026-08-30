//! Fn, FnMut, FnOnce: which one a closure gets, and what decides it.
//!
//!   rustc --edition 2024 three_closure_traits.rs -o /tmp/tct && /tmp/tct

/// The tightest bound: callable through `&self`, so any number of times,
/// and it may not mutate what it captured.
fn twice_by_ref<F: Fn() -> usize>(f: F) -> (usize, usize) {
    (f(), f())
}

/// The middle bound: callable through `&mut self`, so repeatedly, and it may
/// change what it captured between calls.
fn twice_by_mut<F: FnMut() -> usize>(mut f: F) -> (usize, usize) {
    (f(), f())
}

/// The loosest bound: callable through `self`, so exactly once — which is what
/// lets the body move a capture out.
fn once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn main() {
    println!("1. Three closures, three things the body does with the capture");
    let name = String::from("Ada");
    let reads = || name.len();
    println!("   || name.len()          reads it     -> {}", reads());
    println!("   the capture survives the call: name = {name:?}");

    let mut count = 0;
    let mut bumps = || {
        count += 1;
        count
    };
    println!("   || {{ count += 1; count }}  mutates it   -> {} then {}", bumps(), bumps());
    drop(bumps);
    println!("   the mutation is real: count = {count}");

    let owned = String::from("cookie");
    let eats = || owned;
    println!("   || owned               moves it out -> {:?}", eats());
    println!("   ...and `eats` is now spent. Calling it again is E0382.");

    println!();
    println!("2. The ladder: Fn is an FnMut is an FnOnce");
    println!("   trait FnMut<A>: FnOnce<A>   and   trait Fn<A>: FnMut<A>");
    let name2 = String::from("Lovelace");
    let reads2 = || name2.len();
    println!("   the reading closure, through F: Fn      -> {:?}", twice_by_ref(&reads2));
    println!("   the same closure, through F: FnMut      -> {:?}", twice_by_mut(&reads2));
    println!("   the same closure, through F: FnOnce     -> {:?}", once(|| format!("{} chars", reads2())));
    println!("   so a bound of FnOnce accepts ALL closures; a bound of Fn accepts");
    println!("   only the ones that neither mutate nor consume. Take the loosest");
    println!("   bound your body can live with, or you refuse callers for nothing.");

    println!();
    println!("3. What decides it is the BODY, not the `move` keyword");
    let cookie = String::from("cookie");
    let no_move_but_once = || cookie;             // no `move`, and still FnOnce
    println!("   `|| cookie`        (no move)  is FnOnce: {:?}", once(no_move_but_once));

    let biscuit = String::from("biscuit");
    let move_but_fn = move || biscuit.len();      // `move`, and still Fn
    println!("   `move || biscuit.len()`       is Fn:     {:?}", twice_by_ref(&move_but_fn));
    println!("   called twice, which an FnOnce could not be. `move` chose where the");
    println!("   String lives; the body chose which traits the closure gets.");

    println!();
    println!("4. An FnMut closure IS the state — and it needs a `mut` binding");
    let mut seen = Vec::new();
    let mut record = |row: &str| {
        seen.push(row.to_string());
        seen.len()
    };
    println!("   record(\"Ada\")  -> {}", record("Ada"));
    println!("   record(\"Ben\")  -> {}", record("Ben"));
    println!("   record(\"Cara\") -> {}", record("Cara"));
    drop(record);
    println!("   seen = {seen:?}");
    println!("   without `let mut record`, the call is E0596 — the closure has to be");
    println!("   borrowed mutably to be called at all.");

    println!();
    println!("5. Where std picked each one, and why");
    println!("   Option::unwrap_or_else(self, f: F)   F: FnOnce() -> T");
    println!("       runs at most once, so it may hand over an owned fallback.");
    println!("   Iterator::map(self, f: F)            F: FnMut(Self::Item) -> B");
    println!("       runs per item and is allowed to carry a running total.");
    println!("   Iterator::any(&mut self, f: F)       F: FnMut(Self::Item) -> bool");
    println!("   slice::sort_by_key(&mut self, f: F)  F: FnMut(&T) -> K");
    println!("   thread::spawn(f: F)                  F: FnOnce() -> T + Send + 'static");
    println!("       the thread body runs once; `Send + 'static` is a separate promise.");
    println!("   Each is the loosest bound that job can accept. Nothing here is about");
    println!("   speed: the bound decides which closures a caller may hand you.");
}
