//! Kata solution: predict nine sizes, then build a megabyte on a 64 KiB stack.
//!
//!   rustc --edition 2024 where_an_array_lives_kata.rs -o /tmp/walk && /tmp/walk

use std::mem::size_of;
use std::rc::Rc;
use std::thread;

const MIB: usize = 1 << 20;

/// Builds a zeroed megabyte without ever holding it in a stack frame:
/// `vec!` allocates and zeroes on the heap, and the conversions only move the
/// pointer (and the length, until `try_into` puts the length back in the type).
fn megabyte() -> Box<[u8; MIB]> {
    vec![0u8; MIB].into_boxed_slice().try_into().expect("exactly MIB long")
}

fn main() {
    println!("1. Nine sizes, predicted before they were printed");
    let rows: [(&str, usize, &str); 9] = [
        ("[u16; 4]", size_of::<[u16; 4]>(), "the four u16s"),
        ("[[u16; 4]; 2]", size_of::<[[u16; 4]; 2]>(), "eight u16s, still no header"),
        ("(u8, [u16; 4])", size_of::<(u8, [u16; 4])>(), "nine bytes of data, padded to u16's alignment"),
        ("&[u16; 4]", size_of::<&[u16; 4]>(), "a pointer; the 4 is in the type"),
        ("&[u16]", size_of::<&[u16]>(), "a pointer and a length"),
        ("Box<[u16; 4]>", size_of::<Box<[u16; 4]>>(), "thin, like the reference"),
        ("Box<[u16]>", size_of::<Box<[u16]>>(), "fat, like the slice reference"),
        ("Option<Box<[u16; 4]>>", size_of::<Option<Box<[u16; 4]>>>(), "None is the null pointer a Box never is"),
        ("Vec<[u16; 4]>", size_of::<Vec<[u16; 4]>>(), "three words; the rows are on the heap"),
    ];
    for (ty, size, why) in rows {
        println!("   {ty:<22} {size:>2}  {why}");
    }
    println!("   and Rc<[u16]> = {}: fat too; the counts live on the heap beside the elements", size_of::<Rc<[u16]>>());

    println!();
    println!("2. A megabyte on a thread with a 64 KiB stack");
    let (len, sum, box_size) = thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut page = megabyte();
            page[MIB - 1] = 42;
            (page.len(), page.iter().map(|&b| u64::from(b)).sum::<u64>(), size_of::<Box<[u8; MIB]>>())
        })
        .expect("spawn")
        .join()
        .expect("the thread did not overflow");
    println!("   len {len}, sum {sum}, and the Box on that small stack is {box_size} bytes");
    println!("   Box::new([0u8; MIB]) in the same closure aborts a debug build:");
    println!("   the array is built in the closure's frame first, then moved in.");
}
