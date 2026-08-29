fn main() {
    let s = "héllo";

    println!("{:?}", s.bytes().collect::<Vec<u8>>());
    println!("bytes={} chars={}", s.bytes().count(), s.chars().count());

    // ExactSizeIterator: the length is known without walking.
    println!("len hint = {}", s.bytes().len());

    // DoubleEndedIterator: same bytes, from the back.
    println!("{:?}", s.bytes().rev().take(3).collect::<Vec<u8>>());

    // A checksum — a job where bytes really are the unit.
    let sum: u32 = "hello".bytes().map(u32::from).sum();
    println!("sum = {sum}");

    // ASCII classification without decoding.
    let digits = "a1b2c3".bytes().filter(u8::is_ascii_digit).count();
    println!("{digits} ascii digits");
}
