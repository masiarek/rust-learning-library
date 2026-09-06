//! Kata solution: one `same_word` written three ways, then a two-level sort key
//! that puts 'Ł' between L and M — and the one row of the table that has to
//! change between German and Swedish.
//!
//!   rustc --edition 2024 comparing_strings_kata.rs -o /tmp/csk && /tmp/csk

/// Byte equality: what `==` does.
fn exact(a: &str, b: &str) -> bool {
    a == b
}

/// Folds `A`-`Z` only, and allocates nothing.
fn ascii_caseless(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Folds the whole Unicode case-mapping table, at the price of two `String`s.
fn mapped_caseless(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}

/// The Polish primary weight: the base letter, with case and accent discarded.
fn polish_base(c: char) -> char {
    match c {
        '\u{105}' | '\u{104}' => 'a',            // ą Ą
        '\u{107}' | '\u{106}' => 'c',            // ć Ć
        '\u{119}' | '\u{118}' => 'e',            // ę Ę
        '\u{142}' | '\u{141}' => 'l',            // ł Ł
        '\u{144}' | '\u{143}' => 'n',            // ń Ń
        '\u{f3}' | '\u{d3}' => 'o',              // ó Ó
        '\u{15b}' | '\u{15a}' => 's',            // ś Ś
        '\u{17A}' | '\u{179}' | '\u{17C}' | '\u{17B}' => 'z', // ź Ź ż Ż
        other => lowered(other),
    }
}

/// German: the umlauts are variants of their base vowel.
fn german_base(c: char) -> char {
    match c {
        '\u{e4}' | '\u{c4}' => 'a', // ä Ä
        '\u{f6}' | '\u{d6}' => 'o', // ö Ö
        '\u{fc}' | '\u{dc}' => 'u', // ü Ü
        other => lowered(other),
    }
}

/// Swedish: å ä ö are three more letters, and they come after Z. `{`, `|` and
/// `}` are the three code points immediately above 'z', so they stand in for
/// "a letter past the end of the alphabet" without inventing a wider type.
fn swedish_base(c: char) -> char {
    match c {
        '\u{e5}' | '\u{c5}' => '{', // å Å
        '\u{e4}' | '\u{c4}' => '|', // ä Ä
        '\u{f6}' | '\u{d6}' => '}', // ö Ö
        other => lowered(other),
    }
}

fn lowered(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// A two-level key, which is the smallest honest shape a collation key has:
/// compare base letters first, and only break a tie with the original text.
fn key(word: &str, base: fn(char) -> char) -> (String, String) {
    (word.chars().map(base).collect(), word.to_lowercase())
}

fn main() {
    println!("Round 1 -- one question, three answers");
    let pairs = [
        ("Content-Type", "content-type", "ASCII token"),
        ("STRASSE", "stra\u{df}e", "German \u{df}"),
        ("\u{141}\u{d3}D\u{179}", "\u{142}\u{f3}d\u{17a}", "Polish diacritics"),
        ("caf\u{e9}", "cafe\u{301}", "one accent, two spellings"),
    ];
    println!("   {:<16} {:<16} {:>7} {:>7} {:>8}  {}", "a", "b", "==", "ascii", "lowered", "what it is");
    for (a, b, note) in pairs {
        println!(
            "   {:<16} {:<16} {:>7} {:>7} {:>8}  {note}",
            format!("{a:?}"),
            format!("{b:?}"),
            exact(a, b),
            ascii_caseless(a, b),
            mapped_caseless(a, b)
        );
    }
    println!("   Row 2 is the one to look at: every answer is `false`, and the right");
    println!("   answer is `true`. Caseless matching is case FOLDING, and std has");
    println!("   none -- '\u{df}' folds to \"ss\", which no case MAPPING will do for you.");
    println!("   Row 4 is not a case question at all; the two spellings differ before");
    println!("   any case rule is applied, and no comparison in std repairs that.");

    println!("\nRound 2 -- three orders over one list");
    let names = [
        "\u{141}ukasiewicz",
        "Lewandowski",
        "Zawadzki",
        "\u{17B}eromski",
        "Adamczyk",
        "\u{106}wik\u{142}a",
    ];

    let mut byte_order = names;
    byte_order.sort();
    println!("   sort()                     {byte_order:?}");

    let mut lowered_order = names;
    lowered_order.sort_by_key(|n| n.to_lowercase());
    println!("   sort_by_key(to_lowercase)  {lowered_order:?}");

    let mut collated = names;
    collated.sort_by_cached_key(|n| key(n, polish_base));
    println!("   sort_by_cached_key(key)    {collated:?}");
    println!("   Only the third is Polish: '\u{106}' next to C, '\u{141}' between L and M, '\u{17B}'");
    println!("   after Z. The first two agree with each other and with nobody else.");

    println!("\nRound 3 -- the same list, two languages, one row of the table");
    let words = ["Zetter", "\u{c4}pfel", "Apfel", "\u{d6}ver"];
    let mut german = words;
    german.sort_by_cached_key(|w| key(w, german_base));
    println!("   german_base   {german:?}");
    let mut swedish = words;
    swedish.sort_by_cached_key(|w| key(w, swedish_base));
    println!("   swedish_base  {swedish:?}");
    println!("   Neither is a bug. German files '\u{c4}' under A; Swedish makes it the");
    println!("   27th letter. The two keys differ in three match arms and produce");
    println!("   two different, correct orders for the same four words.");

    println!("\nRound 4 -- what a per-character table cannot say at all");
    let czech = ["chata", "hora", "irsky"];
    let mut czech_sorted = czech;
    czech_sorted.sort_by_cached_key(|w| key(w, polish_base));
    println!("   by character  {czech_sorted:?}");
    println!("   Czech sorts 'ch' as ONE letter between H and I, so the answer wanted");
    println!("   is [\"hora\", \"chata\", \"irsky\"]. No mapping from char to char can");
    println!("   express a two-character letter, so the shape of the key is wrong");
    println!("   here, not its contents.");
    println!("   The order is not a property of the string. It is a property of the");
    println!("   LANGUAGE -- which is why correct collation ships as data (CLDR, ICU)");
    println!("   rather than as an algorithm you can derive.");
}
