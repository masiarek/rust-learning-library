fn main() {
    let mut buf = *b"hello";

    // Sound: ASCII going in, and an ASCII-preserving edit.
    let s = unsafe { str::from_utf8_unchecked_mut(&mut buf) };
    s.make_ascii_uppercase();
    println!("{:?}", str::from_utf8(&buf).unwrap());

    // The checked version, which costs one scan and removes the obligation.
    let mut other = *b"hello";
    str::from_utf8_mut(&mut other).unwrap().make_ascii_uppercase();
    println!("{:?}", str::from_utf8(&other).unwrap());

    // Multi-byte text: still sound, because the edit leaves non-ASCII alone.
    let mut wide = *"héllo".as_bytes().first_chunk::<6>().unwrap();
    let w = unsafe { str::from_utf8_unchecked_mut(&mut wide) };
    w.make_ascii_uppercase();
    println!("{:?}", str::from_utf8(&wide).unwrap());
}
