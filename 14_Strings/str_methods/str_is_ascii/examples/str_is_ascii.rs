fn main() {
    for s in ["hello", "héllo", "", "123!@#"] {
        println!("{s:<8?} ascii={}", s.is_ascii());
    }

    // The guard that picks the right family.
    for s in ["Content-Type", "Größe"] {
        let lowered = if s.is_ascii() { s.to_ascii_lowercase() } else { s.to_lowercase() };
        println!("{s:<14?} -> {lowered:?}");
    }

    // For ASCII, the two rulers agree and every offset is a boundary.
    let a = "hello";
    println!("{} {} {}", a.len(), a.chars().count(),
             (0..=a.len()).all(|i| a.is_char_boundary(i)));

    // const.
    const TOKEN: &str = "GET";
    const OK: bool = TOKEN.is_ascii();
    println!("{OK}");
}
