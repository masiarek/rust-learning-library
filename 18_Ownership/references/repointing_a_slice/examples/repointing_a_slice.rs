//! Re-pointing a slice: a `&mut` to the caller's slice reference lets a function
//! move where the caller's view starts and ends. Every form below compiles; the
//! refusals live on the page, as rustc output.
//!
//!   rustc --edition 2024 repointing_a_slice.rs -o /tmp/repointing_a_slice && /tmp/repointing_a_slice

use std::io::{Read, Write};

/// Moves the caller's view one element to the right.
fn skip_first(t: &mut &mut [i32]) {
    let whole = std::mem::take(t); // the caller's `&mut [i32]`, by value; `&mut []` left behind
    *t = &mut whole[1..];
}

/// Writes elements, then re-points its own copy of the reference.
fn rebind_local(mut t: &mut [i32]) {
    t[0] = 99;
    t = &mut t[1..];
    t[0] = 42;
}

fn grow(v: &mut Vec<i32>) {
    v.push(4);
}

/// Hands out the first element with the caller's lifetime `'a`, and shortens the view.
fn pop_front<'a>(t: &mut &'a mut [i32]) -> Option<&'a mut i32> {
    let (first, rest) = std::mem::take(t).split_first_mut()?;
    *t = rest;
    Some(first)
}

/// The shared version: `&[i32]` is `Copy`, so the naive re-slice compiles.
fn skip_first_shared(t: &mut &[i32]) {
    *t = &t[1..];
}

/// A parser cursor: returns the next space-separated word and advances `input` past it.
fn next_word<'a>(input: &mut &'a str) -> Option<&'a str> {
    let s = input.trim_start();
    if s.is_empty() {
        *input = s;
        return None;
    }
    let end = s.find(' ').unwrap_or(s.len());
    let (word, rest) = s.split_at(end);
    *input = rest;
    Some(word)
}

fn main() {
    println!("1. &mut &mut [i32]: the function moves the caller's view");
    let mut data = [1, 2, 3];
    let mut view: &mut [i32] = &mut data;
    skip_first(&mut view);
    println!("   skip_first(&mut view)          -> view = {view:?}");

    println!();
    println!("2. &mut [i32]: elements yes, the caller's view no");
    let mut data = [1, 2, 3];
    let view: &mut [i32] = &mut data;
    rebind_local(view);
    println!("   rebind_local(view)             -> view = {view:?}");
    println!("   `t = &mut t[1..]` re-pointed the function's copy; view is still 3 long");

    println!();
    println!("3. &mut Vec<i32>: the one that can grow");
    let mut v = vec![1, 2, 3];
    grow(&mut v);
    println!("   grow(&mut v)                   -> v = {v:?}");

    println!();
    println!("4. Handing out elements that outlive the call");
    let mut data = [1, 2, 3];
    let mut view: &mut [i32] = &mut data;
    let a = pop_front(&mut view).unwrap();
    let b = pop_front(&mut view).unwrap();
    *a += 10;
    *b += 20;
    println!("   a, b popped, both still live   -> view = {view:?}");
    println!("   after *a += 10 and *b += 20    -> data = {data:?}");

    println!();
    println!("5. The shared version: &mut &[i32], and &mut &str as a parser cursor");
    let data = [1, 2, 3];
    let mut view: &[i32] = &data;
    skip_first_shared(&mut view);
    println!("   skip_first_shared(&mut view)   -> view = {view:?}");
    let line = String::from("GET /index.html HTTP/1.1");
    let mut cursor: &str = &line;
    let method = next_word(&mut cursor);
    let target = next_word(&mut cursor);
    println!("   two next_word calls            -> {method:?}, {target:?}");
    println!("   cursor                         -> {cursor:?}");
    let version = next_word(&mut cursor);
    let past_end = next_word(&mut cursor);
    println!("   two more                       -> {version:?}, {past_end:?}, cursor = {cursor:?}");

    println!();
    println!("6. std: `impl Read for &[u8]` receives &mut &[u8]");
    let mut src: &[u8] = b"hello";
    let mut buf = [0u8; 2];
    let n = src.read(&mut buf).unwrap();
    println!("   src.read(&mut buf)             -> read {n}: {:?}", std::str::from_utf8(&buf).unwrap());
    println!("   src afterwards                 -> {:?}, len {}", std::str::from_utf8(src).unwrap(), src.len());

    println!();
    println!("7. std: `impl Write for &mut [u8]` receives &mut &mut [u8]");
    let mut out = [b'.'; 5];
    let mut dst: &mut [u8] = &mut out;
    let n = dst.write(b"hi").unwrap();
    println!("   dst.write(b\"hi\")               -> wrote {n}, dst len now {}", dst.len());
    println!("   out                            -> {:?}", std::str::from_utf8(&out).unwrap());

    println!();
    println!("Checkpoint. skip_first twice on [1, 2, 3]: what do view and data print?");
    let mut data = [1, 2, 3];
    let mut view: &mut [i32] = &mut data;
    skip_first(&mut view);
    skip_first(&mut view);
    println!("   view = {view:?}");
    println!("   data = {data:?}  (the view moved; nothing was removed from data)");
}
