fn main() {
    let units: Vec<u16> = "hi 👋".encode_utf16().collect();
    println!("{units:?}");
    println!("{:?}", String::from_utf16(&units));

    // A surrogate pair is two units for one char.
    println!("{} units, {} chars", units.len(), "hi 👋".chars().count());

    // An unpaired surrogate is an error, not a character.
    let lone = [0xD83D];
    println!("{:?}", String::from_utf16(&lone).is_err());
    println!("{:?}", String::from_utf16_lossy(&lone));

    // Round trip.
    let text = "héllo 👋";
    let wide: Vec<u16> = text.encode_utf16().collect();
    println!("{}", String::from_utf16(&wide).unwrap() == text);
}
