//! A slice is a view: a pointer and a length into text someone else owns.
//!
//!   rustc --edition 2024 string_slices.rs -o /tmp/ss && /tmp/ss

use std::panic;

/// Returns where the first word ends. A number, tied to nothing.
fn first_word_index(s: &str) -> usize {
    match s.find(' ') {
        Some(i) => i,
        None => s.len(),
    }
}

/// Returns the first word itself. A view, tied to `s` by the borrow checker.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    println!("1. The bug a slice removes");
    let mut stale = String::from("hello world");
    let end = first_word_index(&stale);
    println!("   first_word_index(&s) = {end}       <- a bare usize");
    stale.clear();
    println!("   s.clear()   ->   s = {:?}, {} bytes", stale, stale.len());
    println!("   `end` is still {end}, indexing text that is gone. Nothing warned.");
    println!("   first_word() cannot reach here: holding its &str freezes `s` (E0502).");

    println!("\n2. What a slice is made of");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("   s      String   {:?}   len {}   capacity {}", s, s.len(), s.capacity());
    println!("   hello  &str     {:?}         len {}", hello, hello.len());
    println!("   world  &str     {:?}         len {}   <- points at byte 6", world, world.len());
    println!("   size_of::<&str>()    = {}   (pointer + length, no capacity)", size_of::<&str>());
    println!("   size_of::<String>()  = {}   (pointer + length + capacity)", size_of::<String>());
    println!("   first_word(&s) = {:?}   <- borrowed from `s`, nothing copied", first_word(&s));

    println!("\n3. The range shorthands name the same slice");
    let len = s.len();
    println!("   &s[0..5]  {:?}        &s[..5]  {:?}", &s[0..5], &s[..5]);
    println!("   &s[6..{len}] {:?}        &s[6..]  {:?}", &s[6..len], &s[6..]);
    println!("   &s[0..{len}] {:?}  &s[..]   {:?}", &s[0..len], &s[..]);

    println!("\n4. The indices are BYTES — the one way a slice panics");
    let word = "bete noir";
    let accented = "bête noir";
    println!("   {:?}   {} bytes, {} chars   <- ASCII: they agree", word, word.len(), word.chars().count());
    println!("   {:?}   {} bytes, {} chars   <- they do not", accented, accented.len(), accented.chars().count());
    for (i, c) in accented.char_indices().take(4) {
        println!("      byte {i} -> {c:?} ({} byte(s))", c.len_utf8());
    }
    panic::set_hook(Box::new(|_| {}));
    let cut = panic::catch_unwind(|| accented[0..2].to_string());
    let _ = panic::take_hook();
    match cut {
        Ok(v) => println!("   &accented[0..2] = {v:?}"),
        Err(_) => println!("   &accented[0..2] PANICKED — byte 2 is inside 'ê', not a boundary"),
    }
    println!("   accented.get(0..2) = {:?}      <- the total version: no panic", accented.get(0..2));
    println!("   accented.get(0..3) = {:?}   <- 3 is a boundary", accented.get(0..3));
    println!("   is_char_boundary: 2 -> {}, 3 -> {}",
             accented.is_char_boundary(2), accented.is_char_boundary(3));

    println!("\n5. A literal is already a slice");
    let literal: &'static str = "hello world";
    println!("   literal        &'static str   {:?}", literal);
    println!("   &literal[..5]  &str           {:?}   <- slicing a literal needs no String", &literal[..5]);
    println!("   first_word(literal)      = {:?}", first_word(literal));
    println!("   first_word(&s)           = {:?}   <- &String coerced to &str", first_word(&s));
    println!("   first_word(&s[6..])      = {:?}   <- a slice of a slice", first_word(&s[6..]));

    println!("\n6. Slices are not a string feature");
    let a = [1, 2, 3, 4, 5];
    let part: &[i32] = &a[1..3];
    println!("   let a = [1, 2, 3, 4, 5];");
    println!("   &a[1..3] = {:?}   len {}", part, part.len());
    println!("   size_of::<&[i32]>() = {}   <- same two words as &str", size_of::<&[i32]>());
    println!("   &str is to String what &[T] is to Vec<T>: the borrowed view of the owned buffer.");
}
