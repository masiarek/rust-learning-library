//! Kata solution: the derive that changed behaviour when a field moved.
//!
//!   rustc --edition 2024 what_an_attribute_is_kata.rs -o /tmp/wak && /tmp/wak

/// Field order: name first.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ByName {
    name: &'static str,
    score: u32,
}

/// The same two fields, swapped. Nothing else differs.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ByScore {
    score: u32,
    name: &'static str,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Config {
    seats: u32,
    method: Method,
    strict: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Method {
    #[default]
    Star,
    Approval,
}

#[must_use]
fn tally(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn main() {
    println!("1. Derived Ord compares fields in declaration order");
    let mut a = vec![
        ByName { name: "Cara", score: 1 },
        ByName { name: "Ada", score: 9 },
        ByName { name: "Ben", score: 5 },
    ];
    let mut b = vec![
        ByScore { score: 1, name: "Cara" },
        ByScore { score: 9, name: "Ada" },
        ByScore { score: 5, name: "Ben" },
    ];
    a.sort();
    b.sort();
    println!("   name first : {:?}", a.iter().map(|r| r.name).collect::<Vec<_>>());
    println!("   score first: {:?}", b.iter().map(|r| r.name).collect::<Vec<_>>());
    println!("   Same data, same derive, opposite answers. Moving a field up is a");
    println!("   one-line diff that reads as cosmetic and silently reorders every");
    println!("   sort, every BTreeMap and every binary search over the type.");
    println!("   If the order matters, write the impl:");
    println!("     impl Ord for ByName {{ fn cmp(&self, o: &Self) -> Ordering {{ … }} }}");
    println!("   and then the diff that changes it is the one that says so.");

    println!();
    println!("2. `derive(Default)` on an enum needs `#[default]`");
    println!("   Config::default() = {:?}", Config::default());
    let approving = Config { method: Method::Approval, ..Config::default() };
    println!("   with one field changed: {approving:?}");
    println!("   Without `#[default]` on a variant, deriving Default for an enum is");
    println!("   E0665, \"`#[derive(Default)]` on enum with no `#[default]`\" — the");
    println!("   compiler cannot guess which variant is the zero. For a struct it");
    println!("   can: every field's own Default.");

    println!();
    println!("3. `#[must_use]` turns a discarded value into a warning");
    let total = tally(&[5, 3, 0]);
    println!("   tally(&[5, 3, 0]) = {total}");
    println!("   Writing `tally(&scores);` on its own line warns: \"unused return");
    println!("   value of `tally` that must be used\", and offers `let _ = ...` as");
    println!("   the way to say you meant it. It is the mechanism behind");
    println!("   the warning you have already met on Result and on iterator");
    println!("   adapters — `#[must_use]` sits on the TYPE there, not the method.");

    println!();
    println!("4. Attributes that change what is compiled, not how");
    println!("   #[cfg(test)]         the item does not exist in a normal build");
    println!("   #[cfg(unix)]         nor on Windows");
    println!("   #[cfg(feature = \"x\")] nor unless the feature is on");
    println!("   cfg!(unix) compiled on BOTH platforms — the branch not taken");
    println!("   still had to type-check, and only then was replaced by a literal.");
    println!("   #[cfg] deletes; cfg!() chooses. Which is why a cfg! branch cannot");
    println!("   rot and a #[cfg] one can: nobody compiles the deleted arm.");

    println!();
    println!("5. The four lint levels, and the one to avoid");
    println!("   allow / warn / deny / forbid, innermost wins — except `forbid`,");
    println!("   which cannot be relaxed by an inner `allow` (that is itself an");
    println!("   error). So a crate-wide `#![forbid(unsafe_code)]` is a real");
    println!("   promise, and a crate-wide `#![forbid(warnings)]` is a trap: the");
    println!("   day one generated macro trips a new lint, nobody can allow it");
    println!("   locally. Use deny for style, forbid for safety properties.");
}
