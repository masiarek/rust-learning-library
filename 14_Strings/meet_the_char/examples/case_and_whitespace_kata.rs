//! Kata solution: six transformations, and the two places ASCII case and
//! Unicode case stop being the same operation.
//!
//!   rustc --edition 2024 case_and_whitespace_kata.rs -o /tmp/cwk && /tmp/cwk

/// `MyVariableName` -> `my_variable_name`. An underscore goes before every
/// uppercase letter that is not the first character.
fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// `my_variable_name` -> `MyVariableName`. Split on the underscores, uppercase
/// the first character of each piece — and the first character may be two.
fn to_camel_case(s: &str) -> String {
    s.split('_')
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            let mut chars = piece.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Swap the case of the ASCII letters and leave everything else exactly as it
/// was — using the in-place `char` methods, which is what makes this ASCII-only.
fn swap_ascii_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            let mut c = c;
            if c.is_ascii_uppercase() {
                c.make_ascii_lowercase();
            } else if c.is_ascii_lowercase() {
                c.make_ascii_uppercase();
            }
            c
        })
        .collect()
}

/// Alternating case, counting only the letters — so a space does not consume a
/// turn and the pattern survives it.
fn spongebob(s: &str) -> String {
    let mut upper = false;
    s.chars()
        .map(|c| {
            if c.is_alphabetic() {
                upper = !upper;
                if upper {
                    c.to_uppercase().collect::<String>()
                } else {
                    c.to_lowercase().collect::<String>()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Same letters, ignoring case and whitespace.
fn is_anagram(a: &str, b: &str) -> bool {
    fn key(s: &str) -> Vec<char> {
        let mut v: Vec<char> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| c.to_lowercase())
            .collect();
        v.sort_unstable();
        v
    }
    key(a) == key(b)
}

fn main() {
    println!("1. trim");
    let messy = "\t  Ada Lovelace \n\r\n ";
    println!("   {messy:?}");
    println!("   .trim()       {:?}", messy.trim());
    println!("   .trim_start() {:?}", messy.trim_start());
    println!("   .trim_end()   {:?}", messy.trim_end());
    println!("   trim removes any Unicode whitespace, not just the space character:");
    println!("   {:?}.trim() = {:?}   <- U+00A0, a non-breaking space",
        "\u{a0}x\u{a0}", "\u{a0}x\u{a0}".trim());
    println!("   {:?}.trim_ascii() = {:?}   <- the ASCII-only version leaves it",
        "\u{a0}x\u{a0}", "\u{a0}x\u{a0}".trim_ascii());

    println!("\n2. CamelCase -> snake_case");
    for s in ["MyVariableName", "STARVoting", "Ballot", "alreadySnakeish"] {
        println!("   {:<16} -> {:?}", s, to_snake_case(s));
    }
    println!("   `STARVoting` shows the limit of the one-rule version: an acronym");
    println!("   becomes s_t_a_r_voting. Real converters carry a second rule for a");
    println!("   run of capitals — say so rather than pretending the rule is complete.");

    println!("\n3. snake_case -> CamelCase");
    for s in ["my_variable_name", "ballot", "top_two_runoff", "trailing_"] {
        println!("   {:<16} -> {:?}", s, to_camel_case(s));
    }
    println!("   Round trip: {:?} -> {:?} -> {:?}",
        "my_variable_name",
        to_camel_case("my_variable_name"),
        to_snake_case(&to_camel_case("my_variable_name")));
    println!("   It survives here, and does not for STARVoting — a converter pair is");
    println!("   only a round trip on the names both rules agree about.");

    println!("\n4. Swapping ASCII case only");
    for s in ["Hello World", "café AU LAIT", "ЖУРНАЛ"] {
        println!("   {:<14} -> {:?}", s, swap_ascii_case(s));
    }
    println!("   The é and the Cyrillic are untouched: make_ascii_*case is defined");
    println!("   to leave every non-ASCII byte alone. That is a promise, not a bug —");
    println!("   it is the one case conversion that cannot change a string's length.");
    println!("   The Unicode version can: {:?}.to_uppercase() = {:?} ({} chars)",
        'ß', 'ß'.to_uppercase().collect::<String>(), 'ß'.to_uppercase().count());

    println!("\n5. Spongebob case");
    for s in ["hello world", "star voting is good"] {
        println!("   {:?}", spongebob(s));
    }
    let naive: String = "hello world".chars().enumerate()
        .map(|(i, c)| if i % 2 == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() })
        .collect();
    println!("   Alternating on every char instead of every letter: {naive:?}");
    println!("   The space took a turn, so the two disagree from the w onward:");
    println!("   letters only {:?} against every char {:?}.",
        &spongebob("hello world")[6..], &naive[6..]);
    println!("   Which is right is a taste question; which one you WROTE should not be.");

    println!("\n6. Anagrams");
    for (a, b) in [
        ("Listen", "Silent"),
        ("The eyes", "They see"),
        ("Dormitory", "Dirty Room"),
        ("ballot", "ballots"),
    ] {
        println!("   {:<14} vs {:<14} {}", format!("{a:?}"), format!("{b:?}"), is_anagram(a, b));
    }
    println!("   Sorting chars is the whole trick, and it is why this is a `char`");
    println!("   exercise: sorting BYTES would compare halves of multibyte letters.");
}
