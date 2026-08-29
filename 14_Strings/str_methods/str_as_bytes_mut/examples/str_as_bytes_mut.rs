fn main() {
    let mut owned = String::from("hello world");

    // Sound: ASCII for ASCII, same byte count, still valid UTF-8.
    unsafe {
        let bytes = owned.as_mut_str().as_bytes_mut();
        bytes[0] = b'H';
        bytes[6] = b'W';
    }
    println!("{owned}");

    // Sound for the same reason, and safe to write.
    let mut safe = String::from("hello world");
    safe.as_mut_str().make_ascii_uppercase();
    println!("{safe}");

    // The boundary that makes the unsafe version unsafe.
    let mixed = "café";
    println!("{:?} is {} bytes for {} chars", mixed, mixed.len(), mixed.chars().count());
    println!("last char starts at byte {}", mixed.char_indices().last().unwrap().0);
}
