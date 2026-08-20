//! Kata solution: `_ =>` is a hole you punch in your own safety net.
//!
//! The election office adds a sixth mark — '%', spoiled and reissued. Two audit
//! functions were written before it existed. Only one of them tells you.
//!
//!   rustc --edition 2024 six_kinds_of_zero_kata.rs -o /tmp/skzk && /tmp/skzk

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Scored(u8),
    Blank,
    RaceAbstention,
    CandidateAbstention,
    Spoiled,
    SpoiledReissued, // <- added today
}

/// Written when there were five marks. The `_` arm looked harmless: every
/// remaining case really was a blank at the time.
fn bucket_lazy(m: Mark) -> &'static str {
    match m {
        Mark::Scored(_) => "scored",
        Mark::RaceAbstention => "abstention",
        Mark::CandidateAbstention => "abstention",
        Mark::Spoiled => "spoiled",
        _ => "blank",
    }
}

/// Written the same day, without the `_`. It did not compile this morning
/// until somebody chose an arm for the new mark — and chose "spoiled".
fn bucket_strict(m: Mark) -> &'static str {
    match m {
        Mark::Scored(_) => "scored",
        Mark::Blank => "blank",
        Mark::RaceAbstention => "abstention",
        Mark::CandidateAbstention => "abstention",
        Mark::Spoiled => "spoiled",
        Mark::SpoiledReissued => "spoiled",
    }
}

fn tally<'a>(marks: &[Mark], bucket: fn(Mark) -> &'a str) -> Vec<(&'a str, usize)> {
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
    let ballots = [
        Mark::Scored(5),
        Mark::Scored(3),
        Mark::Blank,
        Mark::RaceAbstention,
        Mark::Spoiled,
        Mark::SpoiledReissued,
        Mark::SpoiledReissued,
        Mark::CandidateAbstention,
    ];

    println!("Eight cells, two audit reports of the same stack:\n");
    println!("  with `_ => \"blank\"`   {:?}", tally(&ballots, bucket_lazy));
    println!("  exhaustive            {:?}", tally(&ballots, bucket_strict));

    println!("\n  The two reissued ballots landed in \"blank\" on the first report.");
    println!("  Nothing failed. It compiled, it ran, and it filed two spoiled");
    println!("  ballots as voters who declined to mark the box — which is the");
    println!("  difference between a printing fault and an electorate's opinion.");

    println!("\nWhat the second function did this morning, before it was fixed —");
    println!("real rustc output, not a paraphrase:\n");
    for line in [
        "error[E0004]: non-exhaustive patterns: `Mark::SpoiledReissued` not covered",
        "   |",
        "   |     match m {",
        "   |           ^ pattern `Mark::SpoiledReissued` not covered",
        "   |",
        "help: ensure that all possible cases are being handled by adding a match",
        "      arm with a wildcard pattern or an explicit pattern as shown",
        "   |",
        "   ~         Mark::Spoiled => \"spoiled\",",
        "   ~         Mark::SpoiledReissued => todo!(),",
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
