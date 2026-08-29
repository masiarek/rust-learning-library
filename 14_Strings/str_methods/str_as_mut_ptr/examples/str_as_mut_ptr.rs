fn main() {
    let mut owned = String::from("hello");

    // Safe, because ASCII -> ASCII is byte-for-byte and cannot break UTF-8.
    let s: &mut str = owned.as_mut_str();
    let p = s.as_mut_ptr();
    unsafe { *p = b'H'; }
    println!("{owned}");

    // The safe spelling of the same edit.
    let mut safe = String::from("hello");
    safe.as_mut_str().make_ascii_uppercase();
    println!("{safe}");

    // Why it is unsafe: 'é' is two bytes, so overwriting one of them alone
    // would leave the string invalid. Shown as a comment, not run.
    let two_byte = String::from("é");
    println!("'é' occupies {} bytes", two_byte.len());
}
