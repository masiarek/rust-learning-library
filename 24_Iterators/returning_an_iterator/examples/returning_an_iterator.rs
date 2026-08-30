//! Handing an iterator back: impl Iterator, dyn Iterator, and the lifetime.
//!
//!   rustc --edition 2024 returning_an_iterator.rs -o /tmp/rai && /tmp/rai

/// The ordinary case. The concrete return type is
/// `Map<Filter<Iter<'_, i32>, {closure}>, {closure}>` — unnameable, because
/// the two closures have no names.
fn doubled_odds(scores: &[i32]) -> impl Iterator<Item = i32> {
    scores.iter().filter(|s| *s % 2 == 1).map(|s| s * 2)
}

/// Borrowing, with the lifetime VISIBLE in the item type. Accepted by every
/// edition: `&str` mentions the borrow, so the opaque type is allowed to keep it.
fn names_over(rows: &[String], len: usize) -> impl Iterator<Item = &str> {
    rows.iter().filter(move |r| r.len() > len).map(String::as_str)
}

/// Borrowing, with the lifetime NOWHERE in the item type — `usize` owns
/// nothing — and the returned iterator holding `rows` anyway. This is the
/// edition-2024 rule doing work: in 2021 it is E0700.
fn lengths(rows: &[String]) -> impl Iterator<Item = usize> {
    rows.iter().map(|r| r.len())
}

/// Two shapes, one function: `impl Trait` cannot do this, because the branches
/// are different types. Boxing erases them.
fn maybe_reversed(scores: &[i32], reverse: bool) -> Box<dyn Iterator<Item = i32> + '_> {
    if reverse {
        Box::new(scores.iter().rev().copied())
    } else {
        Box::new(scores.iter().copied())
    }
}

/// Owning version: no lifetime, because nothing is borrowed.
fn counted(n: u32) -> impl Iterator<Item = u32> {
    (1..=n).map(|i| i * i)
}

struct Roster {
    rows: Vec<String>,
}

impl Roster {
    /// A method handing out a view of its own data. `+ '_` (or the elided
    /// lifetime in edition 2024) is what ties it to `&self`.
    fn shouted(&self) -> impl Iterator<Item = String> + '_ {
        self.rows.iter().map(|r| r.to_uppercase())
    }
}

fn main() {
    let scores = [5, 3, 0, 4, 2, 1];

    println!("1. Return the chain, not the collection");
    println!("   doubled_odds(&scores) -> {:?}", doubled_odds(&scores).collect::<Vec<_>>());
    println!("   The caller decides what to do with it — and can stop early:");
    println!("   .take(1)              -> {:?}", doubled_odds(&scores).take(1).collect::<Vec<_>>());
    println!("   Returning Vec<i32> instead would have allocated, and done all");
    println!("   the work, before the caller said it only wanted one.");

    println!();
    println!("2. Why the signature is `impl Iterator` and not the real type");
    println!("   the real type is Map<Filter<Iter<'_, i32>, {{closure}}>, {{closure}}>");
    println!("   and you cannot write that down: a closure has no name, so the");
    println!("   type has no spelling. `impl Iterator<Item = i32>` is the promise");
    println!("   without the spelling — one concrete type, chosen by the body.");

    println!();
    println!("3. Borrowing: the returned iterator holds the input");
    let rows = vec![
        String::from("Ada"),
        String::from("Bernadette"),
        String::from("Cara"),
    ];
    println!("   names_over(&rows, 3)  -> {:?}", names_over(&rows, 3).collect::<Vec<_>>());
    println!("   Item = &str, so the borrow is visible in the signature and every");
    println!("   edition accepts it.");
    println!("   lengths(&rows)        -> {:?}", lengths(&rows).collect::<Vec<_>>());
    println!("   Item = usize, which mentions no lifetime at all — yet the iterator");
    println!("   still holds `rows`. Edition 2024 captures every lifetime in scope,");
    println!("   so this compiles as written. Under --edition 2021 the SAME function");
    println!("   is E0700, \"hidden type captures lifetime that does not appear in");
    println!("   bounds\", and wants `+ use<'_>` spelled out. Both were checked.");

    println!();
    println!("4. Two branches, two types — which `impl Trait` cannot express");
    println!("   maybe_reversed(.., false) -> {:?}", maybe_reversed(&scores, false).collect::<Vec<_>>());
    println!("   maybe_reversed(.., true)  -> {:?}", maybe_reversed(&scores, true).collect::<Vec<_>>());
    println!("   `Rev<Copied<Iter<i32>>>` and `Copied<Iter<i32>>` are different");
    println!("   types, so an `impl Iterator` return is E0308 — the same refusal");
    println!("   as two closures in one variable. `Box<dyn Iterator>` erases both,");
    println!("   for one allocation and a virtual call per `next`.");

    println!();
    println!("5. Owning, so no lifetime at all");
    println!("   counted(5)            -> {:?}", counted(5).collect::<Vec<_>>());
    println!("   `(1..=n)` owns its state, so nothing is borrowed and the");
    println!("   signature stays bare. This is the version to prefer when you can.");

    println!();
    println!("6. A method handing out a view of its own rows");
    let roster = Roster { rows: rows.clone() };
    println!("   roster.shouted()      -> {:?}", roster.shouted().collect::<Vec<_>>());
    println!("   and the roster is still readable afterwards: {} rows", roster.rows.len());

    println!();
    println!("7. On the way IN, take IntoIterator rather than Iterator");
    fn total(rows: impl IntoIterator<Item = i32>) -> i32 {
        rows.into_iter().sum()
    }
    println!("   total(vec![1, 2, 3])  -> {}", total(vec![1, 2, 3]));
    println!("   total([1, 2, 3])      -> {}", total([1, 2, 3]));
    println!("   total(1..=3)          -> {}", total(1..=3));
    println!("   A parameter of `impl Iterator` would refuse the first two, and");
    println!("   make every caller write `.into_iter()`. Loosest bound wins here");
    println!("   exactly as it does for closures.");
}
