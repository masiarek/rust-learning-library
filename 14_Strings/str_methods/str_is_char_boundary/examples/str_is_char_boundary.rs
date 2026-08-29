fn main() {
    let s = "héllo";      // h=0, é=1..3, l=3, l=4, o=5, len=6

    for i in 0..=s.len() + 1 {
        println!("{i} -> {}", s.is_char_boundary(i));
    }

    // The predicate behind the panic.
    for i in [1, 2, 3] {
        let ok = s.is_char_boundary(i);
        println!("&s[..{i}] {}", if ok { format!("= {:?}", &s[..i]) } else { "would panic".into() });
    }

    // Testing vs repairing.
    let bad = 2;
    println!("test:   {}", s.is_char_boundary(bad));
    println!("repair: {} down, {} up", s.floor_char_boundary(bad), s.ceil_char_boundary(bad));

    // ASCII: every offset is a boundary.
    let ascii = "hello";
    println!("{}", (0..=ascii.len()).all(|i| ascii.is_char_boundary(i)));
}
