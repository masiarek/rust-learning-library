fn main() {
    let s = String::from("héllo");

    // Sound: these bytes came out of a String a moment ago.
    let bytes = s.clone().into_bytes();
    let back = unsafe { String::from_utf8_unchecked(bytes) };
    println!("{back:?} identical={}", back == s);

    // The scan being skipped.
    let again = "héllo".as_bytes().to_vec();
    println!("{:?}", String::from_utf8(again));

    // What the checked version refuses.
    println!("{:?}", String::from_utf8(vec![0xff, 0xfe]).is_err());

    // Neither version copies: from_utf8 costs a scan, not an allocation.
    let big = "x".repeat(1000).into_bytes();
    let n = big.len();
    let text = String::from_utf8(big).unwrap();
    println!("{n} bytes in, {} bytes out", text.len());
}
