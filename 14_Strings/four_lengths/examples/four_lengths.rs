//! Four lengths, and which one the other system means.
//!
//! Build and run:
//!   rustc --edition 2024 four_lengths.rs -o /tmp/fl && /tmp/fl

/// Combining marks in the five main blocks. std has no character-database
/// lookup, so this is the honest approximation -- right for the samples here,
/// wrong in general. The real answer is the unicode-segmentation crate.
fn is_combining(c: char) -> bool {
    matches!(c as u32,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF
        | 0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

/// A crude cluster count: a new one starts at a character that is not a
/// combining mark, not a zero-width joiner, and not preceded by one.
fn clusters(text: &str) -> usize {
    let mut count = 0;
    let mut prev_zwj = false;
    for c in text.chars() {
        let zwj = c == '\u{200D}';
        if !is_combining(c) && !zwj && !prev_zwj {
            count += 1;
        }
        prev_zwj = zwj;
    }
    count
}

fn main() {
    let samples: [(&str, &str); 7] = [
        ("plain ASCII",     "cafe"),
        ("NFC - one char",  "caf\u{00E9}"),
        ("NFD - two chars", "cafe\u{0301}"),
        ("Polish",          "Za\u{017C}\u{00F3}\u{0142}\u{0107}"),
        ("emoji",           "\u{1F600}"),
        ("flag",            "\u{1F1F5}\u{1F1F1}"),
        ("family",          "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}"),
    ];

    println!("Round 1 - the four counts std can and cannot give you");
    println!("   {:<16} {:>6} {:>6} {:>7} {:>10}", "sample", "bytes", "chars", "utf-16", "clusters");
    println!("   {:<16} {:>6} {:>6} {:>7} {:>10}", "", ".len()", "count()", "units", "(no std)");
    println!("   {}", "-".repeat(50));
    for (label, text) in samples {
        println!("   {:<16} {:>6} {:>6} {:>7} {:>10}",
            label,
            text.len(),
            text.chars().count(),
            text.encode_utf16().count(),
            clusters(text));
    }
    println!("   Four columns, four different numbers for the same seven strings.");
    println!("   Only the first two are O(1) and O(n) in std; the fourth is a crate.");

    println!("\nRound 2 - every system you talk to already picked a column");
    let rows = [
        ("bytes",     "HTTP Content-Length, Postgres octet_length, a VARCHAR sized in bytes"),
        ("chars",     "Postgres char_length, Python len(), Rust .chars().count()"),
        ("utf-16",    "JavaScript .length, Java, C#, SQL Server nvarchar(n)"),
        ("clusters",  "the person who counted before hitting your character limit"),
    ];
    for (col, who) in rows {
        println!("   {:<9} {}", col, who);
    }

    println!("\nRound 3 - so the same string fits, or does not, depending who asks");
    let limit = 10;
    println!("   Does it fit a limit of {}?  (n) = the count that system would use\n", limit);
    println!("   {:<16} {:>12} {:>12} {:>12}", "sample", "byte column", "char column", "nvarchar");
    for (label, text) in samples {
        let fits = |n: usize| if n <= limit { format!("yes ({n})") } else { format!("NO  ({n})") };
        println!("   {:<16} {:>12} {:>12} {:>12}",
            label,
            fits(text.len()),
            fits(text.chars().count()),
            fits(text.encode_utf16().count()));
    }
    println!("\n   The Polish row is the one to look at: 10 bytes, 6 chars, 6 utf-16");
    println!("   units. It fits every column here -- and one more letter would");
    println!("   overflow the byte column while the other two still had room.");

    println!("\nRound 4 - the UTF-16 column is the one Rust never shows you");
    for (label, text) in [("emoji", "\u{1F600}"), ("Polish", "Za\u{017C}\u{00F3}\u{0142}\u{0107}")] {
        println!("   {:<8} {:?}  bytes {}  chars {}  utf-16 {}",
            label, text, text.len(), text.chars().count(), text.encode_utf16().count());
    }
    println!("   An emoji is ONE char and TWO utf-16 units -- a surrogate pair.");
    println!("   That is why JavaScript says \"\\u{{1F600}}\".length === 2, and why a");
    println!("   nvarchar(1) rejects it. encode_utf16() is how you ask for that");
    println!("   number without leaving Rust.");

    println!("\nRound 5 - what wc counts, and which method matches");
    let text = "Za\u{017C}\u{00F3}\u{0142}\u{0107} g\u{0119}\u{015B}l\u{0105} ja\u{017A}\u{0144}\n";
    println!("   text = {:?}", text);
    println!("   wc -c  {:>3}   text.len()", text.len());
    println!("   wc -m  {:>3}   text.chars().count()", text.chars().count());
    println!("   wc -w  {:>3}   text.split_whitespace().count()", text.split_whitespace().count());
    println!("   wc -l  {:>3}   text.matches('\\n').count()  -- NEWLINES, not lines",
        text.matches('\n').count());
    println!("   .lines() says {} too -- they agree here, because the text ends in",
        text.lines().count());
    println!("   a newline. Take it away and they stop agreeing:\n");
    for sample in ["a\nb\n", "a\nb"] {
        println!("   {:<8} matches('\\n') = {}   .lines() = {}",
            format!("{:?}", sample), sample.matches('\n').count(), sample.lines().count());
    }
    println!("\n   wc -l is a newline count, and so is matches(). .lines() counts");
    println!("   lines. The two answers differ on exactly the files a person");
    println!("   would call badly formed, which is most files.");
}
