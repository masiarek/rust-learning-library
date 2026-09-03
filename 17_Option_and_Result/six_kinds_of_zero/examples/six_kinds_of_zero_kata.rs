//! Kata solution: `_ =>` is a hole you punch in your own safety net.
//!
//! The survey team adds a sixth mark — '%', unreadable and corrected. Two audit
//! functions were written before it existed. Only one of them tells you.
//!
//!   rustc --edition 2024 six_kinds_of_zero_kata.rs -o /tmp/skzk && /tmp/skzk

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Rated(u8),
    Blank,
    SectionSkipped,
    Declined,
    Unreadable,
    Corrected, // <- added today
}

/// Written when there were five marks. The `_` arm looked harmless: every
/// remaining case really was a blank at the time.
fn bucket_lazy(m: Mark) -> &'static str {
    match m {
        Mark::Rated(_) => "rated",
        Mark::SectionSkipped => "skipped",
        Mark::Declined => "declined",
        Mark::Unreadable => "unreadable",
        _ => "blank",
    }
}

/// Written the same day, without the `_`. It did not compile this morning
/// until somebody chose an arm for the new mark — and chose "unreadable".
fn bucket_strict(m: Mark) -> &'static str {
    match m {
        Mark::Rated(_) => "rated",
        Mark::Blank => "blank",
        Mark::SectionSkipped => "skipped",
        Mark::Declined => "declined",
        Mark::Unreadable => "unreadable",
        Mark::Corrected => "unreadable",
    }
}

fn count<'a>(marks: &[Mark], bucket: fn(Mark) -> &'a str) -> Vec<(&'a str, usize)> {
    let mut out: Vec<(&str, usize)> = Vec::new();
    for m in marks {
        let b = bucket(*m);
        match out.iter_mut().find(|(name, _)| *name == b) {
            Some((_, n)) => *n += 1,
            None => out.push((b, 1)),
        }
    }
    out.sort_by_key(|(name, _)| *name);
    out
}

fn main() {
    let cells = [
        Mark::Rated(5),
        Mark::Rated(3),
        Mark::Blank,
        Mark::SectionSkipped,
        Mark::Unreadable,
        Mark::Corrected,
        Mark::Corrected,
        Mark::Declined,
    ];

    println!("Eight cells, two audit reports of the same stack:\n");
    println!("  with `_ => \"blank\"`   {:?}", count(&cells, bucket_lazy));
    println!("  exhaustive            {:?}", count(&cells, bucket_strict));

    println!("\n  The two corrected cells landed in \"blank\" on the first report.");
    println!("  Nothing failed. It compiled, it ran, and it filed two scanning");
    println!("  faults as people who chose to leave the box empty — which is the");
    println!("  difference between a hardware problem and an opinion.");

    println!("\nWhat the second function did this morning, before it was fixed —");
    println!("real rustc output, not a paraphrase:\n");
    for line in [
        "error[E0004]: non-exhaustive patterns: `Mark::Corrected` not covered",
        "  |",
        "3 |     match m {",
        "  |           ^ pattern `Mark::Corrected` not covered",
        "  |",
        "note: `Mark` defined here",
        "1 | enum Mark { Rated(u8), Blank, SectionSkipped, Declined, Unreadable, Corrected }",
        "  |      ^^^^                                                           --------- not covered",
        "help: ensure that all possible cases are being handled by adding a match",
        "      arm with a wildcard pattern or an explicit pattern as shown",
        "  |",
        "8 ~         Mark::Unreadable => \"unreadable\",",
        "9 ~         Mark::Corrected => todo!(),",
    ] {
        println!("    {line}");
    }

    println!("\n  Read the help text closely: rustc offers you the wildcard as a fix.");
    println!("  Taking it turns this compile error into the silent miscount above,");
    println!("  for every marker anyone adds afterwards. `_` is not a shortcut —");
    println!("  it is a standing promise that every FUTURE variant belongs in that");
    println!("  bucket, made on behalf of people who have not written them yet.");

    println!("\n  Where a wildcard IS right: matching on something genuinely open,");
    println!("  like a char from a file. `_ => return Err(other)` is a real answer");
    println!("  to 'any other character', because there is no finite list to name.");
}
