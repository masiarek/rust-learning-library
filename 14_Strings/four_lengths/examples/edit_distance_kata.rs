//! Kata solution: Levenshtein distance in two rows — and the same pair of
//! words at different distances in characters and in bytes.
//!
//!   rustc --edition 2024 edit_distance_kata.rs -o /tmp/edk && /tmp/edk
//!   rustc --edition 2024 --test edit_distance_kata.rs -o /tmp/edkt && /tmp/edkt

fn edit_distance(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    levenshtein(&a, &b)
}

/// The classic table, kept to two rows: row i holds the distance from the
/// first i items of `a` to every prefix of `b`. Generic over the unit, so the
/// same function can measure bytes.
fn levenshtein<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for (i, x) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, y) in b.iter().enumerate() {
            let replace = prev[j] + usize::from(x != y);
            let delete = prev[j + 1] + 1;
            let insert = cur[j] + 1;
            cur[j + 1] = replace.min(delete).min(insert);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("abc", ""), 3);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("intention", "execution"), 5);
    }
}

fn main() {
    println!("1. Adam's six cases");
    for (a, b, want) in [
        ("kitten", "sitting", 3),
        ("", "", 0),
        ("abc", "abc", 0),
        ("abc", "", 3),
        ("", "abc", 3),
        ("intention", "execution", 5),
    ] {
        let got = edit_distance(a, b);
        assert_eq!(got, want);
        println!("   {:<11} {:<11} {got}", format!("{a:?}"), format!("{b:?}"));
    }

    println!();
    println!("2. Which unit is being edited");
    println!("   {:<10} {:<14} {:>5} {:>5}", "a", "b", "chars", "bytes");
    for (a, b) in [("kitten", "sitting"), ("café", "cafe"), ("żółw", "zolw"), ("caf\u{e9}", "cafe\u{301}")] {
        let chars = edit_distance(a, b);
        let bytes = levenshtein(a.as_bytes(), b.as_bytes());
        println!("   {:<10} {:<14} {chars:>5} {bytes:>5}", format!("{a:?}"), format!("{b:?}"));
    }
    println!("   ASCII agrees in both units. Here each two-byte letter that differs costs");
    println!("   one edit as a char and two as bytes, a replacement plus a deletion:");
    println!("   café/cafe is 1 against 2, and żółw/zolw is 3 against 6.");
    println!("   The last row is one word spelt two ways, precomposed é against e plus a");
    println!("   combining accent: 2 edits in chars, 3 in bytes, and not zero in either");
    println!("   until something normalizes both, which std does not do.");
}
