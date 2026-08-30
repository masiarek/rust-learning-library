// Absence is a type, not a value a pointer can quietly hold.

/// The ballot a voter cast, or nothing when they did not.
fn ballot_for(voter: u32) -> Option<&'static str> {
    match voter {
        1 => Some("Ada"),
        _ => None,
    }
}

fn main() {
    // An Option is not a &str, so the empty case has to be written down.
    let described = match ballot_for(2) {
        Some(choice) => format!("{} characters", choice.len()),
        None => "no ballot".to_string(),
    };
    println!("match on ballot_for(2)       -> {described}");

    println!("ballot_for(1).unwrap_or(\"\")  -> {}", ballot_for(1).unwrap_or("").len());
    println!("ballot_for(2).map_or(0, len) -> {}", ballot_for(2).map_or(0, str::len));

    // Handing `ballot_for(2)` to a function that wants `&str` is E0308, and
    // the compiler's own suggestion is `.expect("REASON")`.
}
