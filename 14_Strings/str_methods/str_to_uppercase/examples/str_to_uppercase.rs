fn main() {
    println!("{:?}", "Hello".to_uppercase());

    // One character becomes two.
    let sharp_s = "ß";
    println!("{:?} -> {:?}", sharp_s, sharp_s.to_uppercase());
    println!("{} char -> {} chars", sharp_s.chars().count(), sharp_s.to_uppercase().chars().count());

    // So it does not round-trip.
    println!("{:?}", "ß".to_uppercase().to_lowercase());
    println!("round trips: {}", "ß".to_uppercase().to_lowercase() == "ß");

    // Ligatures expand too.
    println!("{:?}", "\u{FB01}".to_uppercase());   // LATIN SMALL LIGATURE FI

    // A whole word. Here the BYTE count happens to match -- 'ß' is two bytes
    // and "SS" is two bytes -- while the CHARACTER count still grows.
    let word = "straße";
    let upper = word.to_uppercase();
    println!("{word:?} {} bytes / {} chars", word.len(), word.chars().count());
    println!("{upper:?} {} bytes / {} chars", upper.len(), upper.chars().count());
}
