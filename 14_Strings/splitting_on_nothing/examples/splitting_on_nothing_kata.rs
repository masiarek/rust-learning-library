//! Kata solution: three questions about the empty pattern, each answered by a
//! program rather than from memory — the two counts an empty haystack gives,
//! the byte offsets a Polish word does NOT report, and every character as a
//! borrowed &str with no allocation and no empties.
//!
//! Run:  rustc --edition 2024 splitting_on_nothing_kata.rs && ./splitting_on_nothing_kata

/// Every character as a borrowed slice of `s`. No `String`, no allocation per
/// character, and no empty pieces to filter out afterwards — `char_indices`
/// gives the start, `len_utf8` gives the width, and the two make the range.
fn chars_as_strs(s: &str) -> Vec<&str> {
    s.char_indices().map(|(i, c)| &s[i..i + c.len_utf8()]).collect()
}

fn main() {
    println!("Q1. Two empty haystacks, two different counts");
    println!("    \"\".split(',')  -> {:?}   0 matches, 1 piece",
             "".split(',').collect::<Vec<&str>>());
    println!("    \"\".split(\"\")   -> {:?}   1 match,  2 pieces",
             "".split("").collect::<Vec<&str>>());
    println!("    Neither is the odd one out: both are n+1. The empty pattern");
    println!("    matches once, because even an empty string has one position.");

    println!();
    println!("Q2. Four letters, seven bytes, five positions");
    let w = "żółw";
    println!("    len() = {} bytes, chars().count() = {}", w.len(), w.chars().count());
    println!("    match_indices(\"\") -> {:?}",
             w.match_indices("").map(|(i, _)| i).collect::<Vec<usize>>());
    println!("    split(\"\")         -> {:?}  ({} pieces)",
             w.split("").collect::<Vec<&str>>(), w.split("").count());
    let inside: Vec<usize> = (0..=w.len()).filter(|&i| !w.is_char_boundary(i)).collect();
    println!("    missing: {:?} — each one is inside a two-byte letter, and a", inside);
    println!("    piece starting there would not be UTF-8, so it cannot be a &str.");

    println!();
    println!("Q3. Every character as a &str");
    for s in ["żółw", "a\u{301}", ""] {
        let mine = chars_as_strs(s);
        let naive = s.split("").collect::<Vec<&str>>();
        println!("    {:<12} chars_as_strs -> {:<26} split(\"\") -> {:?}",
                 format!("{s:?}"), format!("{mine:?}"), naive);
        assert_eq!(mine.len(), s.chars().count());
    }
    println!("    split(\"\") alone is wrong on all three: it adds an empty at each");
    println!("    end, so it answers 2 for the empty string, which has no characters.");
}
