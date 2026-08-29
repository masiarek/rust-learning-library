use std::borrow::Cow;

fn main() {
    println!("{:?}", String::from_utf8_lossy(&[104, 105]));
    println!("{:?}", String::from_utf8_lossy(&[104, 0xff, 105]));

    // Borrowed when clean, Owned only when something was replaced.
    for bytes in [&[104u8, 105][..], &[104, 0xff, 105][..]] {
        match String::from_utf8_lossy(bytes) {
            Cow::Borrowed(s) => println!("borrowed {s:?} -- input was valid"),
            Cow::Owned(s) => println!("owned    {s:?} -- something was replaced"),
        }
    }

    // Lossy: the bytes do not survive a round trip.
    let original = vec![104, 0xff, 105];
    let decoded = String::from_utf8_lossy(&original);
    println!("{:?} -> {:?}", original, decoded.as_bytes());

    // One replacement can stand for SEVERAL bad bytes: 0xE2 0x82 is the
    // truncated start of a 3-byte sequence, so it is one bad run, not two.
    for bytes in [&[0xffu8, 0xfe, 0xfd][..], &[0xE2, 0x82][..]] {
        let out = String::from_utf8_lossy(bytes);
        println!("{} bad bytes -> {} replacement char(s)  {:?}",
                 bytes.len(),
                 out.chars().filter(|c| *c == '\u{FFFD}').count(),
                 out);
    }
}
