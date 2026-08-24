//! Kata solution: the signature decides who pays.
//!
//!   rustc --edition 2024 string_vs_str_kata.rs -o /tmp/svsk && /tmp/svsk

/// Borrow in, borrow out: the answer is a window into the caller's own text.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// The same job with an owning signature — every caller must now surrender a String.
fn first_word_owned(s: String) -> String {
    s.split_whitespace().next().unwrap_or("").to_string()
}

fn main() {
    println!("Round 1 — fn first_word(s: &str) -> &str");
    let literal = "score then automatic runoff";
    let owned = String::from("equal preference is allowed");
    println!("   from a literal        {:?}", first_word(literal));
    println!("   from a String         {:?}   <- &owned, coerced for free", first_word(&owned));
    println!("   from a slice of one   {:?}", first_word(&owned[6..]));
    println!("   and `owned` is still usable afterwards: {} bytes", owned.len());

    println!("\nRound 2 — fn first_word_owned(s: String) -> String");
    println!("   from a literal        {:?}   <- had to allocate with .to_string()", first_word_owned(literal.to_string()));
    println!("   from a String         {:?}   <- had to .clone() to keep `owned`", first_word_owned(owned.clone()));
    println!("   from a slice of one   {:?}   <- allocate again", first_word_owned(owned[6..].to_string()));
    println!("   or hand it over:      {:?}   <- moved; `owned` is gone (E0382 next use)", first_word_owned(owned));

    println!("\nThe catalogue:");
    println!("   &str parameter:   every caller pays nothing — a literal, a String,");
    println!("                     and a slice all coerce or borrow for free.");
    println!("   String parameter: every caller pays — an allocation, a clone, or");
    println!("                     the value itself. Take &str unless you must own it.");
}
