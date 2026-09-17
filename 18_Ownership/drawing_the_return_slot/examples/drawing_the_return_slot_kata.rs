//! Kata solution: draw the room for `-> [Point; 2]`, then build it by hand.
//!
//! `#[repr(C)]` fixes the field order and offsets, so the drawing below is a
//! promise rather than today's layout. Every line is a size, an offset or a
//! comparison, so the answer key is the same on every run.
//!
//!   rustc --edition 2024 drawing_the_return_slot_kata.rs -o /tmp/dtrsk && /tmp/dtrsk

use std::mem::{MaybeUninit, offset_of, size_of};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// The drawing, before running anything:
//
//   room for [Point; 2], 16 bytes
//   +0   corners[0].x
//   +4   corners[0].y
//   +8   corners[1].x
//   +12  corners[1].y

/// `fn corners() -> [Point; 2]`, with the hidden pointer written out.
#[inline(never)]
fn corners_into(slot: &mut MaybeUninit<[Point; 2]>) -> &mut [Point; 2] {
    slot.write([Point { x: 0, y: 0 }, Point { x: 640, y: 480 }])
}

fn main() {
    let point = size_of::<Point>();
    println!("the room: {} bytes", size_of::<[Point; 2]>());
    for i in 0..2 {
        println!("   +{:<2}  corners[{i}].x", i * point + offset_of!(Point, x));
        println!("   +{:<2}  corners[{i}].y", i * point + offset_of!(Point, y));
    }

    let mut slot = MaybeUninit::<[Point; 2]>::uninit();
    let room = slot.as_ptr();
    let corners = corners_into(&mut slot);
    println!("the callee filled main's room and handed it back: {}", ptr::eq(&*corners, room));
    println!("{:?}", *corners);
}
