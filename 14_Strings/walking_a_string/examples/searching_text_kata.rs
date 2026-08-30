//! Kata solution: finding things in text with std alone — palindromes, words,
//! fields, offsets, prefixes, every match, and the two jobs that finally do
//! want a regex engine.
//!
//!   rustc --edition 2024 searching_text_kata.rs -o /tmp/stk && /tmp/stk

/// Reads the same forwards and backwards, ignoring case and punctuation.
/// Compares `char`s, so multibyte text works — but see the note in main():
/// a `char` is still not what a reader calls a character.
fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

/// Words, by the only definition std offers: runs between whitespace.
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// One CSV row into its fields. Every field is a view into `row` — no allocation.
fn fields(row: &str) -> Vec<&str> {
    row.split(',').collect()
}

/// Does it look like a URL? Two prefixes and a non-empty rest.
fn looks_like_url(s: &str) -> bool {
    (s.starts_with("http://") || s.starts_with("https://")) && !s.ends_with('/')
}

/// A crude address finder: split on whitespace, strip trailing punctuation,
/// keep what has one `@` with something either side and a dot after it.
fn find_emails(text: &str) -> Vec<&str> {
    text.split_ascii_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| {
            let mut parts = w.split('@');
            match (parts.next(), parts.next(), parts.next()) {
                (Some(user), Some(host), None) => {
                    !user.is_empty() && host.contains('.') && !host.starts_with('.')
                }
                _ => false,
            }
        })
        .collect()
}

/// Replace whole words, case-insensitively, with `****`. Whitespace runs are
/// normalised to single spaces — which is the first thing a regex would not do.
fn censor(text: &str, banned: &[&str]) -> String {
    text.split_whitespace()
        .map(|word| {
            let bare = word.trim_matches(|c: char| !c.is_alphanumeric());
            if banned.iter().any(|b| b.eq_ignore_ascii_case(bare)) {
                word.replace(bare, "****")
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!("1. Palindromes");
    for s in ["racecar", "A man, a plan, a canal: Panama", "Ada", "ala", "kajak"] {
        println!("   {:<32} {}", format!("{s:?}"), is_palindrome(s));
    }
    println!("   to_lowercase() on a char returns an ITERATOR, not a char — 'İ'");
    println!("   lowercases to two chars — which is why flat_map is here.");

    println!("\n2. Counting words");
    let para = "Score every candidate.\n  Then the top two\thave an automatic runoff.";
    println!("   {} words in {} bytes", word_count(para), para.len());
    println!("   split_whitespace() splits on RUNS of any whitespace, so the double");
    println!("   space, the newline and the tab each separate one pair of words:");
    println!("   {:?}", para.split_whitespace().collect::<Vec<_>>());
    println!("   split(' ') only knows the space character, so it keeps the newline");
    println!("   and the tab inside words and reports the gap as an empty field:");
    println!("   {:?}", para.split(' ').collect::<Vec<_>>());
    println!("   On \"a  b\" that is {:?} against {:?}.",
        "a  b".split(' ').collect::<Vec<_>>(),
        "a  b".split_whitespace().collect::<Vec<_>>());

    println!("\n3. CSV fields");
    let row = "Ada,5,2,,0";
    println!("   {row:?} -> {:?}", fields(row));
    println!("   The empty field survives: split() reports the gaps between matches,");
    println!("   so \"a,,b\" is three fields and the middle one is \"\".");

    println!("\n4. Where is it? find and rfind");
    let text = "runoff between the top two, then a second runoff if tied";
    println!("   {text:?}");
    println!("   find(\"runoff\")   = {:?}", text.find("runoff"));
    println!("   rfind(\"runoff\")  = {:?}", text.rfind("runoff"));
    println!("   find(\"instant\")  = {:?}   <- Option, not -1", text.find("instant"));
    if let (Some(a), Some(b)) = (text.find("runoff"), text.rfind("runoff")) {
        println!("   both are BYTE offsets, so they slice directly: {:?} … {:?}",
            &text[a..a + 6], &text[b..]);
    }

    println!("\n5. Does it look like a URL?");
    for s in [
        "https://masiarek.github.io/rust-learning-library",
        "http://example.com",
        "ftp://example.com",
        "example.com",
        "https://example.com/",
    ] {
        println!("   {:<52} {}", s, looks_like_url(s));
    }

    println!("\n6. Every match, not just the first");
    let ballots = "5,4,0 5,5,1 0,0,5 5,2,3";
    let fives: Vec<&str> = ballots.matches('5').collect();
    println!("   {ballots:?}");
    println!("   matches('5')       {:?}  ({} of them)", fives, fives.len());
    println!("   match_indices('5') {:?}",
        ballots.match_indices('5').map(|(i, _)| i).collect::<Vec<_>>());
    println!("   matches gives you the text, match_indices gives you where — and");
    println!("   overlapping matches are not reported: \"aaa\".matches(\"aa\") is {}",
        "aaa".matches("aa").count());

    println!("\n7. Addresses, without a regex engine");
    let inbox = "write to ada@example.com or (ben@sub.example.org). \
                 not-an-email@, nor @example.com, nor plain.text";
    println!("   {:?}", find_emails(inbox));
    println!("   Every one of those is a &str borrowed from `inbox` — no allocation.");

    println!("\n8. Censoring, without a regex engine");
    let post = "The Spoiler effect is a spoiler, and SPOILER talk is everywhere.";
    println!("   before  {post:?}");
    println!("   after   {:?}", censor(post, &["spoiler"]));
    println!("   Both of these are where a regex earns its place: word boundaries,");
    println!("   case folding and capture groups in one pass, instead of a hand-rolled");
    println!("   scanner per rule. `regex` is a crate, not std — deliberately, because");
    println!("   a regex engine is a compiler and std does not ship one.");
}
