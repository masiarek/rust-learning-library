fn main() {
    // "hi" little-endian: 0x68 0x00, 0x69 0x00
    let le = [0x68, 0x00, 0x69, 0x00];
    println!("{:?}", String::from_utf16le(&le));

    // The same bytes read big-endian are different characters entirely.
    println!("{:?}", String::from_utf16be(&le));

    // An odd byte count cannot be decoded.
    println!("{:?}", String::from_utf16le(&[0x68, 0x00, 0x69]).is_err());

    // A BOM is decoded, not consumed.
    let with_bom = [0xFF, 0xFE, 0x68, 0x00];
    let decoded = String::from_utf16le(&with_bom).unwrap();
    println!("{:?} first char {:?}", decoded, decoded.chars().next());
    println!("stripped: {:?}", decoded.strip_prefix('\u{FEFF}'));

    // Round trip through bytes.
    let text = "héllo";
    let bytes: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    println!("{}", String::from_utf16le(&bytes).unwrap() == text);
}
