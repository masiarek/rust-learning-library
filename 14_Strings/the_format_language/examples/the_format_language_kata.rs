//! Kata solution: a table whose columns are computed from the data — then the
//! three ways the mini-language stops doing what the column needed.
//!
//!   rustc --edition 2024 the_format_language_kata.rs -o /tmp/tflk && /tmp/tflk

struct Entry {
    name: &'static str,
    bytes: u64,
    note: &'static str,
}

const ENTRIES: [Entry; 4] = [
    Entry { name: "main.rs", bytes: 1204, note: "entry point" },
    Entry { name: "lib.rs", bytes: 88213, note: "everything else, and then some more of it" },
    Entry { name: "build.rs", bytes: 96, note: "" },
    Entry { name: "\u{17c}\u{f3}\u{142}w.txt", bytes: 7, note: "four chars, seven bytes" },
];

fn main() {
    println!("Round 1 -- widths computed from the data");
    let name_width = ENTRIES.iter().map(|e| e.name.chars().count()).max().unwrap_or(0);
    let size_width = ENTRIES
        .iter()
        .map(|e| e.bytes.to_string().len())
        .max()
        .unwrap_or(0);
    println!("   name column {name_width}, size column {size_width}");
    for e in &ENTRIES {
        println!("   {:<name_width$}  {:>size_width$}", e.name, e.bytes);
    }
    println!("   {:-<width$}", "", width = name_width + size_width + 2);
    let total: u64 = ENTRIES.iter().map(|e| e.bytes).sum();
    println!("   {:<name_width$}  {:>size_width$}", "total", total);
    println!("   `{{:<name_width$}}` reads the width from a binding in scope, so the");
    println!("   table resizes itself and no number is written down twice.");

    println!("\nRound 2 -- the column that refuses to line up");
    println!("   Quoting the names with {{:?}} and padding them:");
    for e in ENTRIES.iter().take(2) {
        println!("      [{:<12?}]", e.name);
    }
    println!("   Nothing padded. Debug for str never consults the width -- the spec");
    println!("   is state on the Formatter and each impl decides whether to read it.");
    println!("   Format FIRST, pad the result second:");
    for e in ENTRIES.iter().take(2) {
        println!("      [{:<12}]", format!("{:?}", e.name));
    }
    println!("   That is the general fix: `format!(\"{{x:?}}\")` produces a String,");
    println!("   and Display for String does honour the width.");

    println!("\nRound 3 -- truncating a cell, and what precision counts");
    let note_width = 18;
    for e in &ENTRIES {
        let note = if e.note.chars().count() > note_width {
            format!("{:.*}\u{2026}", note_width - 1, e.note)
        } else {
            e.note.to_string()
        };
        println!("   {:<name_width$}  {note}", e.name);
    }
    println!("   Precision on a string is a MAXIMUM LENGTH, and it counts chars:");
    let polish = "\u{17c}\u{f3}\u{142}w";
    println!("      {polish:?} is {} bytes, {} chars", polish.len(), polish.chars().count());
    println!("      {{:.3}} gives [{:.3}] -- three chars, not three bytes", polish);
    println!("      so it can never split a character in half, unlike &s[..3].");

    println!("\nRound 4 -- the alignment fmt cannot fix");
    for label in ["ab", "\u{17c}\u{f3}", "\u{1f600}\u{1f600}"] {
        println!(
            "      [{:<6}] {} chars, {} bytes",
            label,
            label.chars().count(),
            label.len()
        );
    }
    println!("   All three were padded to six, and all three are correct: fmt counts");
    println!("   CHARS. A terminal draws CELLS, and an emoji takes two of them, so");
    println!("   the last row is two columns wider on screen than the first. std has");
    println!("   no notion of display width and cannot -- it is a property of the");
    println!("   font and the terminal, not of the string. A table of arbitrary text");
    println!("   needs a width crate; a table of ASCII does not.");
}
