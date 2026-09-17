//! Drawing `sret`: the room a returned value lands in, written out by hand.
//!
//! A real `-> [u64; 3]` return is invisible from Rust: the hidden pointer the
//! caller passes has no name in the source. So this program writes the same
//! thing the compiler writes -- a caller that reserves the room, a callee that
//! fills it through a pointer and hands the pointer back -- and checks each
//! arrow of the drawing as a comparison. No address is printed: a raw address
//! changes from run to run, and a comparison does not.
//!
//!   rustc --edition 2024 drawing_the_return_slot.rs -o /tmp/dtrs && /tmp/dtrs

use std::mem::{MaybeUninit, size_of};
use std::ptr;

/// What the compiler compiles `fn three_numbers() -> [u64; 3]` into, with the
/// hidden pointer given a name: `out` plays `rdi`, and the returned reference
/// plays `rax`.
#[inline(never)]
fn three_numbers_into(out: &mut [u64; 3]) -> &mut [u64; 3] {
    *out = [7, 8, 9];
    out
}

/// The same, with a room nobody has written to yet -- which is what `sret`
/// really hands over. `MaybeUninit::write` fills it and returns `&mut` to the
/// filled value: the `mov rax, rdi` of the drawing.
#[inline(never)]
fn three_numbers_into_uninit(slot: &mut MaybeUninit<[u64; 3]>) -> &mut [u64; 3] {
    slot.write([7, 8, 9])
}

fn main() {
    println!("1. The room main reserves for `let t: [u64; 3]`");
    let t = [0u64; 3];
    let base = t.as_ptr() as usize;
    let offset = |i: usize| (&t[i] as *const u64 as usize) - base;
    println!("   {} bytes; t[0] at +{}, t[1] at +{}, t[2] at +{}",
             size_of::<[u64; 3]>(), offset(0), offset(1), offset(2));

    println!();
    println!("2. sret by hand, with a room main has to zero first");
    let mut t = [0u64; 3];
    println!("   before the call: {t:?}");
    let back = three_numbers_into(&mut t) as *const [u64; 3];
    println!("   the pointer handed back is main's t: {}", ptr::eq(back, &t));
    println!("   after the call:  {t:?}");

    println!();
    println!("3. sret by hand, with a room nobody has written to");
    let mut slot = MaybeUninit::<[u64; 3]>::uninit();
    let room = slot.as_ptr();
    let filled = three_numbers_into_uninit(&mut slot);
    println!("   write() hands back the room it filled: {}", ptr::eq(&*filled, room));
    println!("   read back through it: {:?}", *filled);
}
