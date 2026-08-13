//! `unwrap_or_default`: the fallback comes from the TYPE, not from the call site.
//!
//! It is exactly `unwrap_or_else(T::default)` with the closure spelled by a trait
//! bound. That makes it the shortest of the three, and the only one where the
//! value you get is decided somewhere else — in an `impl Default`, possibly a
//! derived one, possibly in a crate you have never opened.
//!
//! Which is fine when the type's zero really is your domain's zero, and quietly
//! wrong when it is not.
//!
//!   rustc --edition 2024 unwrap_or_default.rs -o /tmp/uod && /tmp/uod

use std::collections::BTreeMap;
use std::fmt::Debug;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn show<T: Default + Debug>(label: &str) {
    println!("  {label:<24} {:?}", T::default());
}

fn step1() {
    banner(1, "What `Default` hands you");

    show::<u8>("u8");
    show::<i64>("i64");
    show::<f64>("f64");
    show::<bool>("bool");
    show::<char>("char");
    show::<String>("String");
    show::<Vec<u8>>("Vec<u8>");
    show::<Option<u8>>("Option<u8>");
    show::<BTreeMap<u8, u8>>("BTreeMap<u8, u8>");
    show::<()>("()");

    let missing: Option<u8> = None;
    println!("  None.unwrap_or_default() {:?}   — the same call as unwrap_or_else(u8::default)", missing.unwrap_or_default());
    println!("      Nothing here is magic: `unwrap_or_default()` is unwrap_or_else with");
    println!("      the closure named by a trait bound, `T: Default`. Note char's — the");
    println!("      null character, not a space — and that Option's own default is None,");
    println!("      which is why a struct full of Options derives cleanly.");
}

// ─────────────────────────────────────────────────────────── Step 2
/// The tempting version: one derive, and every field takes its zero.
#[derive(Debug, Default, Clone, Copy)]
struct DerivedQuorum(u32);

/// The same newtype with the domain's answer written down instead.
#[derive(Debug, Clone, Copy)]
struct HouseQuorum(u32);

impl Default for HouseQuorum {
    fn default() -> Self {
        HouseQuorum(50)
    }
}

fn step2() {
    banner(2, "A derived default is the type's zero, not your domain's");

    let configured: Option<u32> = None; // nobody set a quorum in the config
    let ballots_cast = 40;

    let derived = configured.map(DerivedQuorum).unwrap_or_default();
    let house = configured.map(HouseQuorum).unwrap_or_default();

    println!("  ballots cast: {ballots_cast}, and the config set no quorum");
    println!("  #[derive(Default)] -> {derived:?}  => quorum met? {}", ballots_cast >= derived.0);
    println!("  impl Default (50)  -> {house:?}   => quorum met? {}", ballots_cast >= house.0);
    println!("      Same call site, opposite outcome, and the difference is a line of");
    println!("      code in another file. `derive(Default)` on a newtype means 'zero is");
    println!("      the sensible fallback' — true for a tally, false for a quorum, a");
    println!("      threshold, a timeout, a seat count. The third option is the best one");
    println!("      when there IS no sensible fallback: implement Default for neither,");
    println!("      and `unwrap_or_default()` stops compiling (E0277). A missing impl is");
    println!("      a guard rail, not a gap.");
}

// ─────────────────────────────────────────────────────────── Step 3
#[derive(Debug, Default, PartialEq)]
enum Tiebreak {
    #[default]
    Lot,
    MostFirstPlaces,
    Alphabetical,
}

fn step3() {
    banner(3, "On an enum, the compiler makes you say it out loud");

    let configured: Option<Tiebreak> = None;
    println!("  Tiebreak::default()            -> {:?}", Tiebreak::default());
    println!("  configured.unwrap_or_default() -> {:?}", configured.unwrap_or_default());
    for variant in [Tiebreak::Lot, Tiebreak::MostFirstPlaces, Tiebreak::Alphabetical] {
        let marked = if variant == Tiebreak::default() { "  <- #[default]" } else { "" };
        println!("    {variant:?}{marked}");
    }
    println!("      `#[derive(Default)]` on an enum does not compile without `#[default]`");
    println!("      on a variant (E0665: this enum needs a unit variant marked with");
    println!("      #[default]). There is no first-variant rule and no zero to fall back");
    println!("      on, so the language refuses to guess — and the attribute you are");
    println!("      forced to write is a policy decision sitting in the type, where a");
    println!("      reviewer can see it. Compare Step 2, where a struct's derive makes");
    println!("      the same class of decision silently.");
}

// ─────────────────────────────────────────────────────────── Step 4
#[derive(Debug)]
struct Ballot(u8);

fn report(label: &str, loaded: Option<Vec<Ballot>>) {
    // The convenient reading: no file and an empty file are the same thing.
    let flattened = loaded.unwrap_or_default();
    let total: u32 = flattened.iter().map(|b| b.0 as u32).sum();
    println!("  {label:<22} unwrap_or_default() -> {} ballots, {total} points", flattened.len());
}

fn honest(label: &str, loaded: &Option<Vec<Ballot>>) {
    match loaded {
        None => println!("  {label:<22} match -> no ballot file was provided; nothing to count"),
        Some(v) if v.is_empty() => println!("  {label:<22} match -> a real election in which nobody voted"),
        Some(v) => println!(
            "  {label:<22} match -> {} ballots, {} points",
            v.len(),
            v.iter().map(|b| b.0 as u32).sum::<u32>()
        ),
    }
}

fn step4() {
    banner(4, "Empty is not absent");

    let no_file: Option<Vec<Ballot>> = None;
    let empty_file: Option<Vec<Ballot>> = Some(Vec::new());
    let real: Option<Vec<Ballot>> = Some(vec![Ballot(5), Ballot(3)]);

    honest("no file at all", &no_file);
    honest("a file with no rows", &empty_file);
    honest("a file with 2 rows", &real);
    report("no file at all", no_file);
    report("a file with no rows", empty_file);

    println!("      The first two print the same line once the default is applied, and");
    println!("      one of them is a bug report: an input that never arrived. This is");
    println!("      the Option<Vec<T>> question — if 'missing' and 'empty' mean the same");
    println!("      thing to every caller, store a plain Vec and skip the Option; if they");
    println!("      do not, unwrap_or_default() is the line that throws the difference");
    println!("      away, and it is one word long.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "Where it is exactly right: zero as an identity");

    let marks = ["Ada", "Ben", "Ada", "Cara", "Ada", "Ben"];
    let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
    for name in marks {
        *counts.entry(name).or_default() += 1;
    }
    println!("  approvals: {counts:?}");
    for name in ["Ada", "Dan"] {
        println!(
            "  counts.get({name:?}).copied().unwrap_or_default() -> {}",
            counts.get(name).copied().unwrap_or_default()
        );
    }

    let mut pending = String::from("row 3: '4x' is not a number");
    let drained = std::mem::take(&mut pending);
    println!("  mem::take(&mut pending) -> {drained:?}, leaving {pending:?}");

    println!("      A candidate nobody approved really did get zero approvals: here the");
    println!("      type's zero IS the domain's answer, and `or_default` / `unwrap_or_");
    println!("      default` say so in fewer characters than the alternative. mem::take");
    println!("      is the same idea used for its other half — swap the default IN to");
    println!("      move the real value OUT, which is how you take a field out of a &mut.");
}

// ─────────────────────────────────────────────────────────── Step 6
#[derive(Debug)]
struct Config {
    port: u16,
    seats: u8,
    title: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8080,
            seats: 1,
            title: String::from("(untitled election)"),
        }
    }
}

fn step6() {
    banner(6, "Three spellings, and the place Default really earns its keep");

    let missing: Option<u32> = None;
    println!("  missing.unwrap_or(0)                 -> {}", missing.unwrap_or(0));
    println!("  missing.unwrap_or_else(u32::default) -> {}", missing.unwrap_or_else(u32::default));
    println!("  missing.unwrap_or_default()          -> {}", missing.unwrap_or_default());

    let cfg = Config {
        seats: 3,
        ..Default::default()
    };
    println!("  Config {{ seats: 3, ..Default::default() }}");
    println!("    -> port {}, seats {}, title {:?}", cfg.port, cfg.seats, cfg.title);
    println!("      All three fallbacks produce the same 0, and for a Copy type they");
    println!("      compile to the same thing — pick the one that says what you mean.");
    println!("      Struct update syntax is where Default is unambiguously good: every");
    println!("      field you did not name is filled from ONE impl you can read, and");
    println!("      adding a field later does not break the call. That is the same trait");
    println!("      doing the opposite job — stating the defaults in one place, instead");
    println!("      of quietly supplying one at a call site that never mentions it.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
