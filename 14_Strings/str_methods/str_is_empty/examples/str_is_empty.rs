fn main() {
    let cases = ["", " ", "   ", "\n", "a"];
    println!("{:<6} {:<8} {}", "input", "empty?", "trimmed empty?");
    for s in cases {
        // Debug ignores a width, so pad the formatted string, not the value.
        println!("{:<6} {:<8} {}", format!("{s:?}"), s.is_empty(), s.trim().is_empty());
    }

    // The whole implementation.
    let s = "";
    println!("{}", s.is_empty() == (s.len() == 0));

    // Empty is not missing: Option carries the difference is_empty cannot.
    let answers: [Option<&str>; 3] = [Some("yes"), Some(""), None];
    for a in answers {
        let verdict = match a {
            None => "never asked",
            Some(v) if v.trim().is_empty() => "asked, left blank",
            Some(_) => "answered",
        };
        println!("{a:?} -> {verdict}");
    }
}
