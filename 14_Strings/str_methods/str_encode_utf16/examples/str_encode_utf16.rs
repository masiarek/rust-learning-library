fn main() {
    let s = "hi";
    println!("{:?}", s.encode_utf16().collect::<Vec<u16>>());

    // Outside the BMP: one char, two code units.
    let emoji = "👋";
    let units: Vec<u16> = emoji.encode_utf16().collect();
    println!("{units:?}");
    println!("{} chars, {} utf16 units, {} utf8 bytes",
             emoji.chars().count(), units.len(), emoji.len());

    // Three different "lengths" for the same text.
    let mixed = "a👋b";
    println!("utf8 {} / chars {} / utf16 {}",
             mixed.len(), mixed.chars().count(), mixed.encode_utf16().count());

    // The round trip.
    let wide: Vec<u16> = mixed.encode_utf16().collect();
    println!("{:?}", String::from_utf16(&wide));

    // A NUL-terminated buffer, as a Windows API wants.
    let wide_z: Vec<u16> = "ok".encode_utf16().chain(std::iter::once(0)).collect();
    println!("{wide_z:?}");
}
