//! When to shadow, and when to reach for a different name.
//!
//! The other two shadowing pages answer *what it is* and *what it does to the
//! value underneath*. This one is the judgement call: shadowing is a naming
//! decision, and the question a reader of your code has to answer is whether the
//! second `let` refined the same thing or introduced a different one.
//!
//! The rule that falls out of the six steps below is one sentence — shadow when
//! the new binding is the same concept in a new form — and steps 3 to 5 are the
//! three ways code breaks when it is ignored. Each of those three is a real bug
//! that compiles, and only one of them gets a warning.
//!
//!   rustc --edition 2024 when_to_shadow.rs -o /tmp/wts && /tmp/wts

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ───────────────────────────────────────────────────────────────────── 1
// The design argument, which is easy to miss: the alternative to shadowing is
// not "a longer name", it is `mut` — and `mut` says something much weaker.
fn step1() {
    banner(1, "What the feature buys: it keeps `mut` meaning something");

    // The shadowing version. Three bindings, none of them mutable.
    let raw = "  42  ";
    let raw = raw.trim();
    let raw: u32 = raw.parse().expect("the literal above is a number");
    println!("  shadowed: {raw}");
    println!("      Three `let`s, zero `mut`. Every line's value is final, and");
    println!("      the type changed twice without inventing `raw_trimmed`.");

    // The mutable version cannot do it at all: `mut` re-assigns, and a
    // re-assignment must keep the type. So the type change forces a new name.
    let mut text = "  42  ";
    text = text.trim();
    let number: u32 = text.parse().expect("same literal");
    println!("  with mut: {number}");
    println!("      `mut` could not carry &str -> u32, so a second name arrived");
    println!("      anyway — and `text` is now mutable for the rest of the");
    println!("      scope, which promises the reader far less than the");
    println!("      shadowed version did.");
    println!("      THAT is the trade: shadowing says \"changed HERE\";");
    println!("      `mut` says \"may change ANYWHERE below\".");
}

// ───────────────────────────────────────────────────────────────────── 2
// Every one of these is the same concept arriving in a better form. That is the
// whole test for whether a shadow is the right call.
fn read_header(path: impl AsRef<Path>) -> String {
    // Idiom: generic parameter -> the one concrete type the body wants. The
    // canonical opening line of any `AsRef` API.
    let path = path.as_ref();
    format!("{}", path.display())
}

fn shout(name: Option<String>) -> String {
    // Idiom: unwrap-and-narrow. Option<String> -> String -> &str -> String,
    // and at no point is there a second name to keep straight.
    let Some(name) = name else {
        return "nobody".to_string();
    };
    let name = name.trim();
    name.to_uppercase()
}

struct OrderId(u32);

fn step2() {
    banner(2, "The idioms: same concept, better form");

    println!("  generic -> concrete:  {}", read_header("/etc/hosts"));
    println!("  unwrap-and-narrow:    {}", shout(Some("  ada  ".to_string())));

    // Idiom: freeze. Build it mutably, then take the `mut` away for good.
    let mut totals = Vec::new();
    totals.push(5);
    totals.push(3);
    let totals = totals;
    println!("  frozen after build:   {totals:?}");
    println!("      From this line on, `totals` cannot be pushed to. The `mut`");
    println!("      was scoped to the building, not to the variable's life.");

    // Idiom: narrow into a newtype, and the loose form becomes unnameable.
    let id = 42_u32;
    let id = OrderId(id);
    println!("  narrowed to newtype:  OrderId({})", id.0);
    println!("      The bare u32 is still alive, but nothing can reach it —");
    println!("      so no later line can pass a raw number where an id is due.");

    // Idiom: clone-for-move, shadowed INSIDE a block so the outer name survives.
    let rows = Arc::new(vec![5, 3, 0]);
    let counted = {
        let rows = Arc::clone(&rows);
        thread::spawn(move || rows.len())
    };
    println!(
        "  clone-for-move:       thread counted {}, outer still has {}",
        counted.join().expect("the thread does not panic"),
        rows.len()
    );
    println!("      The shadow lived and died inside the braces. Shadowing in an");
    println!("      inner block is the cheapest way to keep a name you still need.");
}

// ───────────────────────────────────────────────────────────────────── 3
// The first of three bugs that compile. This one at least gets a warning — but
// not the warning you would expect, which is why it survives code review.
//
// rustc says, about the `let mut total` below:
//   warning: variable does not need to be mutable
// and says nothing at all about the shadow that caused it.
#[allow(unused_mut)]
fn step3() {
    banner(3, "The accumulator that never accumulates");

    let mut total = 0;
    for score in [5, 3, 4] {
        let total = total + score; // shadows; the outer `total` is untouched
        println!("  inside the loop, total = {total}");
    }
    println!("  after the loop,  total = {total}");
    println!("      Every iteration built a fresh `total` from the outer 0 and");
    println!("      threw it away at the closing brace. The sum is lost.");
    println!("      The tell is a warning that never mentions shadowing:");
    println!("        warning: variable does not need to be mutable");
    println!("      An accumulator that does not need `mut` is not accumulating.");
    println!("      Read that warning as the bug report it is.");
}

// ───────────────────────────────────────────────────────────────────── 4
// The second bug, and the dangerous one: shadowing does not release what it
// hides, so shadowing anything that holds a resource doubles it.
fn step4() {
    banner(4, "The guard that was never released");

    let counter = Mutex::new(0);
    let register = Mutex::new(0);

    {
        let guard = counter.lock().expect("uncontended");
        println!("  read the counter through its guard: {}", *guard);

        let mut guard = register.lock().expect("uncontended");
        *guard += 1;
        println!("  holding the register guard, value = {}", *guard);

        println!(
            "  counter still locked?  {}",
            counter.try_lock().is_err()
        );
        println!(
            "  register still locked? {}",
            register.try_lock().is_err()
        );
        println!("      BOTH. The first guard did not go anywhere when its name");
        println!("      was taken — it is alive and holding the lock until the");
        println!("      brace, with nothing left that could release it early.");
    }

    println!("  after the brace, counter free? {}", counter.try_lock().is_ok());
    println!("      In one thread this only wastes a lock. Add a second thread");
    println!("      that wants `counter` and it is a deadlock whose cause is a");
    println!("      line that looks like a rename.");
}

// ───────────────────────────────────────────────────────────────────── 5
// The third bug: one name doing two jobs. Nothing warns, because both bindings
// are read — the only thing wrong is that they mean different things.
fn step5() {
    banner(5, "One name, two concepts — the failure with no warning");

    let scores = [5, 2, 4, 0, 3];

    let threshold: usize = 3; // "a score has to be at least this to count"
    println!("  scores      = {scores:?}");
    println!("  minimum score to count: {threshold}");

    // Months later, a different edit needs the row count and reaches for the
    // nearest reasonable word. Both bindings are usize; both are read.
    let threshold = scores.len(); // "how many rows we have" — a DIFFERENT idea
    let counted = scores.iter().filter(|&&s| s >= threshold).count();

    println!("  threshold   = {threshold}   (the row count, not the minimum score)");
    println!("  counted     = {counted}   — expected 3, the scores that are >= 3");
    println!("      Both bindings are `usize` and both are read, so there is no");
    println!("      warning to catch it. The compiler cannot know that the first");
    println!("      `threshold` was a score and the second one a quantity.");
    println!("      This is the case for a second name. Not because shadowing is");
    println!("      unsafe — because the two values are not the same thing.");
}

// ───────────────────────────────────────────────────────────────────── 6
// And the one the compiler does catch outright, with a message that names the
// mechanism. Functions live in the value namespace, so a `let` hides one.
fn rows_read() -> usize {
    461
}

fn step6() {
    banner(6, "A shadow hides a function just as happily as a value");

    let rows_read = rows_read();
    println!("  rows_read = {rows_read}");
    println!("      The name is now a usize, so the function is unreachable:");
    println!("        let again = rows_read();");
    println!("        error[E0618]: expected function, found `usize`");
    println!("          | this function of the same name is available here,");
    println!("          | but it's shadowed by the local binding");
    println!("      Worth reading once, because it is the only shadowing");
    println!("      mistake rustc names out loud. Steps 3 to 5 got a misleading");
    println!("      warning, a correct-looking program, and nothing at all.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();

    println!("\n──── The rule");
    println!("  Shadow when the new binding is the SAME CONCEPT in a new form,");
    println!("  and keep it close to the one it replaces. Reach for a second");
    println!("  name when it is a different thing (step 5), and never shadow");
    println!("  something that holds a resource (step 4).");
}
