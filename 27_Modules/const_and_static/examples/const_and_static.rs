//! `const` is inlined at every use. `static` is one address for the program.
//!
//!   rustc --edition 2024 const_and_static.rs -o /tmp/cs && /tmp/cs

/// Inlined wherever it is named. There is no `MAX_SCORE` in the binary.
const MAX_SCORE: u8 = 5;

/// One place in memory, for the whole run.
static METHOD: &str = "STAR";

/// A const can be computed, as long as the computation is a const fn.
const fn seats_for(voters: u32) -> u32 {
    if voters < 100 { 1 } else { voters / 100 }
}
const SEATS: u32 = seats_for(450);

/// Both work inside an impl, which is where a const usually belongs.
struct Ballot;
impl Ballot {
    const SCALE: &'static str = "0-5";
    fn describe() -> String {
        format!("{} on a {} scale", METHOD, Self::SCALE)
    }
}

fn main() {
    println!("1. Both are compile-time values, and neither may be `let`");
    println!("   MAX_SCORE = {MAX_SCORE}, METHOD = {METHOD}, SEATS = {SEATS}");
    println!("   Both need an explicit type — there is no inference at item level.");
    println!("   Both are SCREAMING_SNAKE_CASE by convention, and rustc warns if");
    println!("   they are not.");

    println!();
    println!("2. The difference is whether it HAS an address");
    let a: *const &str = &METHOD;
    let b: *const &str = &METHOD;
    println!("   &METHOD == &METHOD (static): {}", a == b);
    let c: *const u8 = &MAX_SCORE;
    let d: *const u8 = &MAX_SCORE;
    println!("   &MAX_SCORE == &MAX_SCORE (const): {}", c == d);
    println!("   Both true, and only the first one is a promise. A `static` IS a");
    println!("   single object at a fixed address, guaranteed. A `const` is a value");
    println!("   substituted at each use, and `&MAX_SCORE` has no address to take —");
    println!("   so rustc CONST-PROMOTES it into an anonymous static, and is free");
    println!("   to share one or make several. Do not build anything on that `true`.");
    println!("   Where the substitution is visible is a const that is not promoted:");
    let boxed1 = Box::new(MAX_SCORE);
    let boxed2 = Box::new(MAX_SCORE);
    println!("   Box::new(MAX_SCORE) twice, same address: {}",
             std::ptr::eq(&*boxed1, &*boxed2));
    println!("   Two separate values, because the const was copied into each.");

    println!();
    println!("3. Which to reach for");
    println!("   const   a value with a name: limits, scales, tuning knobs. The");
    println!("           default, and what you want almost every time.");
    println!("   static  when the address matters: a large table you do not want");
    println!("           copied into every use site, an FFI symbol, or anything");
    println!("           whose identity is part of its meaning.");
    println!("   A const of a big array is duplicated at each use; a static is not.");

    println!();
    println!("4. `const fn`, and what it may not do");
    println!("   seats_for(450) = {} — evaluated at compile time, so it can", SEATS);
    println!("   initialise a const. A const fn may branch, loop and do arithmetic,");
    println!("   and may not allocate, call a non-const fn, or read a static.");
    println!("   It is still an ordinary function at run time: seats_for(50) = {}",
             seats_for(50));

    println!();
    println!("5. Associated consts, and `static mut`");
    println!("   Ballot::SCALE   = {}", Ballot::SCALE);
    println!("   Ballot::describe() = {}", Ballot::describe());
    println!("   `static mut` exists and is mutable global state: every access is");
    println!("   `unsafe`, and in the 2024 edition even `&COUNT` inside an unsafe");
    println!("   block is refused — \"creating a shared reference to mutable");
    println!("   static\", from the deny-by-default `static_mut_refs` lint, with");
    println!("   `&raw const COUNT` offered as the replacement. What you want");
    println!("   instead is a `static` holding a Mutex, a OnceLock or an atomic —");
    println!("   safe, because those are the types that make sharing sound.");
}
