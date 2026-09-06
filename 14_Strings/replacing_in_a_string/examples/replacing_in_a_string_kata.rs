//! Kata solution: the substitution table no ordering can save, the "last n"
//! replacement std does not offer, and the difference between deleting with
//! `retain` and deleting with `replace(.., "")`.
//!
//!   rustc --edition 2024 replacing_in_a_string_kata.rs -o /tmp/risk && /tmp/risk

/// One pass, first rule wins, scan continues past what was written.
fn substitute(text: &str, table: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    'scan: while !rest.is_empty() {
        for (from, to) in table {
            // An empty needle matches everywhere and would never advance `rest`.
            if from.is_empty() {
                continue;
            }
            if let Some(tail) = rest.strip_prefix(from) {
                out.push_str(to);
                rest = tail;
                continue 'scan;
            }
        }
        let c = rest.chars().next().unwrap();
        out.push(c);
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// The last `n` occurrences, which `replacen` counts from the wrong end.
/// Editing back to front is the whole trick: every offset still ahead of the
/// cursor is one you have not used yet, so none of them can go stale.
fn replace_last_n(text: &str, pat: &str, to: &str, n: usize) -> String {
    let mut out = String::from(text);
    for (at, found) in text.rmatch_indices(pat).take(n) {
        out.replace_range(at..at + found.len(), to);
    }
    out
}

fn main() {
    println!("1. A table no ordering can save");
    let s = "cat dog cat";
    println!("   {s:?}");
    println!("   cat->dog then dog->cat  {:?}", s.replace("cat", "dog").replace("dog", "cat"));
    println!("   dog->cat then cat->dog  {:?}", s.replace("dog", "cat").replace("cat", "dog"));
    println!("   Both orders collapse the two words into one. A chain cannot swap");
    println!("   anything, because the second pass reads the first pass's output.");
    println!("   substitute(...)         {:?}",
        substitute(s, &[("cat", "dog"), ("dog", "cat")]));
    println!("   One pass gets it right and does not care about the order:");
    println!("                           {:?}",
        substitute(s, &[("dog", "cat"), ("cat", "dog")]));

    println!("\n2. The last two, which replacen cannot count to");
    let log = "log a, log b, log c";
    println!("   {log:?}");
    println!("   {:<22} {:?}   <- counted from the left",
        "replacen(.., 2)", log.replacen("log", "LOGFILE", 2));
    println!("   {:<22} {:?}", "replace_last_n(.., 2)", replace_last_n(log, "log", "LOGFILE", 2));

    // The same offsets, applied front to back: each edit changes the length,
    // so every offset after it is measuring a string that no longer exists.
    let mut wrong = String::from(log);
    let mut ascending: Vec<usize> = log.rmatch_indices("log").take(2).map(|(at, _)| at).collect();
    ascending.reverse();
    for at in ascending {
        wrong.replace_range(at..at + 3, "LOGFILE");
    }
    println!("   {:<22} {:?}", "same offsets, forwards", wrong);
    println!("   The forward pass is not wrong about where the matches were. It is");
    println!("   wrong about what string it is editing after the first one lands.");

    println!("\n3. Two ways to delete, and only one is free");
    let messy = "  a b  c ";
    let mut owned = String::from(messy);
    owned.retain(|c| c != ' ');
    println!("   {:<22} {:?}   <- in place", "retain(|c| c != ' ')", owned);
    println!("   {:<22} {:?}   <- a new String", "replace(' ', \"\")", messy.replace(' ', ""));
    println!("   Same answer, different cost. Now the job retain cannot do:");
    let pairs = "abcba";
    let mut attempt = String::from(pairs);
    attempt.retain(|c| c != 'a' && c != 'b');
    println!("   {:<22} {:?}       <- every a and every b", "retain(not a, not b)", attempt);
    println!("   {:<22} {:?}     <- only the pair \"ab\"", "replace(\"ab\", \"\")", pairs.replace("ab", ""));
    println!("   retain is handed one char at a time and never sees a substring, so");
    println!("   it cannot express \"ab\". That is the line between the two families:");
    println!("   retain filters characters, replace matches patterns.");
}
