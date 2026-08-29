fn main() {
    let mut owned = String::from("hello world");

    // Sound: 0..5 is in range and both ends are boundaries.
    unsafe { owned.as_mut_str().get_unchecked_mut(0..5) }.make_ascii_uppercase();
    println!("{owned:?}");

    // The safe spelling of the same edit.
    let mut safe = String::from("hello world");
    if let Some(part) = safe.get_mut(0..5) {
        part.make_ascii_uppercase();
    }
    println!("{safe:?}");

    // Boundaries first, then the edit — the offsets justify the unsafe block.
    let mut accented = String::from("héllo");
    let cut = accented.char_indices().nth(2).unwrap().0;
    println!("cut at byte {cut}, boundary={}", accented.is_char_boundary(cut));
    unsafe { accented.as_mut_str().get_unchecked_mut(cut..) }.make_ascii_uppercase();
    println!("{accented:?}");
}
