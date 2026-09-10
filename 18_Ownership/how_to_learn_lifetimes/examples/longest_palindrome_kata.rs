//! Kata solution: the longest palindromic substring, returned as a slice of
//! the input — one reference in, one out, and lifetime elision ties them.
//!
//!   rustc --edition 2024 longest_palindrome_kata.rs -o /tmp/lpk && /tmp/lpk
//!   rustc --edition 2024 --test longest_palindrome_kata.rs -o /tmp/lpkt && /tmp/lpkt

/// Expand around every centre — each character for odd lengths, each gap for
/// even ones — and keep the FIRST longest, which is why "babad" gives "bab"
/// and not "aba". Positions are char indices; the final slice converts them to
/// byte offsets taken from `char_indices`, so it cannot split a letter.
fn longest_palindrome(s: &str) -> &str {
    let chars: Vec<char> = s.chars().collect();
    let offsets: Vec<usize> = s.char_indices().map(|(i, _)| i).chain([s.len()]).collect();
    let (mut start, mut end) = (0, 0);
    for centre in 0..chars.len() {
        for (mut lo, mut hi) in [(centre, centre + 1), (centre, centre)] {
            while lo > 0 && hi < chars.len() && chars[lo - 1] == chars[hi] {
                lo -= 1;
                hi += 1;
            }
            if hi - lo > end - start {
                (start, end) = (lo, hi);
            }
        }
    }
    &s[offsets[start]..offsets[end]]
}

/// Two inputs, one borrowed result: now elision has two references it could
/// tie the output to, and gives up. Without `<'a>` this signature is E0106.
fn longer_palindrome<'a>(a: &'a str, b: &'a str) -> &'a str {
    let (pa, pb) = (longest_palindrome(a), longest_palindrome(b));
    if pb.chars().count() > pa.chars().count() { pb } else { pa }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad"), "bab"); // or "aba"
        assert_eq!(longest_palindrome("cbbd"), "bb");
        assert_eq!(longest_palindrome("a"), "a");
        assert_eq!(longest_palindrome(""), "");
        assert_eq!(longest_palindrome("racecar"), "racecar");
    }
}

fn main() {
    println!("1. Adam's five cases");
    for (s, want) in [("babad", "bab"), ("cbbd", "bb"), ("a", "a"), ("", ""), ("racecar", "racecar")] {
        let got = longest_palindrome(s);
        assert_eq!(got, want);
        println!("   {:<11} -> {got:?}", format!("{s:?}"));
    }
    println!("   \"babad\" holds both bab and aba, three long each. The comment in the");
    println!("   test says either will do; the assert_eq! accepts only bab, so a tie");
    println!("   has to keep the first one found: > in the comparison, not >=.");

    println!();
    println!("2. The answer is a view into the input, not a copy");
    let text = String::from("xyracecarzw");
    let p = longest_palindrome(&text);
    let offset = p.as_ptr() as usize - text.as_ptr() as usize;
    println!("   longest_palindrome({text:?}) = {p:?}, starting at byte {offset} of the input");
    println!("   fn longest_palindrome(s: &str) -> &str needs no lifetime written:");
    println!("   one reference goes in, so elision ties the result to it.");

    println!();
    println!("3. Two inputs, and the annotation elision cannot supply");
    println!("   longer_palindrome(\"noon\", \"level\") = {:?}", longer_palindrome("noon", "level"));
    println!("   fn longer_palindrome<'a>(a: &'a str, b: &'a str) -> &'a str");
    println!("   Delete the <'a> and rustc stops with E0106: the result could borrow");
    println!("   from either argument, and the signature has to say which it may.");

    println!();
    println!("4. Characters, so no slice lands inside a letter");
    for s in ["été", "kajak", "ąbą"] {
        println!("   {:<8} -> {:?}", format!("{s:?}"), longest_palindrome(s));
    }
}
