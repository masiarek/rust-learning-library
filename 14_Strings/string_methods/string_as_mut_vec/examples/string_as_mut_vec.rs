fn main() {
    let mut s = String::from("hello");

    // Sound: ASCII appended to ASCII is still valid UTF-8.
    unsafe { s.as_mut_vec().extend_from_slice(b" world") }
    println!("{s:?}");

    // Sound: truncating at a boundary we checked.
    let mut wide = String::from("héllo");
    let cut = wide.floor_char_boundary(3);
    unsafe { wide.as_mut_vec().truncate(cut) }
    println!("{wide:?}");

    // The safe equivalents of both.
    let mut safe = String::from("hello");
    safe.push_str(" world");
    safe.truncate(5);
    println!("{safe:?}");

    // Why the truncate above needed a boundary check.
    let check = "héllo";
    println!("boundary at 2? {}  at 3? {}",
             check.is_char_boundary(2), check.is_char_boundary(3));
}
