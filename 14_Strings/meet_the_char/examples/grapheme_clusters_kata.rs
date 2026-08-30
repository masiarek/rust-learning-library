//! Kata solution: the third ruler. `chars().rev()` is not "reverse the text",
//! and `chars().count()` is not "how many characters a reader sees".
//!
//!   rustc --edition 2024 grapheme_clusters_kata.rs -o /tmp/gck && /tmp/gck
//!
//! std has no grapheme segmentation, on purpose: the rules are a Unicode annex
//! (UAX #29) that changes with each Unicode release, so they live in a crate
//! that can ship on Unicode's schedule rather than Rust's. In real code:
//!
//!     use unicode_segmentation::UnicodeSegmentation;
//!     let n = s.graphemes(true).count();
//!     let back: String = s.graphemes(true).rev().collect();
//!
//! The segmenter below is NOT that. It handles exactly the five joiners this
//! page uses — combining marks, ZWJ sequences, variation selectors, skin-tone
//! modifiers and regional-indicator pairs — and nothing else. It is here so the
//! *shape* of the answer is visible; it is not a substitute for the crate.

/// A mark that attaches to the character before it.
fn is_extender(c: char) -> bool {
    matches!(c,
        '\u{0300}'..='\u{036F}'      // combining diacritical marks
        | '\u{1AB0}'..='\u{1AFF}'    // combining diacritical marks extended
        | '\u{20D0}'..='\u{20FF}'    // combining marks for symbols
        | '\u{FE00}'..='\u{FE0F}'    // variation selectors
        | '\u{1F3FB}'..='\u{1F3FF}') // skin-tone modifiers
}

fn is_regional_indicator(c: char) -> bool {
    ('\u{1F1E6}'..='\u{1F1FF}').contains(&c)
}

/// Split into clusters: a base character plus whatever attaches to it.
fn clusters(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let mut cluster = String::new();
        cluster.push(chars[i]);
        // A flag is exactly two regional indicators.
        if is_regional_indicator(chars[i])
            && i + 1 < chars.len()
            && is_regional_indicator(chars[i + 1])
        {
            cluster.push(chars[i + 1]);
            i += 2;
        } else {
            i += 1;
        }
        // Then absorb marks, and anything a zero-width joiner attaches.
        while i < chars.len() {
            if is_extender(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
            } else if chars[i] == '\u{200D}' && i + 1 < chars.len() {
                cluster.push(chars[i]);
                cluster.push(chars[i + 1]);
                i += 2;
            } else {
                break;
            }
        }
        out.push(cluster);
    }
    out
}

fn grapheme_count(s: &str) -> usize {
    clusters(s).len()
}

fn reverse_graphemes(s: &str) -> String {
    clusters(s).into_iter().rev().collect()
}

fn escaped(s: &str) -> String {
    s.chars().map(|c| c.escape_unicode().to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    // "café" written the second legal way: e + U+0301 COMBINING ACUTE ACCENT.
    let cafe = "cafe\u{301}";
    let family = "👨\u{200D}👩\u{200D}👧\u{200D}👦";
    let flag = "🇵🇱";

    println!("1. Three answers to \"how long is it\"");
    println!("   {:<12} {:>5} {:>6} {:>10}", "string", "bytes", "chars", "graphemes");
    for (label, s) in [("cafe+U+0301", cafe), ("family", family), ("flag", flag), ("plain ada", "ada")] {
        println!("   {:<12} {:>5} {:>6} {:>10}", label, s.len(), s.chars().count(), grapheme_count(s));
    }
    println!("   The family emoji is 4 people and 3 zero-width joiners: 7 chars, 1");
    println!("   thing a reader points at. len() and chars().count() are both right");
    println!("   answers to questions nobody asked.");

    println!("\n2. Reversing by char breaks it");
    println!("   original          {cafe:?}  -> renders as {cafe}");
    let by_char: String = cafe.chars().rev().collect();
    println!("   chars().rev()     {by_char:?}  -> renders as {by_char}");
    println!("   the accent is now FIRST, with no base character in front of it — so it");
    println!("   renders on its own, and would land on whatever this string is glued to:");
    println!("   {}", escaped(&by_char));

    println!("\n3. Reversing by grapheme keeps it");
    let by_cluster = reverse_graphemes(cafe);
    println!("   clusters          {:?}", clusters(cafe));
    println!("   reversed          {by_cluster:?}  -> renders as {by_cluster}");
    println!("   {}", escaped(&by_cluster));
    println!("   The é travelled as one unit, because the mark moved with its base.");

    println!("\n4. The same test on the emoji");
    let emoji = format!("{family}{flag}!");
    println!("   input             {emoji:?}");
    println!("   chars().rev()     {:?}", emoji.chars().rev().collect::<String>());
    println!("   graphemes rev     {:?}", reverse_graphemes(&emoji));
    println!("   The joiners survive, but the sequence does not: boy-girl-woman-man is");
    println!("   not a defined emoji sequence, so it renders as four separate people —");
    println!("   and the flag's two regional indicators swapped into a pair that is not");
    println!("   a country. Neither string is corrupt UTF-8; every char is still valid,");
    println!("   which is why nothing errors and the bug reaches a screen.");

    println!("\n5. What this segmenter is not");
    println!("   It knows five joiners. UAX #29 also covers Hangul syllable blocks,");
    println!("   Indic conjuncts, prepend characters and more, and the rules move with");
    println!("   each Unicode release — which is exactly why std does not have them.");
    println!("   Reach for unicode-segmentation the moment the input is not yours.");
}
