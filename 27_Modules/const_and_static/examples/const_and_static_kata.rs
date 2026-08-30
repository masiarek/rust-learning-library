//! Kata solution: the address you may not rely on, and the const fn that will not.
//!
//!   rustc --edition 2024 const_and_static_kata.rs -o /tmp/csk && /tmp/csk

const SCALE: [u8; 6] = [0, 1, 2, 3, 4, 5];
static METHODS: [&str; 3] = ["STAR", "Approval", "Plurality"];

const fn quorum(voters: u32) -> u32 {
    // Allowed in a const fn: arithmetic, comparison, if, loops, indexing.
    let half = voters / 2;
    if half == 0 { 1 } else { half + 1 }
}

/// Not const: it allocates.
fn quorum_message(voters: u32) -> String {
    format!("{} of {voters}", quorum(voters))
}

struct Election;
impl Election {
    const DEFAULT_SEATS: u32 = 1;
    const MAX_SEATS: u32 = 10;

    fn seats(requested: u32) -> u32 {
        requested.clamp(Self::DEFAULT_SEATS, Self::MAX_SEATS)
    }
}

fn main() {
    println!("1. Where they may appear, and where they may not");
    println!("   const QUORUM: u32 = quorum(450);  ->  {}", quorum(450));
    println!("   const N: u32 = some_fn();         ->  E0015 unless some_fn is const");
    println!("   static as an array length: [u8; SCALE.len()] is fine, because");
    println!("   SCALE is a const. A `static`'s value cannot be used that way.");
    let sized: [u8; SCALE.len()] = SCALE;
    println!("   [u8; SCALE.len()] built: {sized:?}");

    println!();
    println!("2. The address question, measured");
    let c1: *const u8 = &SCALE[0];
    let c2: *const u8 = &SCALE[0];
    let s1: *const &str = &METHODS[0];
    let s2: *const &str = &METHODS[0];
    println!("   &SCALE[0] twice   (const):  {}", c1 == c2);
    println!("   &METHODS[0] twice (static): {}", s1 == s2);
    println!("   Both true, and only the second is guaranteed. Taking a reference");
    println!("   to a const promotes the value into an anonymous static, and rustc");
    println!("   may share one or emit several — so equality here is an");
    println!("   observation about this build, not a rule. The static is a rule.");
    let a = Box::new(SCALE);
    let b = Box::new(SCALE);
    println!("   Box::new(SCALE) twice, same address: {}",
             std::ptr::eq(&*a as *const _, &*b as *const _));
    println!("   Two copies, because the const was substituted into each Box.");

    println!();
    println!("3. What a const fn may not do");
    println!("   quorum(450)         = {}   compile time or run time, either", quorum(450));
    println!("   quorum_message(450) = {}   run time only", quorum_message(450));
    println!("   The second allocates a String, and allocation is not allowed in a");
    println!("   const context. Neither is calling a non-const fn, or reading a");
    println!("   `static`. Marking a function `const` is a PROMISE about what it");
    println!("   does not do, and it is part of your public API: un-consting one");
    println!("   later is a breaking change.");

    println!();
    println!("4. Associated consts, which is where most consts belong");
    println!("   Election::DEFAULT_SEATS = {}", Election::DEFAULT_SEATS);
    println!("   Election::seats(0)  = {}", Election::seats(0));
    println!("   Election::seats(50) = {}", Election::seats(50));
    println!("   Namespaced under the type, so two types can each have a MAX_SEATS");
    println!("   and neither has to be MAX_SEATS_FOR_ELECTION. A trait can declare");
    println!("   one too, which is how `u8::MAX` and `f64::EPSILON` are written.");

    println!();
    println!("5. The rule");
    println!("   const   unless you can say why the address matters.");
    println!("   static  for a large table, an FFI symbol, or shared mutable state");
    println!("           behind a Mutex / OnceLock / atomic.");
    println!("   A const array of 10_000 entries copied into three call sites is");
    println!("   30_000 entries in the binary. That is the one case where the");
    println!("   default is wrong, and the fix is one keyword.");
}
