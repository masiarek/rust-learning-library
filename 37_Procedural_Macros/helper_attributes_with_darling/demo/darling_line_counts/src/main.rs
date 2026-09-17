//! Compares this lesson's crates with helper_attributes_by_hand's, so the
//! numbers on both pages are a program's output rather than a hand count.
//!
//! `include_str!` reads each file while compiling, and Cargo rebuilds this
//! program whenever one of them changes.

/// A source file of the by-hand derive, in the neighbouring lesson's demo.
macro_rules! by_hand {
    ($file:literal) => {
        include_str!(concat!(
            "../../../../helper_attributes_by_hand/demo/by_hand_derive/src/",
            $file
        ))
    };
}

/// A source file of this lesson's derive.
macro_rules! darling {
    ($file:literal) => {
        include_str!(concat!("../../darling_derive/src/", $file))
    };
}

const MACRO_FILES: [(&str, &str, &str); 4] = [
    ("attrs.rs", "by hand", by_hand!("attrs.rs")),
    ("attrs.rs", "darling", darling!("attrs.rs")),
    ("lib.rs", "by hand", by_hand!("lib.rs")),
    ("lib.rs", "darling", darling!("lib.rs")),
];

fn main() {
    println!(
        "{:<9} {:<8} {:>5} {:>5}",
        "file", "version", "lines", "code"
    );
    for (file, version, text) in MACRO_FILES {
        let lines = text.lines().count();
        // Code: not blank and not a comment. Doc comments count as comments.
        let code = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .count();
        println!("{file:<9} {version:<8} {lines:>5} {code:>5}");
    }

    // The programs that use the derive are copies; check that they still are.
    let app = include_str!("../../../../helper_attributes_by_hand/demo/by_hand_app/src/main.rs")
        == include_str!("../../darling_app/src/main.rs");
    let mistakes =
        include_str!("../../../../helper_attributes_by_hand/demo/by_hand_mistakes/src/main.rs")
            == include_str!("../../darling_mistakes/src/main.rs");
    println!();
    println!("darling_app is by_hand_app, unchanged: {app}");
    println!("darling_mistakes is by_hand_mistakes, unchanged: {mistakes}");
}
