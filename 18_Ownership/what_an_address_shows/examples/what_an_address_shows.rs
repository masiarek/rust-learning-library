//! What `&x` prints — and what actually moved when the number changes.
//!
//! Note what this program does NOT do: print an address. Every line below is a
//! comparison or a distance, because a raw address differs on every run and
//! could never be an answer key. That restraint IS the lesson.
//!
//!   rustc --edition 2024 what_an_address_shows.rs -o /tmp/was && /tmp/was

fn main() {
    let x = String::from("hello world");
    let header_x = &x as *const String; // the three words, on the stack
    let bytes_x = x.as_ptr(); //            the text, on the heap

    println!("1. A String is in two places at once");
    println!("   size_of::<String>() = {}   ptr + len + capacity, on the stack", std::mem::size_of::<String>());
    println!("   x.len()             = {}   bytes of text, on the heap", x.len());
    println!("   `&x` is the address of the first one. Never the second.");

    let y = x; // <- the move

    println!();
    println!("2. What `let y = x;` did");
    println!("   header at a new address? {}", header_x as usize != &y as *const String as usize);
    println!("   heap bytes relocated?    {}", bytes_x != y.as_ptr());
    println!("   same allocation?         {}", std::ptr::eq(bytes_x, y.as_ptr()));
    println!("   {} bytes were copied between stack slots; {} bytes of text stayed put.",
        std::mem::size_of::<String>(), y.len());

    println!();
    println!("3. A changed address proves nothing about moving — Copy does it too");
    let a: i32 = 5;
    let b = a; // a copy, not a move: `a` is still usable below
    println!("   a and b at different addresses? {}", (&a as *const i32) != (&b as *const i32));
    println!("   and yet a is still alive: a = {}, b = {}", a, b);

    println!();
    println!("4. What you can safely print: distances, not addresses");
    let cells: [u8; 4] = [10, 20, 30, 40];
    let step = &cells[1] as *const u8 as usize - &cells[0] as *const u8 as usize;
    println!("   &cells[1] - &cells[0] = {} byte   — the same on every run", step);
    println!("   the addresses themselves are different on every run.");
}
