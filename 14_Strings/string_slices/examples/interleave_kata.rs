//! Kata solution: is `s3` an interleaving of `s1` and `s2`? The remaining
//! slices are the whole state — and one of the original five assertions was
//! wrong.
//!
//!   rustc --edition 2024 interleave_kata.rs -o /tmp/ik && /tmp/ik
//!   rustc --edition 2024 --test interleave_kata.rs -o /tmp/ikt && /tmp/ikt

use std::collections::HashMap;

fn is_interleave(s1: &str, s2: &str, s3: &str) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    let mut memo = HashMap::new();
    step(s1, s2, s3, &mut memo)
}

/// Take the next character of `c` from the front of `a` or of `b`. The three
/// remaining slices are the entire state, and because `c` is always exactly as
/// long as `a` and `b` together, the pair of remaining lengths names that state
/// — so the pair is the memo key, and no text is ever copied.
fn step(a: &str, b: &str, c: &str, memo: &mut HashMap<(usize, usize), bool>) -> bool {
    let Some(first) = c.chars().next() else {
        return true;
    };
    if let Some(&known) = memo.get(&(a.len(), b.len())) {
        return known;
    }
    let w = first.len_utf8();
    let answer = (a.starts_with(first) && step(&a[w..], b, &c[w..], memo))
        || (b.starts_with(first) && step(a, &b[w..], &c[w..], memo));
    memo.insert((a.len(), b.len()), answer);
    answer
}

/// Every interleaving of `a` and `b`, by brute force — for checking a claim.
fn all_interleavings(a: &str, b: &str) -> Vec<String> {
    let (Some(x), Some(y)) = (a.chars().next(), b.chars().next()) else {
        return vec![format!("{a}{b}")];
    };
    let mut out = Vec::new();
    for rest in all_interleavings(&a[x.len_utf8()..], b) {
        out.push(format!("{x}{rest}"));
    }
    for rest in all_interleavings(a, &b[y.len_utf8()..]) {
        out.push(format!("{y}{rest}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interleave() {
        assert!(is_interleave("aabcc", "dbbca", "aadbbcbcac"));
        assert!(!is_interleave("aabcc", "dbbca", "aadbbbaccc"));
        assert!(is_interleave("", "", ""));
        assert!(is_interleave("ab", "cd", "abcd"));
        // The original line was `assert!(!is_interleave("ab", "cd", "acbd"));`.
        // But a, c, b, d takes a and b from "ab" and c and d from "cd", each in
        // order, so it IS an interleaving; main() prints all six to show it.
        assert!(is_interleave("ab", "cd", "acbd"));
        assert!(!is_interleave("ab", "cd", "bacd"));
    }
}

fn main() {
    println!("1. The four original cases that were right");
    for (a, b, c, want) in [
        ("aabcc", "dbbca", "aadbbcbcac", true),
        ("aabcc", "dbbca", "aadbbbaccc", false),
        ("", "", "", true),
        ("ab", "cd", "abcd", true),
    ] {
        let got = is_interleave(a, b, c);
        assert_eq!(got, want);
        println!("   {:<8} {:<8} {:<13} {got}", format!("{a:?}"), format!("{b:?}"), format!("{c:?}"));
    }

    println!();
    println!("2. The fifth, which the original test got wrong");
    let all = all_interleavings("ab", "cd");
    println!("   every interleaving of \"ab\" and \"cd\" ({}): {all:?}", all.len());
    println!("   \"acbd\" is among them          {}", all.iter().any(|s| s == "acbd"));
    println!("   is_interleave(\"ab\", \"cd\", \"acbd\") = {}", is_interleave("ab", "cd", "acbd"));
    println!("   is_interleave(\"ab\", \"cd\", \"bacd\") = {}", is_interleave("ab", "cd", "bacd"));
    assert!(is_interleave("ab", "cd", "acbd"));
    assert!(!is_interleave("ab", "cd", "bacd"));
    println!("   The test asserted false for acbd, so a correct solution failed it.");
    println!("   Checking a specification against brute force is cheap while the");
    println!("   inputs are this small.");

    println!();
    println!("3. Slices advance by a character's width, not by one");
    println!("   is_interleave(\"żó\", \"łw\", \"żłów\") = {}", is_interleave("żó", "łw", "żłów"));
    assert!(is_interleave("żó", "łw", "żłów"));
    println!("   Every cut is first.len_utf8() bytes long, so a slice never lands");
    println!("   inside a letter and the memo key is still a pair of byte lengths.");
}
