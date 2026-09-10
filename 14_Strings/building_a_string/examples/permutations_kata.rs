//! Kata solution: every distinct permutation, built in one buffer that grows
//! with `push` and shrinks with `pop`.
//!
//!   rustc --edition 2024 permutations_kata.rs -o /tmp/pk && /tmp/pk
//!   rustc --edition 2024 --test permutations_kata.rs -o /tmp/pkt && /tmp/pkt

/// The characters are sorted first, so equal letters sit side by side; a
/// letter is then skipped while its twin to the left is still unused, which
/// is the one rule that stops "aa" from coming out twice. The results arrive
/// already unique and already in sorted order — no set needed.
fn permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    let mut used = vec![false; chars.len()];
    let mut buf = String::with_capacity(s.len());
    let mut out = Vec::new();
    extend(&chars, &mut used, &mut buf, 0, &mut out);
    out
}

/// One level of the recursion: try each unused letter at position `depth`,
/// then take it back off the buffer before trying the next.
fn extend(chars: &[char], used: &mut [bool], buf: &mut String, depth: usize, out: &mut Vec<String>) {
    if depth == chars.len() {
        out.push(buf.clone());
        return;
    }
    for i in 0..chars.len() {
        if used[i] || (i > 0 && chars[i] == chars[i - 1] && !used[i - 1]) {
            continue;
        }
        used[i] = true;
        buf.push(chars[i]);
        extend(chars, used, buf, depth + 1, out);
        buf.pop();
        used[i] = false;
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let mut perms = permutations("abc");
        perms.sort();
        assert_eq!(perms, vec!["abc", "acb", "bac", "bca", "cab", "cba"]);

        let mut perms = permutations("aa");
        perms.sort();
        assert_eq!(perms, vec!["aa"]); // no duplicates

        assert_eq!(permutations("").len(), 1);
    }
}

fn main() {
    println!("1. Adam's three cases");
    let mut perms = permutations("abc");
    perms.sort();
    assert_eq!(perms, vec!["abc", "acb", "bac", "bca", "cab", "cba"]);
    println!("   \"abc\" -> {perms:?}");
    let mut perms = permutations("aa");
    perms.sort();
    assert_eq!(perms, vec!["aa"]);
    println!("   \"aa\"  -> {perms:?}");
    assert_eq!(permutations("").len(), 1);
    println!("   \"\"    -> {:?}, one arrangement of nothing rather than none", permutations(""));

    println!();
    println!("2. How many, and the repeats the skip rule removes");
    println!("   {:<9} {:>6} {:>9} {:>9}", "input", "n!", "distinct", "returned");
    for s in ["abcd", "aabb", "banana", "aaaa"] {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let (mut repeats, mut run) = (1, 1);
        for pair in chars.windows(2) {
            if pair[0] == pair[1] {
                run += 1;
            } else {
                repeats *= factorial(run);
                run = 1;
            }
        }
        repeats *= factorial(run);
        let distinct = factorial(chars.len()) / repeats;
        let got = permutations(s).len();
        assert_eq!(got, distinct);
        println!("   {:<9} {:>6} {:>9} {:>9}", format!("{s:?}"), factorial(chars.len()), distinct, got);
    }
    println!("   distinct = n! divided by the factorial of each letter's count, and the");
    println!("   skip rule produces exactly that many without generating the rest.");

    println!();
    println!("3. Permute characters, not bytes");
    println!("   \"ée\" -> {:?}", permutations("ée"));
    println!(
        "   the two bytes of \"é\" swapped: String::from_utf8 is_err = {}",
        String::from_utf8(vec![0xA9, 0xC3]).is_err()
    );
    println!("   A permutation of bytes can be something that is not text at all.");
}
