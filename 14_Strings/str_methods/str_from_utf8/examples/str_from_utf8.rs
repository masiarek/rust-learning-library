// The invalid byte arrays below are the point of the example, so the
// lint that spots them is turned off rather than worked around.
#![allow(invalid_from_utf8)]

fn main() {
    let good = [104, 105];
    println!("{:?}", str::from_utf8(&good));

    // Invalid sequence: where it failed, and how long the bad run is.
    let bad = [104, 0xff, 105];
    match str::from_utf8(&bad) {
        Ok(s) => println!("{s:?}"),
        Err(e) => println!("valid_up_to {} error_len {:?}", e.valid_up_to(), e.error_len()),
    }

    // Truncated input: error_len is None, meaning "incomplete", not "invalid".
    let cut = "é".as_bytes();
    println!("{:?}", str::from_utf8(&cut[..1]).unwrap_err().error_len());

    // Borrowed, not copied: the same bytes, reinterpreted.
    let owned = vec![104, 105];
    let view = str::from_utf8(&owned).unwrap();
    println!("{view:?} from a {}-byte buffer", owned.len());

    // const validation.
    const BYTES: &[u8] = b"compiled";
    const TEXT: &str = match str::from_utf8(BYTES) { Ok(s) => s, Err(_) => "" };
    println!("{TEXT:?}");
}
