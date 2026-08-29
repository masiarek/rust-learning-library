fn main() {
    let s = "hé!";
    println!("{:?}", s.as_bytes());          // [104, 195, 169, 33]
    println!("{} bytes, {} chars", s.as_bytes().len(), s.chars().count());

    // A byte-literal comparison, which is what as_bytes is really for.
    println!("{}", "GET /".as_bytes().starts_with(b"GET"));

    // An ASCII-only scan: legal because every byte here is one character,
    // so a byte index and a character index are the same number.
    let code = "A7";
    let bytes = code.as_bytes();
    println!("letter {:?}, digit {}", bytes[0] as char, bytes[1] - b'0');

    // The trap: byte 1 of "hé!" is half of 'é'.
    println!("byte 1 = {}, char 1 = {:?}", s.as_bytes()[1], s.chars().nth(1).unwrap());
}
