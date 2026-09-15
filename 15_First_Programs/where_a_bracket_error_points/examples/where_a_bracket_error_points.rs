//! Where a bracket error points.
//!
//! rustc pairs every `(`, `[` and `{` in a file with its closing partner. This
//! is a MODEL of that pairing, not rustc's code: the smallest set of rules that
//! prints the same message, naming the same brackets at the same line:column,
//! as rustc 1.98.0 does for the five files in `../refusals/`. They are read
//! with `include_str!`, so the model sees exactly the bytes rustc saw.
//!
//!   rustc --edition 2024 where_a_bracket_error_points.rs -o /tmp/wabep && /tmp/wabep

const FILES: [(&str, &str); 5] = [
    ("extra_paren.rs", include_str!("../refusals/extra_paren.rs")),
    ("far_away.rs", include_str!("../refusals/far_away.rs")),
    ("missing_paren.rs", include_str!("../refusals/missing_paren.rs")),
    ("wrong_kind.rs", include_str!("../refusals/wrong_kind.rs")),
    ("missing_brace.rs", include_str!("../refusals/missing_brace.rs")),
];

/// One bracket and where it sits. Line and column both start at 1, as in rustc.
#[derive(Clone, Copy)]
struct Bracket {
    ch: char,
    line: usize,
    col: usize,
}

enum Verdict {
    /// Every bracket found its partner.
    Balanced,
    /// A closer arrived with nothing open at all. `stray` is the earlier closer
    /// that took the nearest opener, because nothing open was its kind.
    Unexpected {
        closer: Bracket,
        stray: Option<(Bracket, Bracket)>,
    },
    /// A closer met an opener of another kind.
    Mismatched {
        closer: Bracket,
        unclosed: Bracket,
        meant_for: Option<Bracket>,
    },
    /// The file ended with an opener still waiting.
    Unclosed { opener: Bracket },
}

fn partner(close: char) -> char {
    match close {
        ')' => '(',
        ']' => '[',
        _ => '{',
    }
}

fn check(src: &str) -> Verdict {
    let mut open: Vec<Bracket> = Vec::new();
    let mut stray: Option<(Bracket, Bracket)> = None; // (the opener it took, the closer)
    let mut in_string = false;
    let mut escaped = false;

    for (i, text) in src.lines().enumerate() {
        for (j, ch) in text.chars().enumerate() {
            let here = Bracket { ch, line: i + 1, col: j + 1 };
            if in_string {
                // Inside "..." a bracket is only text: `"{n}"` opens nothing.
                if escaped {
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    in_string = false;
                }
                continue;
            }
            match ch {
                '"' => in_string = true,
                '(' | '[' | '{' => open.push(here),
                ')' | ']' | '}' => {
                    let want = partner(ch);
                    let Some(&top) = open.last() else {
                        return Verdict::Unexpected { closer: here, stray };
                    };
                    if top.ch == want {
                        open.pop();
                    } else if let Some(&outer) = open.iter().rev().find(|b| b.ch == want) {
                        // Its partner IS open, further out: what is open inside it was never closed.
                        return Verdict::Mismatched { closer: here, unclosed: top, meant_for: Some(outer) };
                    } else {
                        // Nothing open is its kind. It closes the nearest opener anyway,
                        // and the damage shows up later, at a closer with nothing left.
                        open.pop();
                        stray = stray.or(Some((top, here)));
                    }
                }
                _ => {}
            }
        }
    }

    if let Some((top, closer)) = stray {
        return Verdict::Mismatched { closer, unclosed: top, meant_for: None };
    }
    match open.last() {
        Some(&opener) => Verdict::Unclosed { opener },
        None => Verdict::Balanced,
    }
}

fn label(b: Bracket, says: &str) {
    let at = format!("`{}` at {}:{}", b.ch, b.line, b.col);
    println!("    {at:<12} {says}");
}

fn report(verdict: Verdict) {
    match verdict {
        Verdict::Balanced => println!("  every bracket has its partner"),
        Verdict::Unexpected { closer, stray } => {
            println!("  error: unexpected closing delimiter: `{}`", closer.ch);
            println!("  --> {}:{}", closer.line, closer.col);
            if let Some((nearest, extra)) = stray {
                label(nearest, "the nearest open delimiter");
                label(extra, &format!("missing open `{}` for this delimiter", partner(extra.ch)));
            }
            label(closer, "unexpected closing delimiter");
        }
        Verdict::Mismatched { closer, unclosed, meant_for } => {
            println!("  error: mismatched closing delimiter: `{}`", closer.ch);
            println!("  --> {}:{}", unclosed.line, unclosed.col);
            if let Some(outer) = meant_for {
                label(outer, "closing delimiter possibly meant for this");
            }
            label(unclosed, "unclosed delimiter");
            label(closer, "mismatched closing delimiter");
        }
        Verdict::Unclosed { opener } => {
            println!("  error: this file contains an unclosed delimiter");
            println!("  --> end of file");
            label(opener, "unclosed delimiter");
        }
    }
}

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    for (name, src) in FILES {
        banner(name);
        report(check(src));
    }

    banner("extra_paren.rs with the ) the label points at deleted");
    let fixed = FILES[0].1.replacen("to_string());", "to_string();", 1);
    assert_ne!(fixed, FILES[0].1, "the replacement must have found the stray )");
    report(check(&fixed));
    println!("  This check has nothing left to say, so rustc goes on to the");
    println!("  next mistake in the file: `println!(s)` on line 3.");
}
