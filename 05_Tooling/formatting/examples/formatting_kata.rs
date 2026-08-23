//! Kata: the reformat that *did* change the program.
//!
//! `rustfmt` moved two spaces in this lesson's example and the output did not
//! budge, because whitespace outside a literal is inert. Inside a literal it is
//! data — and `rustfmt` will not touch it. So the one place re-indenting can
//! change what a program does is the one place the formatter refuses to go,
//! which means the tool that normally protects you is silent here by design.
//!
//! The counterexample below is a ballot block in a string, read by a rule that
//! cares about indentation. Line it up with the surrounding code and the
//! election empties out.

/// A ballot block exactly as it was pasted in: the header sits at column 0 and
/// its rows are indented under it.
const AS_PASTED: &str = r#"
ballots:
  5,2,0
  0,4,5
  2,5,4
"#;

/// The same block after somebody lined it up with the code around it. Every
/// line gained four spaces. `rustfmt` did not do this, and will not undo it.
const TIDIED: &str = r#"
    ballots:
      5,2,0
      0,4,5
      2,5,4
"#;

/// The smallest indentation-sensitive reader there is: a line at indent 0 opens
/// a section, and any line indented under it is one of that section's rows.
/// YAML, Python and Markdown all decide structure this same way.
fn read(block: &str) -> Vec<(&str, Vec<&str>)> {
    let mut sections: Vec<(&str, Vec<&str>)> = Vec::new();
    for line in block.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent == 0 {
            sections.push((line.trim_end(), Vec::new()));
        } else if let Some(open) = sections.last_mut() {
            open.1.push(line.trim());
        }
    }
    sections
}

fn ballots(block: &str) -> usize {
    read(block).iter().map(|(_, rows)| rows.len()).sum()
}

fn describe(label: &str, block: &str) {
    let sections = read(block);
    println!("{label}");
    println!("  bytes in the literal: {}", block.len());
    println!("  sections found:       {}", sections.len());
    for (header, rows) in &sections {
        println!("    {header} -> {} rows", rows.len());
    }
    println!("  ballots counted:      {}", ballots(block));
    println!();
}

fn main() {
    describe("as pasted", AS_PASTED);
    describe("tidied to line up with the surrounding code", TIDIED);

    println!(
        "same election, {} ballots vs {}",
        ballots(AS_PASTED),
        ballots(TIDIED)
    );
    println!("rustfmt changed neither block: a string literal is data, not code.");
}
