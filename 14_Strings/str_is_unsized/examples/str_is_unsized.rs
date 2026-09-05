//! `str` is unsized: the size is a property of the value, not of the type,
//! so you never hold a `str` — only a pointer to one, and that pointer is
//! where the length lives.
//!
//! Run:  rustc --edition 2024 str_is_unsized.rs && ./str_is_unsized

use std::fmt::Display;

// One function, three unsized types. Without `?Sized` the implicit bound on
// `T` is `Sized`, and none of the three calls in section 4 would compile.
fn bytes_behind<T: ?Sized>(x: &T) -> usize {
    size_of_val(x)
}

// Unsizedness is contagious. A `str` is allowed as a struct's LAST field, and
// the struct is then unsized too — so `&Record` is a fat pointer, and there is
// no way to write a `Record { .. }` literal. Move `text` above `id` and even
// the declaration is E0277.
#[allow(dead_code)]
struct Record {
    id: u32,
    text: str,
}

fn main() {
    let word = size_of::<usize>();

    println!("1. The size belongs to the VALUE, not to the type");
    println!("   size_of_val(\"hello\")        = {}", size_of_val("hello"));
    println!("   size_of_val(\"hello, world\") = {}", size_of_val("hello, world"));
    println!("   both values have type `str`, and they are not the same size");
    println!("   size_of::<str>()            = does not compile (E0277)");

    println!();
    println!("2. So a `str` lives behind a pointer, measured in machine words");
    println!("   &str          {} words   pointer + length", size_of::<&str>() / word);
    println!("   &String       {} word    pointer (the String holds its own len)",
             size_of::<&String>() / word);
    println!("   String        {} words   pointer + length + capacity", size_of::<String>() / word);
    println!("   Box<str>      {} words   pointer + length, owned, no capacity",
             size_of::<Box<str>>() / word);

    println!();
    println!("3. The second word IS the length: a subslice shares the first one");
    let s = "hello, world";
    let head = &s[0..5];
    println!("   s.as_ptr() == head.as_ptr()  {}", s.as_ptr() == head.as_ptr());
    println!("   s.len() {}   head.len() {}   same bytes, different length word",
             s.len(), head.len());

    println!();
    println!("4. `?Sized` is what lets one function take all three");
    let nums: &[i32] = &[1, 2, 3];
    let shown: &dyn Display = &7i32;
    println!("   bytes_behind(\"hello\")         = {}", bytes_behind("hello"));
    println!("   bytes_behind(&[1, 2, 3])      = {}   (3 x i32)", bytes_behind(nums));
    println!("   bytes_behind(&7i32 as &dyn D) = {}    (size_of_val follows the vtable)",
             bytes_behind(shown));

    println!();
    println!("5. Every fat pointer is two words — but the second word differs");
    println!("   &str          {} words   the second is a LENGTH", size_of::<&str>() / word);
    println!("   &[i32]        {} words   the second is a LENGTH", size_of::<&[i32]>() / word);
    println!("   &dyn Display  {} words   the second is a VTABLE pointer",
             size_of::<&dyn Display>() / word);
    println!("   &i32          {} word    sized target, nothing to carry",
             size_of::<&i32>() / word);

    println!();
    println!("6. It is contagious: a struct ending in a `str` is unsized too");
    println!("   &Record       {} words   Record {{ id: u32, text: str }}",
             size_of::<&Record>() / word);
    println!("   you cannot write the literal, and you cannot put `text` first");
}
