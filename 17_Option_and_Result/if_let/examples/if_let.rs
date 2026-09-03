//! `if let`: a `match` with one arm — and what you give up for it.
//!
//! Most of the time you want to act on `Some` and do nothing at all on `None`.
//! Writing that as a `match` costs you an empty arm; `if let` is the same thing
//! with the empty arm deleted. The deletion is also the catch: the compiler stops
//! checking that you covered everything, which is the trade this file is about.
//!
//!   rustc --edition 2024 if_let.rs -o /tmp/il && /tmp/il

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn first(list: &[i32]) -> Option<&i32> {
    list.first()
}

fn step1() {
    banner(1, "The empty arm you did not want to write");

    let list = vec![10, 20, 30];

    match first(&list) {
        Some(x) => println!("  match  -> The first element is {x}"),
        None => {} // the arm that exists only to satisfy the compiler
    }

    if let Some(x) = first(&list) {
        println!("  if let -> The first element is {x}");
    }

    let empty: Vec<i32> = vec![];
    if let Some(x) = first(&empty) {
        println!("  if let -> The first element is {x}");
    }
    println!("      Nothing printed for the empty list — `if let` just does not run.");
    println!("      Same behaviour, same cost, one fewer arm to read.");
}

// ─────────────────────────────────────────────────────────── Step 2
#[derive(Debug)]
struct Review {
    author: &'static str,
    stars: u8,
}

fn step2() {
    banner(2, "It is a `match`, so any pattern works — and it can take an `else`");

    let reviews = vec![
        Review { author: "Ada", stars: 5 },
        Review { author: "Ben", stars: 2 },
    ];

    // Destructure through the reference the pattern hands you.
    if let Some(&Review { author, stars }) = reviews.first() {
        println!("  struct pattern  -> {author} gave {stars}");
    }

    // A literal inside the pattern is a condition, not a binding.
    if let Some(Review { author, stars: 5 }) = reviews.first() {
        println!("  literal in it   -> {author} gave the maximum");
    }

    // Tuples, nesting, and `_` all behave exactly as they do in a `match`.
    let pair = Some((1, "first"));
    if let Some((rank, label)) = pair {
        println!("  tuple pattern   -> rank {rank} is the {label}");
    }

    // And when you do care about the other case, `else` is right there.
    let missing: Option<i32> = None;
    if let Some(v) = missing {
        println!("  else            -> got {v}");
    } else {
        println!("  else            -> nothing there, and this arm says so");
    }
}

// ─────────────────────────────────────────────────────────── Step 3
#[derive(Debug)]
enum Mark {
    Rated(u8),
    Blank,
    Unreadable, // added to the enum later — this is the whole point of the step
}

fn count_with_match(marks: &[Mark]) -> (u32, u32, u32) {
    let (mut rated, mut blank, mut unreadable) = (0, 0, 0);
    for m in marks {
        match m {
            Mark::Rated(_) => rated += 1,
            Mark::Blank => blank += 1,
            Mark::Unreadable => unreadable += 1, // the compiler demanded this line
        }
    }
    (rated, blank, unreadable)
}

fn count_with_if_let(marks: &[Mark]) -> u32 {
    let mut rated = 0;
    for m in marks {
        if let Mark::Rated(_) = m {
            rated += 1;
        }
    }
    rated
}

fn step3() {
    banner(3, "The trade: `if let` is not exhaustive");

    let marks = vec![
        Mark::Rated(5),
        Mark::Blank,
        Mark::Rated(3),
        Mark::Unreadable,
        Mark::Rated(0),
    ];

    let (rated, blank, unreadable) = count_with_match(&marks);
    let counted = rated + blank + unreadable;
    let partial = count_with_if_let(&marks);
    let lost = marks.len() - partial as usize;
    println!("  match  -> {rated} rated, {blank} blank, {unreadable} unreadable = {counted} of {} marks", marks.len());
    println!("  if let -> {partial} rated, and {lost} marks silently unaccounted for");
    println!("      The day `Unreadable` was added to the enum, the `match` stopped");
    println!("      compiling and someone had to decide what to do about it.");
    println!("      The `if let` compiled fine and quietly went on under-counting.");
    println!("      That is the price of the deleted arm — pay it deliberately.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn top_seller(sales: &[(&str, u32)]) -> String {
    let Some((name, units)) = sales.first() else {
        return "no products at all".to_string();
    };
    // `name` and `units` are in scope for the whole rest of the function,
    // at the left margin, with no `if` wrapped around the real work.
    format!("{name} leads with {units}")
}

fn step4() {
    banner(4, "`let … else`: bind, or leave — the guard clause");

    println!("  populated -> {}", top_seller(&[("widget", 12), ("gadget", 7)]));
    println!("  empty     -> {}", top_seller(&[]));
    println!("      `if let` indents the happy path; `let … else` keeps it flat and");
    println!("      sends the failure out of the function. The `else` block must");
    println!("      diverge — return, break, continue, or panic — so what follows it");
    println!("      can rely on the binding existing.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "`while let`: keep going until the pattern stops matching");

    let mut stack = vec![10, 20, 30];
    while let Some(top) = stack.pop() {
        println!("  popped {top}, {} left", stack.len());
    }
    println!("      `pop()` is a partial function returning Option, so the loop's");
    println!("      exit condition IS the None — no length check, no index, no");
    println!("      off-by-one available to get wrong.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "When you only want a bool: `matches!` and `is_some_and`");

    let mark = Mark::Rated(4);
    println!("  matches!(mark, Mark::Rated(_))        -> {}", matches!(mark, Mark::Rated(_)));
    println!("  matches!(mark, Mark::Rated(n) if n>4) -> {}", matches!(mark, Mark::Rated(n) if n > 4));

    let stars: Option<u8> = Some(4);
    println!("  stars.is_some_and(|n| n > 3)          -> {}", stars.is_some_and(|n| n > 3));
    println!("      `if let` with an empty body and a flag set inside it is a smell.");
    println!("      If the answer is a bool, ask for a bool.");
}

// ─────────────────────────────────────────────────────────── Step 7
fn step7() {
    banner(7, "Edition 2024: `if let` chains");

    let first: Option<&str> = Some("alpha");
    let second: Option<&str> = Some("beta");

    if let Some(a) = first
        && let Some(b) = second
        && a != b
    {
        println!("  chained -> comparing {a} against {b}");
    }

    let solo: Option<&str> = None;
    if let Some(a) = first
        && let Some(b) = solo
    {
        println!("  chained -> comparing {a} against {b}");
    } else {
        println!("  chained -> no comparison: one of the two is absent");
    }
    println!("      Two `if let`s and a condition in one head, each binding visible");
    println!("      to the next. Stable since Rust 1.88, and only in edition 2024 —");
    println!("      before that this was a staircase of nested `if let`s.");
}

// ─────────────────────────────────────────────────────────── Step 8
fn step8() {
    banner(8, "It binds by MOVING, unless you ask otherwise");

    let name: Option<String> = Some("Ada".to_string());

    // `if let Some(n) = name` would move the String out and `name` would be
    // unusable afterwards. Matching on a reference borrows instead.
    if let Some(n) = &name {
        println!("  &name        -> borrowed {n}");
    }
    println!("  name after   -> {name:?}   (still ours)");

    if let Some(n) = name.as_deref() {
        println!("  as_deref()   -> {n} as a &str, no clone");
    }

    // The moving form, done on purpose, at the point we are finished with it.
    if let Some(n) = name {
        println!("  moved        -> took ownership of {n}");
    }
    println!("      Reaching for `.clone()` here is the usual reflex and the usual");
    println!("      mistake: `&opt`, `.as_ref()`, or `.as_deref()` cost nothing.");
}

// ─────────────────────────────────────────────────────────── Step 9
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropped the {}", self.0);
    }
}

impl Noisy {
    fn lookup(&self) -> Option<i32> {
        None
    }
}

fn open() -> Noisy {
    Noisy("temporary the scrutinee built")
}

fn step9() {
    banner(9, "Edition 2024 moved when the scrutinee's temporary dies");

    if let Some(v) = open().lookup() {
        println!("  found {v}");
    } else {
        println!("  the else block runs — and the temporary is already gone");
    }
    println!("      In edition 2021 the drop line came AFTER the else block: the");
    println!("      temporary lived to the end of the whole `if let`. If that");
    println!("      temporary is a lock guard, the else block that tries to take");
    println!("      the same lock deadlocks. Edition 2024 fixed it by dropping");
    println!("      before the else — one of the edition's few behaviour changes.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    step7();
    step8();
    step9();
    println!();
}
