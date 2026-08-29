fn main() {
    let s = String::with_capacity(32);
    println!("len {} capacity {}", s.len(), s.capacity());

    // No reallocation while it fits.
    let mut planned = String::with_capacity(32);
    for word in ["alpha ", "beta ", "gamma"] {
        planned.push_str(word);
        println!("len {:>2} capacity {:>2}", planned.len(), planned.capacity());
    }

    // Growing from nothing walks a doubling sequence instead.
    let mut grown = String::new();
    for word in ["alpha ", "beta ", "gamma"] {
        grown.push_str(word);
        println!("len {:>2} capacity {:>2}  (from new)", grown.len(), grown.capacity());
    }
    println!("equal contents: {}", planned == grown);

    // Bytes, not characters.
    let wide = "é".repeat(4);
    println!("4 chars need {} bytes", wide.len());

    // Sizing from the input is the honest estimate.
    let input = "the quick brown fox";
    let mut upper = String::with_capacity(input.len());
    upper.push_str(&input.to_ascii_uppercase());
    println!("{} / {}", upper.len(), upper.capacity());
}
