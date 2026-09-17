//! A reference is aligned to what it points at — align_of::<T>() — not to usize.
//!
//! No line prints an address: every check is a remainder or a comparison that
//! comes out the same on every run.
//!
//!   rustc --edition 2024 aligned_to_the_referent.rs -o /tmp/atr && /tmp/atr

use std::mem::{align_of, offset_of, size_of};

#[allow(dead_code)]
struct Header {
    tag: u8,
    count: u32,
    flag: u8,
}

#[allow(dead_code)]
#[repr(C)]
struct HeaderC {
    tag: u8,
    count: u32,
    flag: u8,
}

#[allow(dead_code)]
#[repr(C, packed)]
struct Packed {
    tag: u8,
    count: u32,
}

/// Starts on a multiple of 8, so the byte after its first field never can.
#[repr(C, align(8))]
struct OnAnEightBoundary(Packed);

fn main() {
    println!("1. Each type has its own alignment");
    for (ty, align) in [
        ("u8", align_of::<u8>()),
        ("[u8; 10]", align_of::<[u8; 10]>()),
        ("u16", align_of::<u16>()),
        ("u32", align_of::<u32>()),
        ("[u32; 3]", align_of::<[u32; 3]>()),
        ("u64", align_of::<u64>()),
        ("usize", align_of::<usize>()),
        ("&u8", align_of::<&u8>()),
    ] {
        println!("   align_of::<{ty}>(){:w$} = {align}", "", w = 10 - ty.len());
    }
    println!("   A &u8 is itself 8-aligned; the u8 it points at needs only 1.");

    println!();
    println!("2. So a &u8 may sit at any address");
    let bytes = [0u8; 2];
    let first = &bytes[0] as *const u8 as usize;
    let second = &bytes[1] as *const u8 as usize;
    println!("   &bytes[1] - &bytes[0] = {}", second - first);
    println!("   at least one of the two is not a multiple of 8: {}", first % 8 != 0 || second % 8 != 0);
    println!("   and both are perfectly good references.");

    println!();
    println!("3. Every reference meets ITS type's alignment");
    let small = 1u16;
    let mid = 2u32;
    let big = 3u64;
    println!("   (&small as *const u16).is_aligned() {}", (&small as *const u16).is_aligned());
    println!("   (&mid   as *const u32).is_aligned() {}", (&mid as *const u32).is_aligned());
    println!("   (&big   as *const u64).is_aligned() {}", (&big as *const u64).is_aligned());

    println!();
    println!("4. Padding is how a struct keeps each field aligned");
    println!("   #[repr(C)]  HeaderC  size {:>2}  tag @{} count @{} flag @{}",
        size_of::<HeaderC>(), offset_of!(HeaderC, tag), offset_of!(HeaderC, count), offset_of!(HeaderC, flag));
    println!("   repr(Rust)  Header   size {:>2}  tag @{} count @{} flag @{}",
        size_of::<Header>(), offset_of!(Header, tag), offset_of!(Header, count), offset_of!(Header, flag));
    println!("   repr(C) keeps the written order: 3 bytes of padding before count, 3");
    println!("   after flag. Rust's default layout may reorder, and on rustc 1.98.0 it");
    println!("   put count first and needs only 2 bytes of padding.");

    println!();
    println!("5. #[repr(packed)] removes the padding, and with it the alignment");
    println!("   Packed  size {}  align {}  count @{}",
        size_of::<Packed>(), align_of::<Packed>(), offset_of!(Packed, count));
    let boxed = OnAnEightBoundary(Packed { tag: 1, count: 42 });
    let place = &raw const boxed.0.count; // a raw pointer: `&boxed.0.count` is E0793
    println!("   &raw const ...count  is_aligned() {}", place.is_aligned());
    println!("   place.read_unaligned()  = {}", unsafe { place.read_unaligned() });
    let copied = { boxed.0.count };
    println!("   {{ boxed.0.count }} copied out = {copied}");
    println!("   A &u32 there would break the promise every &u32 makes, so rustc");
    println!("   refuses to create one; the raw pointer and read_unaligned do not");
    println!("   promise alignment, so they are allowed.");
}
