//! What `Vec` promises, and what it only happens to do.
//!
//!   rustc --edition 2024 vec_guarantees.rs -o /tmp/vg && /tmp/vg
//!
//! Every claim here is checked as a *relationship* rather than as a raw number,
//! because addresses, word sizes and allocator behaviour differ between
//! machines and this file's output is an answer key CI diffs on another one.

use std::mem::{align_of, size_of};

struct Loud(u8);
impl Drop for Loud {
    fn drop(&mut self) {
        print!("{} ", self.0);
    }
}

/// True when the vector has no real allocation behind it — std parks the
/// pointer of an unallocated `Vec` at the element type's alignment rather than
/// at null. That is an implementation detail; the *guarantee* is only that the
/// pointer is never null.
fn is_dangling<T>(v: &Vec<T>) -> bool {
    v.as_ptr() as usize == align_of::<T>()
}

fn main() {
    println!("1. Three numbers, and the order of them is not yours to know");
    let v = vec![1u32, 2, 3];
    println!("   ptr/len/cap: len {} cap {}", v.len(), v.capacity());
    println!("   `Vec` is and always will be a (pointer, capacity, length) triplet —");
    println!("   no more, no less. The order of the fields is unspecified, and std");
    println!("   says outright that the ABI is not stable. Read them with the");
    println!("   methods; never assume a layout.");

    println!();
    println!("2. The pointer is never null, so Option<Vec<T>> is free");
    println!("   size_of::<Option<Vec<u8>>>() == size_of::<Vec<u8>>(): {}",
             size_of::<Option<Vec<u8>>>() == size_of::<Vec<u8>>());
    println!("   That is the null-pointer optimization: `None` is spelled with the");
    println!("   one bit pattern the pointer promises never to hold, so the wrapper");
    println!("   costs nothing.");

    println!();
    println!("3. ...but the pointer need not point at an allocation");
    let a: Vec<u32> = Vec::new();
    let b: Vec<u32> = vec![];
    let c: Vec<u32> = Vec::with_capacity(0);
    let mut d: Vec<u32> = Vec::with_capacity(9);
    d.shrink_to_fit();
    println!("   never allocated — new() {} vec![] {} with_capacity(0) {} shrunk-empty {}",
             is_dangling(&a), is_dangling(&b), is_dangling(&c), is_dangling(&d));
    let real: Vec<u32> = Vec::with_capacity(4);
    println!("   with_capacity(4) did allocate: {}", !is_dangling(&real));
    println!("   The rule is exact: a Vec allocates if and only if");
    println!("   size_of::<T>() * capacity() > 0.");

    println!();
    println!("4. A zero-sized element never allocates, however many you push");
    let mut z: Vec<()> = Vec::new();
    for _ in 0..1_000 {
        z.push(());
    }
    println!("   pushed {} units: still unallocated {}, capacity is usize::MAX {}",
             z.len(), is_dangling(&z), z.capacity() == usize::MAX);
    println!("   Note what this breaks: \"capacity() == 0 means it has not");
    println!("   allocated\" is false here. The size*capacity rule still holds.");

    println!();
    println!("5. Nothing is ever stored inline");
    println!("   size_of::<Vec<[u8; 4096]>>() == size_of::<Vec<u8>>(): {}",
             size_of::<Vec<[u8; 4096]>>() == size_of::<Vec<u8>>());
    println!("   `Vec` will never do the \"small\" optimization some C++ string and");
    println!("   vector implementations do, where a few elements live in the struct");
    println!("   itself. Two reasons std gives: the contents would stop having a");
    println!("   stable address when the Vec is merely moved, and every access would");
    println!("   pay for a branch. If you want that, it is a crate, not the std type.");

    println!();
    println!("6. capacity() is exact, so you can predict every reallocation");
    let mut v: Vec<u32> = Vec::with_capacity(3);
    for n in 0..4u32 {
        let (len, cap) = (v.len(), v.capacity());
        v.push(n);
        println!("   push #{n}: len {len} of cap {cap}  ->  cap {}   (len == cap? {})",
                 v.capacity(), len == cap);
    }
    println!("   `push` and `insert` reallocate when len == capacity and never");
    println!("   otherwise. The reported capacity is completely accurate and can be");
    println!("   relied on — which is what makes `with_capacity` a promise rather");
    println!("   than a hint. Bulk insertions are the carve-out: std reserves the");
    println!("   right to reallocate in those even when it need not.");

    println!();
    println!("7. It never shrinks itself");
    let mut v = vec![0u8; 64];
    v.clear();
    println!("   after clear(): len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("   after shrink_to_fit(): len {} cap {}", v.len(), v.capacity());
    println!("   Emptying a Vec and refilling it to the same length should cost the");
    println!("   allocator nothing — which is the behaviour you want in a loop, and");
    println!("   the reason handing memory back has to be asked for.");

    println!();
    println!("8. When len == capacity, Box<[T]> is free in both directions");
    let exact = vec![1u32, 2, 3];
    println!("   vec![1,2,3]: len {} cap {} — equal, so into_boxed_slice()",
             exact.len(), exact.capacity());
    let boxed = exact.into_boxed_slice();
    let back = Vec::from(boxed);
    println!("   neither reallocates nor moves the elements. Back again: len {} cap {}",
             back.len(), back.capacity());
    let mut slack: Vec<u32> = Vec::with_capacity(10);
    slack.extend([1, 2, 3]);
    println!("   with slack (len 3 of cap {}), the conversion has to shrink first,",
             slack.capacity());
    let back = Vec::from(slack.into_boxed_slice());
    println!("   and the spare capacity is gone for good: len {} cap {}",
             back.len(), back.capacity());

    println!();
    println!("9. The one thing you may do to the spare capacity");
    let mut buf: Vec<u8> = Vec::with_capacity(4);
    for (i, slot) in buf.spare_capacity_mut().iter_mut().enumerate() {
        slot.write(b'a' + i as u8);
    }
    // SAFETY: the loop above initialised every one of the 4 slots the capacity
    // guarantees are there. Writing to the excess capacity and then raising the
    // length to match is explicitly always valid.
    unsafe { buf.set_len(4) };
    println!("   wrote into the excess capacity, then set_len: {:?}",
             String::from_utf8(buf).unwrap());

    println!();
    println!("10. Three things that are NOT promises");
    println!("   The growth factor. Doubling is this std's current choice; the only");
    println!("   promise is amortised O(1) push.");
    print!("   The drop order — today it is front to back: ");
    drop(vec![Loud(1), Loud(2), Loud(3)]);
    println!();
    println!("   ...but std says the order has changed before and may change again.");
    println!("   And erasure. Removed elements are neither overwritten nor preserved;");
    println!("   the freed buffer may be handed to the next allocation as it stands.");
    println!("   Zeroing it yourself is not reliable either — the optimizer is free to");
    println!("   delete a write nothing reads. Use a crate built for the job.");
}
