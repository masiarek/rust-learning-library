fn main() {
    let boxed: Box<str> = String::from("héllo").into_boxed_str();
    println!("{} bytes as str", boxed.len());

    let bytes: Box<[u8]> = boxed.into_boxed_bytes();
    println!("{bytes:?}");
    println!("{} bytes as [u8]", bytes.len());

    // Onward to Vec<u8>, still without copying the contents.
    let v: Vec<u8> = bytes.into_vec();
    println!("{v:?}");

    // The checked trip back.
    println!("{:?}", String::from_utf8(v));

    // The UTF-8 promise really is gone: arbitrary bytes are now representable.
    let invalid: Box<[u8]> = vec![0xff, 0xfe].into_boxed_slice();
    println!("{:?}", std::str::from_utf8(&invalid).is_err());
}
