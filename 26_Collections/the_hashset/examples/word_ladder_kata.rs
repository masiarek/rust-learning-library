//! Kata solution: a breadth-first word ladder over a `HashSet<&str>` — the
//! set borrows the caller's words, and a lookup made with a scratch buffer
//! hands back the set's own `&str`.
//!
//!   rustc --edition 2024 word_ladder_kata.rs -o /tmp/wlk && /tmp/wlk
//!   rustc --edition 2024 --test word_ladder_kata.rs -o /tmp/wlkt && /tmp/wlkt

use std::collections::{HashSet, VecDeque};

/// Breadth first: every word one change away is tried before any word two
/// changes away, so the first time `end_word` is reached the count is the
/// shortest. Assumes lowercase ASCII words of equal length — the candidates
/// are made by overwriting one byte at a time.
fn word_ladder_length(begin_word: &str, end_word: &str, word_list: &[&str]) -> i32 {
    let mut unseen: HashSet<&str> = word_list.iter().copied().collect();
    if !unseen.contains(end_word) {
        return 0;
    }
    unseen.remove(begin_word);
    let mut frontier = VecDeque::from([(begin_word, 1)]);
    while let Some((word, steps)) = frontier.pop_front() {
        let mut buf = word.as_bytes().to_vec();
        for i in 0..buf.len() {
            let original = buf[i];
            for b in b'a'..=b'z' {
                buf[i] = b;
                let candidate = std::str::from_utf8(&buf).expect("ASCII in, ASCII out");
                // `take` looks up with the temporary and returns the stored
                // `&str`: a borrow of the caller's list, not of `buf`.
                if let Some(next) = unseen.take(candidate) {
                    if next == end_word {
                        return steps + 1;
                    }
                    frontier.push_back((next, steps + 1));
                }
            }
            buf[i] = original;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_ladder() {
        assert_eq!(
            word_ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );

        assert_eq!(
            word_ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log"]),
            0 // no possible transformation
        );

        assert_eq!(word_ladder_length("a", "b", &["b"]), 2);
    }
}

fn main() {
    println!("1. Adam's three cases");
    let list = ["hot", "dot", "dog", "lot", "log", "cog"];
    let with_cog = word_ladder_length("hit", "cog", &list);
    let without = word_ladder_length("hit", "cog", &list[..5]);
    let one_letter = word_ladder_length("a", "b", &["b"]);
    assert_eq!((with_cog, without, one_letter), (5, 0, 2));
    println!("   hit -> cog, cog in the list       {with_cog}");
    println!("   hit -> cog, cog missing           {without}   (0 means no ladder)");
    println!("   a -> b                            {one_letter}");
    println!("   The count includes both ends: hit hot dot dog cog is five words.");

    println!();
    println!("2. Which &str a lookup hands back");
    let owned = vec![String::from("hot"), String::from("dot")];
    let set: HashSet<&str> = owned.iter().map(String::as_str).collect();
    let probe = String::from("hot");
    let found = set.get(probe.as_str()).expect("hot is in the set");
    println!("   the answer points into the list   {}", found.as_ptr() == owned[0].as_ptr());
    println!("   the answer points into the probe  {}", found.as_ptr() == probe.as_ptr());
    println!("   A HashSet<&str> can be searched with any &str, here one built in a");
    println!("   scratch buffer, and it answers with the key it stored. So the frontier");
    println!("   holds borrows of the caller's words, and no String is made per visit.");

    println!();
    println!("3. The assumption the one-byte overwrite makes");
    let mut bytes = "żab".as_bytes().to_vec();
    bytes[0] = b'a';
    println!("   \"żab\" is {} bytes for 3 letters; overwrite byte 0 and from_utf8 is_err = {}",
        "żab".len(),
        std::str::from_utf8(&bytes).is_err()
    );
    println!("   One byte per letter holds for Adam's words and fails for Polish ones;");
    println!("   a ladder over real words would walk a Vec<char> instead.");
}
