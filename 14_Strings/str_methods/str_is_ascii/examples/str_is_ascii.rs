fn main() {
    for s in ["hello", "héllo", "", "123!@#"] {
        println!("{:<8} ascii={}", format!("{s:?}"), s.is_ascii());
    }

    // The guard that picks the right family.
    for s in ["Content-Type", "Größe"] {
        let lowered = if s.is_ascii() { s.to_ascii_lowercase() } else { s.to_lowercase() };
        println!("{:<14} -> {lowered:?}", format!("{s:?}"));
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
