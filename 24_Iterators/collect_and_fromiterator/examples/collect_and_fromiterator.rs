//! collect: the consumer whose behaviour is chosen by the type you ask for.
//!
//!   rustc --edition 2024 collect_and_fromiterator.rs -o /tmp/cfi && /tmp/cfi

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Voter(String);

/// A collection of our own becomes a `collect` target by implementing one trait.
#[derive(Debug)]
struct Roster {
    rows: Vec<String>,
    longest: usize,
}

impl FromIterator<String> for Roster {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut rows = Vec::new();
        let mut longest = 0;
        for row in iter {
            longest = longest.max(row.len());
            rows.push(row);
        }
        Roster { rows, longest }
    }
}

/// Three use sites that pin `collect`'s target without anyone writing it there.
#[derive(Debug)]
struct Report {
    lines: Vec<String>,
}

fn widest(lines: Vec<String>) -> usize {
    lines.iter().map(String::len).max().unwrap_or(0)
}

fn shout(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| w.to_uppercase()).collect()
}

fn main() {
    println!("1. One call, and the TYPE decides what gets built");
    let words = ["Ada", "Ben", "Cara", "Ada"];
    let v: Vec<&str> = words.into_iter().collect();
    let s: String = words.into_iter().collect();
    let set: BTreeSet<&str> = words.into_iter().collect();
    let dq: VecDeque<&str> = words.into_iter().collect();
    println!("   Vec<&str>      {v:?}");
    println!("   String         {s:?}");
    println!("   BTreeSet<&str> {set:?}   (sorted, and the duplicate Ada is gone)");
    println!("   VecDeque<&str> {dq:?}");
    println!("   Same iterator four times. `collect` did not decide any of this;");
    println!("   the annotation did, by picking whose FromIterator impl runs.");

    println!();
    println!("2. Pairs collect into a map");
    let pairs = [("Ada", 5), ("Ben", 3), ("Ada", 4)];
    let map: BTreeMap<&str, i32> = pairs.into_iter().collect();
    println!("   BTreeMap  {map:?}");
    println!("   Ada appears twice in the input and once here: for a map, a later");
    println!("   key overwrites an earlier one. `collect` is not a merge — reach");
    println!("   for `fold` with `entry().or_insert()` when the totals matter.");

    println!();
    println!("3. Turbofish, when there is no variable to annotate");
    println!("   .collect::<Vec<_>>().len()   = {}", words.into_iter().collect::<Vec<_>>().len());
    println!("   the `_` is inference doing the element type; only the container");
    println!("   has to be named, and often only in one of the two places.");

    println!();
    println!("4. The one everybody needs: Result<Vec<_>, _> from fallible rows");
    let good = ["5", "3", "0"];
    let bad = ["5", "no", "0", "also no"];

    let all_good: Result<Vec<i32>, _> = good.into_iter().map(str::parse::<i32>).collect();
    println!("   good rows -> {:?}", all_good);

    let calls = Cell::new(0);
    let any_bad: Result<Vec<i32>, _> = bad
        .into_iter()
        .map(|s| {
            calls.set(calls.get() + 1);
            s.parse::<i32>()
        })
        .collect();
    println!("   bad rows  -> {}", match &any_bad {
        Ok(v) => format!("Ok({v:?})"),
        Err(e) => format!("Err({e})"),
    });
    println!("   and it stopped after {} of 4 rows — collect into a Result is", calls.get());
    println!("   short-circuiting. Note the shape it flipped: an iterator OF");
    println!("   Results became one Result OF a Vec, so the caller has one thing");
    println!("   to check instead of one per row.");

    let unflipped: Vec<Result<i32, _>> = bad.into_iter().map(str::parse::<i32>).collect();
    let shapes: Vec<String> = unflipped
        .iter()
        .map(|r| match r {
            Ok(n) => format!("Ok({n})"),
            Err(_) => "Err(..)".to_string(),
        })
        .collect();
    println!("   Same pipeline, one annotation apart:");
    println!("   Vec<Result<_, _>> -> [{}]", shapes.join(", "));
    println!("   all 4 rows ran and every outcome is kept — the un-flipped shape");
    println!("   has nothing to short-circuit TO, so it does not.");
    println!("   Only the FIRST error survives. To keep them all, collect into a");
    println!("   (Vec<_>, Vec<_>) with partition, or Vec<Result<_, _>> and sort it out.");

    let both: (Vec<_>, Vec<_>) = bad
        .into_iter()
        .map(str::parse::<i32>)
        .partition(Result::is_ok);
    println!("   partitioned -> {} ok, {} err (every error kept)", both.0.len(), both.1.len());

    println!();
    println!("5. Option collects the same way, and so does anything else you write");
    let evens: Option<Vec<i32>> = [2, 4, 6].into_iter().map(|n| (n % 2 == 0).then_some(n)).collect();
    let mixed: Option<Vec<i32>> = [2, 3, 6].into_iter().map(|n| (n % 2 == 0).then_some(n)).collect();
    println!("   all even -> {evens:?}");
    println!("   one odd  -> {mixed:?}   (one None and the whole answer is None)");

    let roster: Roster = ["Ada", "Bernadette", "Cara"].into_iter().map(String::from).collect();
    println!("   our own  -> Roster with {} rows, longest {}", roster.rows.len(), roster.longest);
    println!("   one `impl FromIterator<String> for Roster` and `.collect()` works");
    println!("   on it — the trait is the whole extension point.");

    println!();
    println!("6. What collect costs, and the two ways to pay less");
    let sizes: Vec<usize> = words.into_iter().map(str::len).collect();
    let mut reused: Vec<usize> = Vec::with_capacity(words.len());
    reused.extend(words.into_iter().map(str::len));
    println!("   collect  -> {sizes:?}  capacity {}", sizes.capacity());
    println!("   extend   -> {reused:?}  capacity {}", reused.capacity());
    println!("   `collect` allocates a new collection every time. `extend` pours");
    println!("   into one you already have, which is the loop-friendly form; and");
    println!("   an exact `size_hint` is what lets either make one allocation.");

    println!();
    println!("7. The useful degenerate case: Result<(), E>");
    let writes = ["5", "3", "0"];
    let checked: Result<(), std::num::ParseIntError> = writes
        .into_iter()
        .map(|s| s.parse::<i32>().map(|_| ()))
        .collect();
    println!("   all rows parse            -> {checked:?}");
    let checked_bad: Result<(), _> = ["5", "no"]
        .into_iter()
        .map(|s| s.parse::<i32>().map(|_| ()))
        .collect();
    println!("   one row does not          -> {}", match &checked_bad {
        Ok(()) => "Ok(())".to_string(),
        Err(e) => format!("Err({e})"),
    });
    println!("   There is a FromIterator impl for `()`, so collecting an iterator");
    println!("   of `Result<(), E>` gives one `Result<(), E>`: did every step work,");
    println!("   with the first failure and nothing else kept. That is the shape");
    println!("   for a run of fallible side effects whose successes carry no value.");
    println!("   Same trick, same warning: it stops at the first Err.");

    println!();
    println!("8. Deduplicating is a collect, and it costs a trait or two");
    let dedup: HashSet<Voter> = ["Ada", "Ben", "Ada"].into_iter().map(|n| Voter(n.to_string())).collect();
    let mut names: Vec<_> = dedup.iter().map(|v| v.0.as_str()).collect();
    names.sort();
    println!("   three names -> HashSet<Voter> -> {names:?}");
    println!("   `Voter` had to derive Eq and Hash to land in a HashSet, and Ord");
    println!("   to land in a BTreeSet. Which collection you collect into is a");
    println!("   claim about your type, checked at the collect call.");

    println!();
    println!("9. You do not have to WRITE the type — it has to be KNOWABLE");
    let lines = words.into_iter().map(str::to_uppercase).collect();
    let report = Report { lines };
    println!("   struct field    -> {:?}", report.lines);
    println!("   return position -> {:?}", shout(&words));
    println!(
        "   by-value arg    -> widest = {}",
        widest(words.into_iter().map(str::to_uppercase).collect())
    );
    println!("   Not one of those three `collect` calls names a type, and all");
    println!("   three compile: inference runs BACKWARD from where the value");
    println!("   lands. An annotation is not the requirement — a DETERMINED type");
    println!("   is, and a struct field, a return type or a parameter determines");
    println!("   one just as well as a binding does.");
    println!("   The use site that does NOT work is a `&[String]` parameter, which");
    println!("   is the one Rust otherwise tells you to prefer: `&Vec<String>` ->");
    println!("   `&[String]` is a deref coercion, and inference will not run a");
    println!("   coercion backward. It takes `[String]` literally and rejects it");
    println!("   as unsized — so that call is where you go back to a turbofish.");
}
