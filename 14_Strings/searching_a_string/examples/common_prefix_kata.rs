//! Kata solution: the longest common prefix — by characters, since two words
//! can share a first byte and not a first letter.
//!
//!   rustc --edition 2024 common_prefix_kata.rs -o /tmp/cpk && /tmp/cpk
//!   rustc --edition 2024 --test common_prefix_kata.rs -o /tmp/cpkt && /tmp/cpkt

/// Adam's signature, which hands back an owned `String`.
fn longest_common_prefix(strings: &[&str]) -> String {
    borrowed_prefix(strings).to_owned()
}

/// The same answer without allocating: a slice of the first word. `'a` says
/// the result borrows from the words themselves — not from the array that
/// holds them, which may be a temporary.
fn borrowed_prefix<'a>(strings: &[&'a str]) -> &'a str {
    let Some((&first, rest)) = strings.split_first() else {
        return "";
    };
    let mut end = first.len();
    for s in rest {
        end = first[..end]
            .char_indices()
            .zip(s.chars())
            .take_while(|&((_, a), b)| a == b)
            .last()
            .map_or(0, |((i, a), _)| i + a.len_utf8());
    }
    &first[..end]
}

/// The tempting version: count equal leading BYTES.
fn shared_bytes(a: &str, b: &str) -> usize {
    a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
        assert_eq!(longest_common_prefix(&["dog", "racecar", "car"]), "");
        assert_eq!(longest_common_prefix(&["interspecies", "interstellar", "interstate"]), "inters");
        assert_eq!(longest_common_prefix(&[]), "");
        assert_eq!(longest_common_prefix(&["single"]), "single");
    }
}

fn main() {
    println!("1. Adam's five cases");
    let cases: [(&[&str], &str); 5] = [
        (&["flower", "flow", "flight"], "fl"),
        (&["dog", "racecar", "car"], ""),
        (&["interspecies", "interstellar", "interstate"], "inters"),
        (&[], ""),
        (&["single"], "single"),
    ];
    for (words, want) in cases {
        let got = longest_common_prefix(words);
        assert_eq!(got, want);
        println!("   {:<47} -> {got:?}", format!("{words:?}"));
    }

    println!();
    println!("2. The borrowed version allocates nothing");
    let words = ["interspecies", "interstellar", "interstate"];
    let p = borrowed_prefix(&words);
    println!("   borrowed_prefix -> {p:?}");
    println!("   points into words[0]: {}", p.as_ptr() == words[0].as_ptr());
    println!("   One lifetime parameter is what lets a function hand back a view into");
    println!("   its caller's data instead of a copy of it.");

    println!();
    println!("3. Bytes are the wrong unit");
    for (a, b) in [("café", "cafè"), ("é", "è"), ("Łódź", "Łomża")] {
        let n = shared_bytes(a, b);
        println!(
            "   {a:?} / {b:?}: shared bytes {n}, a.get(..{n}) = {:?}, by characters {:?}",
            a.get(..n),
            borrowed_prefix(&[a, b])
        );
    }
    println!("   é is C3 A9 and è is C3 A8: the first byte agrees and the letter does");
    println!("   not. Slicing at that count is the panic `get` turns into None.");
    println!("   Where the shared bytes happen to end on a boundary, as with Ł, the two");
    println!("   answers agree, which is why a byte version passes every ASCII test.");
}
