fn main() {
    // "hi" big-endian: 0x00 0x68, 0x00 0x69
    let be = [0x00, 0x68, 0x00, 0x69];
    println!("{:?}", String::from_utf16be(&be));

    // The wrong endianness succeeds, with different characters -- no error.
    println!("{:?}", String::from_utf16le(&be));

    // Which is why the byte order must come from the format, not a guess.
    let text = "hi";
    let le: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let be2: Vec<u8> = text.encode_utf16().flat_map(u16::to_be_bytes).collect();
    println!("{le:?} vs {be2:?}");
    println!("{:?} {:?}", String::from_utf16le(&le), String::from_utf16be(&be2));

    // An odd byte count is refused.
    println!("{:?}", String::from_utf16be(&[0x00, 0x68, 0x00]).is_err());
}
