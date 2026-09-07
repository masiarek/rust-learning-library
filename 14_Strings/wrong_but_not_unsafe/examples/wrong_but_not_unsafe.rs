//! Two string bugs that never crashed anything — they just answered wrong.

fn main() {
    // ---------------------------------------------------------------- 1 ----
    // rust-lang/rust#16589, filed 2014-08-18: `"bananas".contains("nana")`
    // was `false`, and it was the only substring of "bananas" for which the
    // answer was wrong. The sweep below is the one from the bug report.
    println!("#16589  \"bananas\".contains(..)  — every substring, 2014 and now");
    let word = "bananas";
    let mut tested = 0;
    let mut not_found: Vec<&str> = Vec::new();
    for i in 0..word.len() {
        for j in i + 1..=word.len() {
            let part = &word[i..j];
            tested += 1;
            if !word.contains(part) {
                not_found.push(part);
            }
        }
    }
    println!("  substrings tested          {tested}");
    println!("  reported missing           {not_found:?}");
    println!("  contains(\"nana\")           {}", word.contains("nana"));
    println!("  find(\"nana\")               {:?}", word.find("nana"));

    // ---------------------------------------------------------------- 2 ----
    // rust-lang/rust#124714, filed 2024-05-04: `to_lowercase` put a
    // non-final sigma at the end of a word, but only when the ASCII fast
    // path had already eaten every character before it.
    println!();
    println!("#124714  to_lowercase and the final sigma");
    for word in ["ΣΣ", "ΟΔΟΣ", "aΣ", "abcdefghijklmnopΣ"] {
        println!("  {word:>20}  ->  {}", word.to_lowercase());
    }
    println!("  {:>20}  ->  {}", "'Σ' on its own", 'Σ'.to_lowercase());

    // The input shape the bug needed: an ASCII run, then the sigma. On the
    // affected releases the answer flipped back to σ once the ASCII fast path
    // had swallowed a whole number of its chunks.
    println!();
    println!("  k letters, then Σ — does the word end in ς?");
    let wrong: Vec<usize> = (1..=32)
        .filter(|k| !("a".repeat(*k) + "Σ").to_lowercase().ends_with('ς'))
        .collect();
    println!("    k in 1..=32, ending in σ   {wrong:?}");
    println!("    k = 0, no word at all      {:?}", "Σ".to_lowercase());
}
