//! Kata solution: regroup a flat byte buffer into fixed-width rows, and
//! send both shapes the allocation bill.
//!
//!   rustc --edition 2024 vec_of_arrays_kata.rs -o /tmp/voak && /tmp/voak

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Split a flat buffer into four-byte rows plus whatever did not divide.
///
/// `chunks_exact` guarantees every chunk is four bytes long and the type
/// system cannot see it, so `try_into` is where the guarantee is cashed.
/// `expect` is honest here in a way `unwrap` on user input never is: the
/// iterator's own contract is what makes it unreachable.
fn regroup(bytes: &[u8]) -> (Vec<[u8; 4]>, &[u8]) {
    let rows = bytes
        .chunks_exact(4)
        .map(|c| c.try_into().expect("chunks_exact(4) yields four bytes"))
        .collect();
    (rows, bytes.chunks_exact(4).remainder())
}

fn dotted(addr: &[u8; 4]) -> String {
    addr.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(".")
}

fn main() {
    println!("1. A short row at the end of the buffer");
    let mut wire: Vec<u8> = vec![192, 168, 0, 1, 10, 0, 0, 1, 127, 0, 0, 1];
    wire.extend_from_slice(&[8, 8]); // the read stopped mid-address
    let (rows, leftover) = regroup(&wire);
    println!("   {} bytes in", wire.len());
    for row in &rows {
        println!("     {}", dotted(row));
    }
    println!("   leftover: {leftover:?} — {} bytes, not enough for a row",
             leftover.len());
    println!("   Nothing was dropped silently: remainder() is the part the");
    println!("   iterator refused, and it is a slice you can keep and prepend");
    println!("   to the next read.");

    println!();
    println!("2. Why the obvious one-liner does not compile");
    println!("   bytes.chunks_exact(4).collect::<Vec<[u8; 4]>>()");
    println!("   -> error[E0277]: a value of type `Vec<[u8; 4]>` cannot be");
    println!("      built from an iterator over elements of type `&[u8]`");
    println!("   A &[u8] has its length in the value; a [u8; 4] has it in the");
    println!("   type. Converting between them is exactly what try_into does,");
    println!("   and it returns a Result because in general it can fail.");

    println!();
    println!("3. The bill, for a thousand addresses");
    let flat: Vec<u8> = (0..4000u32).map(|b| b as u8).collect();
    let before = ALLOCS.load(Relaxed);
    let packed: Vec<[u8; 4]> = flat.chunks_exact(4).map(|c| c.try_into().unwrap()).collect();
    let mid = ALLOCS.load(Relaxed);
    let nested: Vec<Vec<u8>> = flat.chunks_exact(4).map(|c| c.to_vec()).collect();
    let after = ALLOCS.load(Relaxed);
    println!("   shape          rows   allocations");
    println!("   Vec<[u8; 4]>   {:>4}   {:>4}", packed.len(), mid - before);
    println!("   Vec<Vec<u8>>   {:>4}   {:>4}", nested.len(), after - mid);
    println!("   One, not two: chunks_exact knows its own length, so collect");
    println!("   reserves the whole buffer up front instead of doubling into");
    println!("   it. The nested form pays that once too — the extra thousand");
    println!("   are the rows themselves, each its own block.");

    println!();
    println!("4. Fixed-width rows sort and dedup for free");
    let mut seen: Vec<[u8; 4]> = vec![
        [10, 0, 0, 1],
        [192, 168, 0, 1],
        [10, 0, 0, 1],
        [127, 0, 0, 1],
    ];
    seen.sort();
    seen.dedup();
    let listed: Vec<String> = seen.iter().map(dotted).collect();
    println!("   {}", listed.join("  "));
    println!("   [u8; 4] is Ord and Eq because u8 is, so the derive ladder");
    println!("   reaches the row without anyone writing an impl.");

    println!();
    println!("5. What only Vec<Vec<u8>> could have held");
    let ragged: Vec<Vec<u8>> = vec![vec![10, 0, 0, 1], vec![255, 255]];
    println!("   {:?} — row lengths {:?}", ragged,
             ragged.iter().map(|r| r.len()).collect::<Vec<_>>());
    println!("   Here that is not a feature. An address that is not four bytes");
    println!("   is a bug, and Vec<[u8; 4]> makes it one the compiler catches:");
    println!("   push([10, 0, 0]) is error[E0308], expected an array with a");
    println!("   size of 4, found one with a size of 3.");
}
