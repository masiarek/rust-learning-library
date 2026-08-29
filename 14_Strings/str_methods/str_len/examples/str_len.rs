fn main() {
    let ascii = "hello";
    let accented = "héllo";
    let emoji = "hi 👋";

    for s in [ascii, accented, emoji] {
        println!("{:<8} len={} chars={}", s, s.len(), s.chars().count());
    }

    // len() is the length of the *view*, not of whatever owns the bytes.
    let owner = String::from("hello world");
    let window = &owner[0..5];
    println!("owner {} / window {}", owner.len(), window.len());

    // It is a const fn, so a literal's length is known at compile time.
    const GREETING: &str = "hello";
    const N: usize = GREETING.len();
    println!("const len = {N}");
}
