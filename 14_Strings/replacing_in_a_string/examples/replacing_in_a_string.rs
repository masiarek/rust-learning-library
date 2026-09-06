//! Replacing part of a string: what `replace` actually returns, the two methods
//! that really do edit in place, and the ordering bug a chain of replaces walks
//! straight into.
//!
//!   rustc --edition 2024 replacing_in_a_string.rs -o /tmp/ris && /tmp/ris

use std::borrow::Cow;

/// A substitution table applied in ONE pass: at each position, the first rule
/// that matches wins and the scan continues past what it produced, so nothing
/// a rule writes is ever seen by another rule.
fn substitute(text: &str, table: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    'scan: while !rest.is_empty() {
        for (from, to) in table {
            // An empty needle matches everywhere and would never advance `rest`.
            if from.is_empty() {
                continue;
            }
            if let Some(tail) = rest.strip_prefix(from) {
                out.push_str(to);
                rest = tail;
                continue 'scan;
            }
        }
        let c = rest.chars().next().unwrap();
        out.push(c);
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// Only pay for a new `String` when there is something to change.
fn tabs_to_spaces(s: &str) -> Cow<'_, str> {
    if s.contains('\t') {
        Cow::Owned(s.replace('\t', "    "))
    } else {
        Cow::Borrowed(s)
    }
}

fn main() {
    println!("1. replace does not replace -- it returns a new String");
    let original = "config.old.toml";
    let renamed = original.replace("old", "new");
    println!("   original  {original:?}   <- untouched");
    println!("   renamed   {renamed:?}");
    println!("   The receiver is a &str and a str cannot change length, so there");
    println!("   is nowhere for an edit to go. The answer comes back as a String.");

    println!("\n2. Every occurrence, unless you say how many");
    let csv = "a,b,c,d";
    println!("   replace(',', \";\")       {:?}", csv.replace(',', ";"));
    println!("   replacen(',', \";\", 1)   {:?}", csv.replacen(',', ";", 1));
    println!("   replacen(',', \";\", 99)  {:?}", csv.replacen(',', ";", 99));
    println!("   There is no rreplacen: the count always runs from the left.");

    println!("\n3. The needle is a Pattern; the replacement is plain text");
    let noisy = "sda1 sdb2 sdc3";
    println!("   char      {:?}", noisy.replace(' ', " | "));
    println!("   &str      {:?}", noisy.replace("sd", "hd"));
    println!("   [char]    {:?}", noisy.replace(['1', '2', '3'], "N"));
    println!("   closure   {:?}", noisy.replace(char::is_numeric, "N"));
    println!("   What you cannot do is refer to what matched: the replacement is a");
    println!("   &str, not a template. No $1, no backreferences -- that is a regex");
    println!("   engine, and std does not ship one.");

    println!("\n4. It allocates even when nothing matched");
    for s in ["one\ttwo", "one two"] {
        let out = tabs_to_spaces(s);
        let kind = match out {
            Cow::Borrowed(_) => "borrowed  <- no allocation",
            Cow::Owned(_) => "owned     <- a new String",
        };
        println!("   {:<10} {}", format!("{s:?}"), kind);
    }
    println!("   A bare replace() would have allocated on both rows. contains() is");
    println!("   the cheap question that lets the second row skip the copy.");

    println!("\n5. A chain of replaces is not simultaneous");
    let raw = "a < b & c";
    let wrong = raw.replace('<', "&lt;").replace('&', "&amp;");
    let right = raw.replace('&', "&amp;").replace('<', "&lt;");
    println!("   raw      {raw:?}");
    println!("   < first  {wrong:?}   <- the & it just wrote got escaped again");
    println!("   & first  {right:?}");
    println!("   Ordering saved this pair. It does not scale: each new rule has to");
    println!("   be checked against the output of every earlier one.");

    println!("\n6. One pass, and the order stops mattering");
    let table = [("&", "&amp;"), ("<", "&lt;"), (">", "&gt;")];
    let reversed = [(">", "&gt;"), ("<", "&lt;"), ("&", "&amp;")];
    println!("   substitute(raw, table)     {:?}", substitute(raw, &table));
    println!("   substitute(raw, reversed)  {:?}", substitute(raw, &reversed));
    println!("   Same answer both ways, because the scan moves past what it wrote.");
    println!("   Built from strip_prefix -- the search method from the page before.");

    println!("\n7. Editing in place, for real");
    let mut cfg = String::from("timeout = 30s");
    let at = cfg.find(char::is_numeric).unwrap();
    cfg.replace_range(at.., "60s");
    println!("   find(char::is_numeric) = {at}, then replace_range({at}.., \"60s\")");
    println!("   {cfg:?}");
    println!("   replace_range takes a BYTE RANGE, not a pattern -- which is exactly");
    println!("   the number find() hands you, and exactly the number that can panic.");

    let mut phone = String::from("+1 (555) 010-9999");
    phone.retain(|c| c.is_ascii_digit());
    println!("   retain(is_ascii_digit) {:?}   <- deletion in place, no new String", phone);

    println!("\n8. The panic replace_range keeps");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut animal = String::from("żółw");
    let boom = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        animal.replace_range(0..1, "z");
    }));
    std::panic::set_hook(hook);
    println!("   \"żółw\".replace_range(0..1, \"z\")  -> {}",
        if boom.is_ok() { "ok" } else { "panicked: 1 is inside 'ż'" });
    let mut animal = String::from("żółw");
    animal.replace_range(0..'ż'.len_utf8(), "z");
    println!("   0..'ż'.len_utf8() instead        -> {animal:?}");
    println!("   replace() never has this problem: it only ever cuts where a match");
    println!("   started, and a match starts on a character.");
}
