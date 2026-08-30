fn main() {
    // The fill is '.' so that padding, where it happens, is visible.

    // The bug: a width on {:?} over text is accepted and then ignored.
    println!("[{:.<8?}]  {{:.<8?}} on a &str", "ab");
    // The fix: build the Debug string first, then pad that with Display.
    println!("[{:.<8}]  {{:.<8}} on format!(\"{{:?}}\")", format!("{:?}", "ab"));

    // Width reaches every impl the same way. Only some of them apply it.
    println!("\nsame spec, eight different Debug impls:");
    for (label, shown) in [
        ("i32", format!("[{:.<8?}]", 42)),
        ("f64", format!("[{:.<8?}]", 1.5)),
        ("bool", format!("[{:.<8?}]", true)),
        ("&str", format!("[{:.<8?}]", "ab")),
        ("char", format!("[{:.<8?}]", 'a')),
        ("String", format!("[{:.<8?}]", String::from("ab"))),
        ("Option<u8>", format!("[{:.<8?}]", Some(1u8))),
        ("Vec<u8>", format!("[{:.<8?}]", vec![1u8, 2])),
    ] {
        println!("  {label:<12} {shown}");
    }

    // A container hands its elements the SAME Formatter, so the width lands
    // inside the brackets. Vec<&str> looks untouched only because &str ignores it.
    println!("\ncontainers forward the spec to their elements:");
    println!("  Vec<u8>   [{:.<6?}]", vec![1u8, 2]);
    println!("  Vec<&str> [{:.<6?}]", vec!["a", "b"]);

    // Precision is the same story: it truncates through Display, not Debug.
    println!("\nprecision, on the same eight-character string:");
    println!("  {{:.3}}  [{:.3}]", "abcdefgh");
    println!("  {{:.3?}} [{:.3?}]", "abcdefgh");
}
