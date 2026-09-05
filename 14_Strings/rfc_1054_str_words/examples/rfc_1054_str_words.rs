fn parts(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn main() {
    let messy = "  the quick\tbrown\nfox  ";

    println!("WHAT SHIPPED IS THE RFC'S FORMULA, VERBATIM");
    let rfc = messy.split(char::is_whitespace).filter(|s| !s.is_empty());
    println!("  split_whitespace()            {:?}", parts(messy));
    println!("  split(is_whitespace).filter() {:?}", rfc.clone().collect::<Vec<&str>>());
    println!("  identical: {}", messy.split_whitespace().eq(rfc));

    println!();
    println!("WHY IT IS A METHOD AND NOT A PATTERN (lilyball's objection)");
    println!("  \" a b \".split(is_whitespace)  {:?}", " a b ".split(char::is_whitespace).collect::<Vec<&str>>());
    println!("  \" a b \".split_whitespace()    {:?}", parts(" a b "));
    println!("  a Pattern cannot drop the leading and trailing empties;");
    println!("  the .filter() is what the method exists to carry.");

    println!();
    println!("THE ANSWER EVERYONE REACHES FOR FIRST");
    println!("  split(' ')  {:?}", messy.split(' ').collect::<Vec<&str>>());
    println!("  it neither collapses runs nor knows a tab is whitespace.");

    println!();
    println!("THE ARGUMENT THE RFC WON: \"A WORD\" HAS NO PORTABLE DEFINITION");
    let ja = "私は学生です";
    println!("  {ja}  (\"I am a student\") has no spaces in it at all,");
    println!("  so split_whitespace finds {} piece for its {} chars,",
             ja.split_whitespace().count(), ja.chars().count());
    println!("  where a Japanese reader finds several words.");
    println!("  punctuation rides along too: {:?}", parts("don't stop, e.g. now"));
    println!("  UAX #29 would answer all of this. std declined to ship an answer.");

    println!();
    println!("...AND \"WHITESPACE\" TURNS OUT TO BE AMBIGUOUS TOO");
    println!("  {:<16}{:<15}{:<18}{:<16}{}", "char", "is_whitespace", "split_whitespace", "split_ascii_ws", "lines");
    for (name, c) in [
        ("SPACE  U+0020", ' '),      ("TAB    U+0009", '\t'),
        ("VT     U+000B", '\u{0B}'), ("FF     U+000C", '\u{0C}'),
        ("NBSP   U+00A0", '\u{A0}'), ("NEL    U+0085", '\u{85}'),
        ("OGHAM  U+1680", '\u{1680}'), ("LS     U+2028", '\u{2028}'),
        ("PS     U+2029", '\u{2029}'), ("ZWSP   U+200B", '\u{200B}'),
        ("FS     U+001C", '\u{1C}'),
    ] {
        let s = format!("a{c}b");
        println!(
            "  {:<16}{:<15}{:<18}{:<16}{}",
            name,
            c.is_whitespace(),
            s.split_whitespace().count(),
            s.split_ascii_whitespace().count(),
            s.lines().count(),
        );
    }
    println!("  VT is ASCII and IS whitespace, yet split_ascii_whitespace skips it,");
    println!("  because is_ascii_whitespace is the WhatWG set, minus U+000B.");
    println!("  ZWSP is called a space and is not one (White_Space=no).");
    println!("  FS is not whitespace here; Python's split() DOES split on it.");
    println!("  LS and PS split words here but are NOT line endings -- see RFC 1212.");
}
