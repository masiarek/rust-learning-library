//! Kata solution: full justification — and the width that has to be counted
//! in characters, because a byte count breaks the lines in the wrong places.
//!
//!   rustc --edition 2024 justify_text_kata.rs -o /tmp/jtk && /tmp/jtk
//!   rustc --edition 2024 --test justify_text_kata.rs -o /tmp/jtkt && /tmp/jtkt

fn justify_text(words: &[&str], max_width: usize) -> Vec<String> {
    justify_by(words, max_width, |w| w.chars().count())
}

/// Greedy lines, then the spare spaces shared out, leftmost gaps first. The
/// unit of width is a parameter, so the same algorithm can be run with the
/// wrong one.
fn justify_by(words: &[&str], max_width: usize, width: fn(&str) -> usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut i = 0;
    while i < words.len() {
        let mut j = i + 1;
        let mut used = width(words[i]);
        while j < words.len() && used + 1 + width(words[j]) <= max_width {
            used += 1 + width(words[j]);
            j += 1;
        }
        let line = &words[i..j];
        let gaps = line.len() - 1;
        if j == words.len() || gaps == 0 {
            // The last line, or a line of one word: left-justify and pad.
            let text = line.join(" ");
            let pad = max_width.saturating_sub(width(&text));
            lines.push(format!("{text}{}", " ".repeat(pad)));
        } else {
            let letters: usize = line.iter().map(|w| width(w)).sum();
            let spaces = max_width - letters;
            let (each, extra) = (spaces / gaps, spaces % gaps);
            let mut out = String::with_capacity(max_width);
            for (k, w) in line.iter().enumerate() {
                out.push_str(w);
                if k < gaps {
                    out.push_str(&" ".repeat(each + usize::from(k < extra)));
                }
            }
            lines.push(out);
        }
        i = j;
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justify_text() {
        let words = ["This", "is", "an", "example", "of", "text", "justification."];
        let result = justify_text(&words, 16);

        assert_eq!(result[0], "This    is    an");
        assert_eq!(result[1], "example  of text");
        assert_eq!(result[2], "justification.  ");
    }
}

fn main() {
    println!("1. Adam's example, 16 wide");
    let words = ["This", "is", "an", "example", "of", "text", "justification."];
    let result = justify_text(&words, 16);
    assert_eq!(result[0], "This    is    an");
    assert_eq!(result[1], "example  of text");
    assert_eq!(result[2], "justification.  ");
    for line in &result {
        println!("   |{line}|");
    }
    println!("   The second line has 3 spare spaces for 2 gaps, so the left gap takes");
    println!("   the extra one.");
    assert_eq!(result[2], format!("{:<16}", "justification."));
    println!("   The last line is exactly what {{:<16}} produces — the one kind of line");
    println!("   the format mini-language can justify on its own.");

    println!();
    println!("2. The same algorithm with the width counted in bytes");
    let polish = ["Łódź", "żółw", "mak", "ser"];
    for (unit, lines) in [
        ("chars", justify_by(&polish, 10, |w| w.chars().count())),
        ("bytes", justify_by(&polish, 10, str::len)),
    ] {
        for line in &lines {
            println!("   {unit}  |{line}|  {} characters wide", line.chars().count());
        }
    }
    println!("   Counted in bytes, Łódź and żółw look 7 wide each, so they no longer");
    println!("   share a line, and the padding comes out 3 characters short.");
}
