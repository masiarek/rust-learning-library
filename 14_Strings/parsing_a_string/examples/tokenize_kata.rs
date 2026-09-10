//! Kata solution: a tokenizer that splits on several delimiters and keeps a
//! quoted run together — plus the unterminated quote a `Vec` cannot report.
//!
//!   rustc --edition 2024 tokenize_kata.rs -o /tmp/tk && /tmp/tk
//!   rustc --edition 2024 --test tokenize_kata.rs -o /tmp/tkt && /tmp/tkt

/// Walks the input by `char`, so a delimiter may be any character, not only
/// an ASCII one. Inside double quotes delimiters are ordinary text, and the
/// quotes themselves are dropped. A run of delimiters yields no empty tokens,
/// but a quoted empty string ("") is a token: the quotes are what make it one.
fn tokenize(input: &str, delimiters: &[char]) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut started = false;
    for c in input.chars() {
        if c == '"' {
            quoted = !quoted;
            started = true;
        } else if !quoted && delimiters.contains(&c) {
            if started {
                tokens.push(std::mem::take(&mut current));
                started = false;
            }
        } else {
            current.push(c);
            started = true;
        }
    }
    if started {
        tokens.push(current);
    }
    tokens
}

/// The same walk, except an unterminated quote is an error naming where it
/// opened — the failure Adam's signature has no way to express.
fn try_tokenize(input: &str, delimiters: &[char]) -> Result<Vec<String>, String> {
    let mut open_at = None;
    for (i, c) in input.char_indices() {
        if c == '"' {
            open_at = if open_at.is_some() { None } else { Some(i) };
        }
    }
    match open_at {
        Some(i) => Err(format!("unterminated quote opened at byte {i}")),
        None => Ok(tokenize(input, delimiters)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let delims = vec![' ', ',', ';'];
        assert_eq!(
            tokenize("hello,world;rust lang", &delims),
            vec!["hello", "world", "rust", "lang"]
        );

        assert_eq!(
            tokenize("hello \"rust lang\" world", &[' ']),
            vec!["hello", "rust lang", "world"]
        );

        assert_eq!(tokenize("", &delims), Vec::<String>::new());
    }
}

fn main() {
    println!("1. Adam's three cases");
    let delims = [' ', ',', ';'];
    let a = tokenize("hello,world;rust lang", &delims);
    let b = tokenize("hello \"rust lang\" world", &[' ']);
    let c = tokenize("", &delims);
    assert_eq!(a, vec!["hello", "world", "rust", "lang"]);
    assert_eq!(b, vec!["hello", "rust lang", "world"]);
    assert_eq!(c, Vec::<String>::new());
    println!("   {a:?}");
    println!("   {b:?}");
    println!("   {c:?}");

    println!();
    println!("2. Three rules the tests leave to you");
    for input in ["a,,b", "a \"\" b", "ab\"c d\"e"] {
        println!("   {:<14} -> {:?}", format!("{input:?}"), tokenize(input, &delims));
    }
    println!("   Adam's cases pass whichever way these go, so each one is a decision:");
    println!("   an empty run vanishes, an empty quoted token stays, and a quote in the");
    println!("   middle of a word only switches the delimiter rule off and back on.");

    println!();
    println!("3. The failure the return type cannot hold");
    let input = "a \"b c";
    println!("   tokenize     -> {:?}", tokenize(input, &[' ']));
    println!("   try_tokenize -> {:?}", try_tokenize(input, &[' ']));
    println!("   A Vec<String> can only say what it found. A missing closing quote is");
    println!("   the input being wrong, and only a Result can say that.");

    println!();
    println!("4. A delimiter wider than a byte");
    println!("   {:?}", tokenize("αβγ·δε·ζ", &['·']));
}
