fn main() {
    let units: Vec<u16> = "hi 👋".encode_utf16().collect();
    println!("{:?}", String::from_utf16_lossy(&units));

    // Unpaired surrogates are replaced rather than rejected.
    let broken = [0x0068, 0xD83D, 0x0069];
    println!("{:?}", String::from_utf16(&broken).is_err());
    println!("{:?}", String::from_utf16_lossy(&broken));

    // Always a String, never a Cow -- re-encoding is unavoidable.
    let clean: String = String::from_utf16_lossy(&units);
    println!("{} bytes of UTF-8 from {} UTF-16 units", clean.len(), units.len());
}
