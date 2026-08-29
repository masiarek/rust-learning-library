fn main() {
    println!("{:?}", "Hello World".to_lowercase());

    // The length can change.
    let dotted = "\u{0130}";                  // LATIN CAPITAL LETTER I WITH DOT ABOVE
    println!("{} bytes -> {} bytes", dotted.len(), dotted.to_lowercase().len());

    // Context-sensitive: one input character, two different outputs.
    println!("{:?}", "ΣΣ".to_lowercase());
    println!("{:?}", "ΟΔΟΣ".to_lowercase());

    // Not locale-aware: Turkish would want 'ı' here.
    println!("{:?}", "I".to_lowercase());

    // Allocates; the original is untouched.
    let original = "MiXeD";
    let lowered = original.to_lowercase();
    println!("{original:?} {lowered:?}");

    // Case-insensitive comparison, ASCII and otherwise.
    println!("{}", "HELLO".eq_ignore_ascii_case("hello"));
    println!("{}", "ΑΒΓ".to_lowercase() == "αβγ".to_lowercase());
}
