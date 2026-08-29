fn main() {
    let s = String::from("héllo");
    println!("{s:?} len {} capacity {}", s.len(), s.capacity());

    // The round trip: exactly the three values that came out.
    let (ptr, len, cap) = s.into_raw_parts();
    println!("carried across as {len} and {cap}");
    let back = unsafe { String::from_raw_parts(ptr, len, cap) };
    println!("{back:?}");

    // A Vec<u8> allocated by Rust works too, if the bytes are valid UTF-8.
    let mut v = Vec::from("hi".as_bytes());
    v.reserve_exact(6);
    let (p, l, c) = (v.as_mut_ptr(), v.len(), v.capacity());
    std::mem::forget(v);
    let rebuilt = unsafe { String::from_raw_parts(p, l, c) };
    println!("{rebuilt:?} capacity {}", rebuilt.capacity());
}
