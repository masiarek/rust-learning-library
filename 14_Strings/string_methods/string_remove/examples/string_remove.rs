fn main() {
    let mut s = String::from("héllo");
    println!("removed {:?}, left {:?}", s.remove(1), s);

    // The offset must start a character.
    let mut t = String::from("héllo");
    println!("boundary at 2? {}", t.is_char_boundary(2));
    println!("removed {:?}", t.remove(0));

    // Removing by rule: retain is linear and does not need offsets.
    let mut vowels = String::from("programming");
    vowels.retain(|c| !"aeiou".contains(c));
    println!("{vowels:?}");

    // The same by hand is quadratic AND needs care with shifting offsets.
    let mut manual = String::from("programming");
    let mut i = 0;
    while i < manual.len() {
        if "aeiou".contains(manual[i..].chars().next().unwrap()) {
            manual.remove(i);
        } else {
            i += manual[i..].chars().next().unwrap().len_utf8();
        }
    }
    println!("{manual:?}");
}
