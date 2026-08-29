fn main() {
    let be = [0x00, 0x68, 0x00, 0x69];
    println!("{:?}", String::from_utf16be_lossy(&be));

    // Truncation is replaced rather than refused.
    println!("{:?}", String::from_utf16be_lossy(&[0x00, 0x68, 0x00]));

    // The wrong endianness is silent -- no error, no replacement char.
    let wrong = String::from_utf16le_lossy(&be);
    println!("{:?} contains no replacement char: {}",
             wrong, !wrong.contains('\u{FFFD}'));
}
