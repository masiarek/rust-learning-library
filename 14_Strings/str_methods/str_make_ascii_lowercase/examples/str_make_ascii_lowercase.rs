fn main() {
    let mut s = String::from("HELLO Wörld");
    s.make_ascii_lowercase();
    println!("{s:?}");                        // ö is untouched

    // In place: no allocation, and the original binding is changed.
    let mut header = String::from("Content-Type");
    header.make_ascii_lowercase();
    println!("{header:?}");

    // It returns (), so it does not chain.
    let mut t = String::from("ABC");
    let unit = t.make_ascii_lowercase();
    println!("{t:?} and the call returned {unit:?}");

    // A Unicode in-place edit has to go through String, because it can grow.
    let mut u = String::from("İSTANBUL");
    println!("{} bytes", u.len());
    u = u.to_lowercase();
    println!("{u:?}, {} bytes", u.len());

    // Only part of a string, via a mutable slice.
    let mut part = String::from("ABCDEF");
    part.as_mut_str()[..3].make_ascii_lowercase();
    println!("{part:?}");
}
