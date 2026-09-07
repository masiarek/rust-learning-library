//! Kata: write the test that would have caught each bug, then run it against
//! the compiler you have.

/// Every substring of a word must be findable in it. Periodic words are the
/// ones that broke the 2014 searcher, so the list is stacked with them.
fn unfindable_substrings(word: &str) -> Vec<&str> {
    let mut missing = Vec::new();
    for i in 0..word.len() {
        for j in i + 1..=word.len() {
            if word.is_char_boundary(i) && word.is_char_boundary(j) {
                let part = &word[i..j];
                if !word.contains(part) || word.find(part).is_none() {
                    missing.push(part);
                }
            }
        }
    }
    missing
}

/// A sigma at the end of a word lowercases to ς however long the word is.
fn ascii_runs_ending_in_plain_sigma(max: usize) -> Vec<usize> {
    (1..=max)
        .filter(|k| !("a".repeat(*k) + "\u{3a3}").to_lowercase().ends_with('\u{3c2}'))
        .collect()
}

/// The other thing a "convert the ASCII prefix in chunks" path can get wrong:
/// a character whose case mapping is longer than the character.
fn ascii_runs_losing_the_sharp_s(max: usize) -> Vec<usize> {
    (0..=max)
        .filter(|k| {
            let word = "a".repeat(*k) + "\u{df}";
            word.to_uppercase() != "A".repeat(*k) + "SS"
        })
        .collect()
}

fn main() {
    println!("property 1 — every substring of a word is found in it");
    for word in ["bananas", "aaaa", "abcabcabc", "mississippi", "zażółć"] {
        println!("  {word:>12}  unfindable {:?}", unfindable_substrings(word));
    }

    println!();
    println!("property 2 — Σ at the end of a word lowercases to ς");
    println!("  k in 1..=64, wrong  {:?}", ascii_runs_ending_in_plain_sigma(64));

    println!();
    println!("property 3 — ß uppercases to SS whatever precedes it");
    println!("  k in 0..=64, wrong  {:?}", ascii_runs_losing_the_sharp_s(64));

    println!();
    println!("what std's own sigma tests looked like before 2024:");
    let mut longest = 0;
    for word in ["\u{391}\u{3a3}", "\u{391}'\u{3a3}", "\u{391}\u{3a3}'\u{391}", "'\u{3a3}", "''\u{3a3}", "\u{3a3}"] {
        let ascii_prefix = word.bytes().take_while(u8::is_ascii).count();
        longest = longest.max(ascii_prefix);
        println!("  {word:>6}  ->  {:<8}  ascii prefix {ascii_prefix}", word.to_lowercase());
    }
    println!("  longest ASCII prefix in the whole set  {longest}");
}
