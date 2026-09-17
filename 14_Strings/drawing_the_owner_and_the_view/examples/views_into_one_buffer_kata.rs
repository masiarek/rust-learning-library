//! Kata solution: split a String into words and find every word inside the
//! one buffer they share.
//!
//!   rustc --edition 2024 views_into_one_buffer_kata.rs -o /tmp/vob && /tmp/vob

use std::mem::size_of;

/// Where `view` starts, in bytes from the start of `owner`'s buffer.
fn offset_in(owner: &str, view: &str) -> Option<usize> {
    let buffer = owner.as_bytes().as_ptr_range();
    let start = view.as_ptr();
    buffer.contains(&start).then(|| start.addr() - buffer.start.addr())
}

fn main() {
    let line = String::from("zażółć gęślą jaźń");
    let words: Vec<&str> = line.split(' ').collect();

    println!("1. Three views into one buffer of {} bytes", line.len());
    println!("   {:<10} {:>6} {:>4} {:>6}", "word", "offset", "len", "chars");
    for word in &words {
        let offset = offset_in(&line, word).expect("every word points into line");
        println!("   {:<10} {offset:>6} {:>4} {:>6}", format!("{word:?}"), word.len(), word.chars().count());
    }

    println!();
    println!("2. The second buffer");
    let handle_words = size_of::<&str>() / size_of::<usize>();
    println!("   words holds {} handles of {handle_words} machine words each", words.len());
    let handles = words.as_ptr().cast::<u8>();
    println!("   and they sit in a buffer of their own: inside line's buffer? {}",
        line.as_bytes().as_ptr_range().contains(&handles));
    println!("   Text bytes copied to make the three words: none. Heap buffers: two,");
    println!("   line's text and the Vec's array of handles.");

    println!();
    println!("3. What owning them would cost");
    let owned: Vec<String> = words.iter().map(|w| (*w).to_owned()).collect();
    let separate = owned.iter().all(|w| offset_in(&line, w).is_none());
    println!("   to_owned on each word: {} more buffers, none inside line's: {separate}", owned.len());
}
