//! Kata solution: split off the header.
//!
//! Both functions return the first `n` bytes and advance the caller's slice past
//! them, or return `None` and leave the caller's slice as it was. The trap is in
//! the second: `mem::take` empties the caller's slice first, so a `?` after it
//! returns `None` and leaves the caller holding the empty slice.
//! `take_header_mut_early` below is that version, run.
//!
//!   rustc --edition 2024 repointing_a_slice_kata.rs -o /tmp/repointing_a_slice_kata && /tmp/repointing_a_slice_kata

use std::mem;

/// Shared: `&'a [u8]` is `Copy`, so `split_at_checked` works on a copy of the
/// caller's slice and both halves keep `'a`. No `take` needed.
fn take_header<'a>(input: &mut &'a [u8], n: usize) -> Option<&'a [u8]> {
    let (header, rest) = input.split_at_checked(n)?; // `None` returns before any write
    *input = rest;
    Some(header)
}

/// Exclusive: check the length while the caller's slice is still in place, then
/// take it. Past the check, `split_at_mut` cannot panic.
fn take_header_mut<'a>(input: &mut &'a mut [u8], n: usize) -> Option<&'a mut [u8]> {
    if input.len() < n {
        return None;
    }
    let (header, rest) = mem::take(input).split_at_mut(n);
    *input = rest;
    Some(header)
}

/// Compiles, and breaks the contract: on `None` the `&mut []` that `take` left
/// behind is what the caller keeps.
fn take_header_mut_early<'a>(input: &mut &'a mut [u8], n: usize) -> Option<&'a mut [u8]> {
    let (header, rest) = mem::take(input).split_at_mut_checked(n)?;
    *input = rest;
    Some(header)
}

fn main() {
    println!("1. take_header on &mut &[u8]");
    let packet: [u8; 6] = [0xCA, 0xFE, 0x00, 0x02, 0x68, 0x69];
    let mut rest: &[u8] = &packet;
    let magic = take_header(&mut rest, 2);
    let len = take_header(&mut rest, 2);
    println!("   take 2, take 2                 -> {magic:02X?}, {len:02X?}");
    println!("   rest                           -> {rest:02X?}");
    let too_long = take_header(&mut rest, 9);
    println!("   take 9                         -> {too_long:?}, rest still {rest:02X?}");

    println!();
    println!("2. take_header_mut on &mut &mut [u8]");
    let mut buf = [0u8; 6];
    let mut view: &mut [u8] = &mut buf;
    let magic = take_header_mut(&mut view, 2).unwrap();
    let len = take_header_mut(&mut view, 2).unwrap(); // `magic` is still live: both have 'a
    magic.copy_from_slice(&[0xCA, 0xFE]);
    len.copy_from_slice(&2u16.to_be_bytes());
    println!("   take 2, take 2, write both     -> view left {} long", view.len());
    let too_long = take_header_mut(&mut view, 9);
    println!("   take 9                         -> {too_long:?}, view still {} long", view.len());
    view.copy_from_slice(b"hi");
    println!("   buf                            -> {buf:02X?}");

    println!();
    println!("3. The early `?` after mem::take");
    let mut buf = [0u8; 6];
    let mut view: &mut [u8] = &mut buf;
    let too_long = take_header_mut_early(&mut view, 9);
    println!("   take 9                         -> {too_long:?}, view now {} long", view.len());
}
