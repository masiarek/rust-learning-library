// The invalid byte arrays below are the point of the example, so the
// lint that spots them is turned off rather than worked around.
#![allow(invalid_from_utf8)]

fn main() {
    let s = "héllo";

    // Sound: these bytes came from a &str a moment ago.
    let bytes = s.as_bytes();
    let back = unsafe { str::from_utf8_unchecked(bytes) };
    println!("{back:?} identical={}", back == s);

    // The check being skipped, for comparison.
    println!("{:?}", str::from_utf8(bytes));

    // What the checked version refuses -- and what the unchecked one would
    // have made undefined behaviour instead of an error.
    let invalid = [0xff, 0xfe];
    println!("{:?}", str::from_utf8(&invalid).is_err());

    // A round trip through a Vec, still sound.
    let owned: Vec<u8> = s.as_bytes().to_vec();
    println!("{:?}", unsafe { str::from_utf8_unchecked(&owned) });
}
