fn main() {
    println!("{:?}", "hello".split_at(2));
    println!("{:?}", "hello".split_at(0));
    println!("{:?}", "hello".split_at(5));

    // Byte offsets: 'é' occupies bytes 1 and 2, so only 1 and 3 are legal.
    let s = "héllo";
    for mid in 0..=s.len() {
        let ok = s.is_char_boundary(mid);
        println!("mid {mid} boundary={ok:<5} {:?}", if ok { Some(s.split_at(mid)) } else { None });
    }

    // The safe form, which reports the refusal instead of panicking.
    println!("{:?}", s.split_at_checked(2));
    println!("{:?}", s.split_at_checked(99));
}
