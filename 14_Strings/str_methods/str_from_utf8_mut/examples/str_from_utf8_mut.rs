fn main() {
    let mut buf = *b"hello world";

    let text = str::from_utf8_mut(&mut buf).unwrap();
    text.make_ascii_uppercase();
    println!("{:?}", str::from_utf8(&buf).unwrap());

    // Editing one field of a decoded frame, in place.
    let mut frame = *b"name=alice";
    {
        let s = str::from_utf8_mut(&mut frame).unwrap();
        if let Some(i) = s.find('=') {
            s[i + 1..].make_ascii_uppercase();
        }
    }
    println!("{:?}", str::from_utf8(&frame).unwrap());

    // Invalid input is refused, with the same error detail.
    let mut bad = [104, 0xff];
    println!("{:?}", str::from_utf8_mut(&mut bad).unwrap_err().valid_up_to());
}
