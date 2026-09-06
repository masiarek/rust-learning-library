//! K139 — Measure a column four ways, then truncate it without breaking a letter.
//!
//!   rustc --edition 2024 four_lengths_kata.rs -o /tmp/flk && /tmp/flk

fn utf16(s: &str) -> usize { s.encode_utf16().count() }

/// Longest prefix of `s` that fits in `max` UTF-8 bytes WITHOUT splitting a
/// character. The whole job: never return a byte index that is not a boundary.
fn truncate_bytes(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn main() {
    let column = [
        "Nowak",
        "\u{141}ukasiewicz",                       // Łukasiewicz
        "\u{17B}eromski",                          // Żeromski
        "\u{4E2D}\u{6751}",                        // 中村
        "Zo\u{EB} \u{1F600}",                      // Zoë + emoji
        "\u{1F600}\u{1F600}\u{1F600}\u{1F600}\u{1F600}",   // five emoji
    ];

    println!("Round 1 - four measurements of one column");
    println!("   {:<16} {:>6} {:>6} {:>7}", "value", "bytes", "chars", "utf-16");
    for s in column {
        println!("   {:<16} {:>6} {:>6} {:>7}", s, s.len(), s.chars().count(), utf16(s));
    }

    println!("\nRound 2 - the same limit, three different verdicts");
    let limit = 8;
    println!("   A column declared as 8 units. Which rows does each rule reject?\n");
    println!("   {:<16} {:>10} {:>10} {:>10}", "value", "VARCHAR", "char_length", "nvarchar");
    let mut disagreements = 0;
    for s in column {
        let (b, c, u) = (s.len() <= limit, s.chars().count() <= limit, utf16(s) <= limit);
        if !(b == c && c == u) {
            disagreements += 1;
        }
        let mark = |ok: bool| if ok { "ok" } else { "REJECT" };
        println!("   {:<16} {:>10} {:>10} {:>10}", s, mark(b), mark(c), mark(u));
    }
    println!("\n   {disagreements} row(s) where the three rules disagree. Every one of them");
    println!("   is a row that loads into one database and bounces off another.");
    println!("   The five-emoji row splits all three ways: 20 bytes, 5 chars,");
    println!("   10 utf-16 units -- rejected by two rules and accepted by one.");

    println!("\nRound 3 - truncating to a byte limit is where it actually breaks");
    let max = 6;
    for s in column {
        let cut = truncate_bytes(s, max);
        println!("   {:<16} -> {:?} ({} bytes of {} allowed)", s, cut, cut.len(), max);
    }
    println!("\n   &s[..6] would PANIC on the rows where byte 6 lands inside a letter.");
    println!("   is_char_boundary() is the check that makes the cut safe, and the");
    println!("   loop backing off from `max` is the whole of the fix.");

    println!("\nRound 4 - prove the naive cut really does panic");
    let name = "\u{141}ukasiewicz";
    for i in [1, 2, 6] {
        match name.get(..i) {
            Some(p) => println!("   get(..{i}) = Some({p:?})"),
            None => println!("   get(..{i}) = None      <- &name[..{i}] would panic here"),
        }
    }
    println!("\n   get() is the same question asked politely. Byte 1 is inside 'Ł',");
    println!("   which is two bytes, so a limit measured in bytes cannot be applied");
    println!("   with a slice unless you check first.");

    println!("\nRound 5 - the tables above are misaligned, and that is the lesson");
    println!("   Look at the CJK row: it does not line up. `{{:<16}}` pads to a");
    println!("   width counted in CHARS, and 中 occupies two terminal columns.");
    println!("   So the formatter picked a fifth answer to 'how long' -- display");
    println!("   width -- and std does not have that one either. Even the code");
    println!("   printing this table had to choose a count, and chose wrong.");
}
