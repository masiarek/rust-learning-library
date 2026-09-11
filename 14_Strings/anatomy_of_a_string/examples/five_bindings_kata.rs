//! Kata solution: five bindings, two buffers — which names own bytes, which
//! point at bytes something else owns, and how big each handle is.
//!
//!   rustc --edition 2024 five_bindings_kata.rs -o /tmp/fbk && /tmp/fbk

use std::mem::size_of;

/// A handle's size in machine words. Words, not bytes, because this is the
/// number that stays the same on a 32-bit target: the bytes would halve.
fn words<T>(_: &T) -> usize {
    size_of::<T>() / size_of::<usize>()
}

fn main() {
    let a: &str = "hello";
    let b: String = String::from("hello");
    let c: &String = &b;
    let d: &str = b.as_str();
    let e: &&str = &a;

    println!("1. Five handles, measured in machine words");
    println!("   {:<5} {:<9} {:>5}   what the handle holds", "name", "type", "words");
    let rows: [(&str, &str, usize, &str); 5] = [
        ("a", "&str", words(&a), "the address of the bytes, and their length"),
        ("b", "String", words(&b), "address, length and capacity of its own buffer"),
        ("c", "&String", words(&c), "the address of b: a pointer to the pointer"),
        ("d", "&str", words(&d), "the address of b's bytes, and a length"),
        ("e", "&&str", words(&e), "the address of a"),
    ];
    for (name, ty, w, what) in rows {
        println!("   {name:<5} {ty:<9} {w:>5}   {what}");
    }
    assert_eq!([words(&a), words(&b), words(&c), words(&d), words(&e)], [2, 3, 1, 2, 1]);

    println!();
    println!("2. Two buffers behind five names");
    println!("   a and e reach the same bytes        {}", a.as_ptr() == e.as_ptr());
    println!("   b, c and d reach the same bytes     {}", b.as_ptr() == c.as_ptr() && c.as_ptr() == d.as_ptr());
    println!("   a and b hold equal text             {}", a == b);
    println!("   a and b hold it in the same bytes   {}", a.as_ptr() == b.as_ptr());
    println!("   b's buffer: capacity {} for its {} bytes", b.capacity(), b.len());

    println!();
    println!("3. Where each one lives");
    println!("   All five handles are locals, so all five are on the stack. Only two");
    println!("   copies of the bytes exist: \"hello\" in the program's read-only data,");
    println!("   which a and e reach, and the copy on the heap that b owns and that c");
    println!("   and d reach. Drop b, and any later use of c or d stops compiling;");
    println!("   a and e do not notice, because a literal lasts as long as the program.");
}
