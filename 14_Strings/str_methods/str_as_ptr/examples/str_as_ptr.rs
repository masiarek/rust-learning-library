fn main() {
    let s = "hello";

    // Addresses differ every run, so prove the layout with *derived* arithmetic.
    let start = s.as_ptr() as usize;
    let third = s[2..].as_ptr() as usize;
    println!("a slice at byte 2 starts {} bytes along", third - start);

    // The pointer plus the length is the whole of a &str.
    let rebuilt = unsafe { std::slice::from_raw_parts(s.as_ptr(), s.len()) };
    println!("{:?}", std::str::from_utf8(rebuilt));

    // No terminator: the length is carried beside the pointer, not in the
    // bytes, which is why a C API needs CString rather than this pointer.
    println!("len is carried separately: {}", s.len());

    // Growing past the capacity gets a new buffer, so any pointer taken
    // before it is stale. Capacity is the deterministic signal; whether the
    // address itself changes is up to the allocator.
    let mut owned = String::with_capacity(4);
    owned.push_str("abcd");
    let before = owned.capacity();
    owned.push_str("efgh");
    println!("capacity {} -> {}", before, owned.capacity());
}
