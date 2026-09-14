// C's strcpy writes strlen(src)+1 bytes into the destination and never asks how
// big it is. Rust has no such function: a write into a fixed buffer states an
// index or a length, and one past the end is a checked panic AT THE WRITE, not a
// silent overwrite of whatever sat after it. The panic hook is silenced so the
// program's own lines are its whole output.
use std::panic::{catch_unwind, AssertUnwindSafe, set_hook};

fn main() {
    let name = b"Grace Brewster Murray Hopper"; // 28 bytes, like the C source
    let mut buf = [0u8; 8];
    set_hook(Box::new(|_| {}));

    // 1. The byte-by-byte copy, the shape of strcpy's loop. The index is checked
    //    on every write, so it panics at the 9th -- before touching the 9th byte.
    let hit_the_wall = catch_unwind(AssertUnwindSafe(|| {
        let mut b = [0u8; 8];
        for (i, &byte) in name.iter().enumerate() {
            b[i] = byte;            // panics when i == 8
        }
    }))
    .is_err();
    println!("byte-by-byte copy of {} bytes into [u8; 8]: {}",
             name.len(),
             if hit_the_wall { "panicked at the 9th write, buffer's end intact" } else { "ok" });

    // 2. The bulk copy. copy_from_slice requires the two slices be equal length,
    //    so a source that does not fit cannot even be handed to it.
    let refused = catch_unwind(AssertUnwindSafe(|| {
        let mut b = [0u8; 8];
        b.copy_from_slice(name);    // 28 into 8: panics, writes nothing
    }))
    .is_err();
    println!("buf.copy_from_slice(28 bytes into 8): {}",
             if refused { "panicked, nothing written" } else { "ok" });

    // 3. The checked form: copy exactly what fits, and know how much that was.
    let n = name.len().min(buf.len());
    buf[..n].copy_from_slice(&name[..n]);
    println!("copied {n} of {} bytes: {:?}", name.len(), std::str::from_utf8(&buf[..n]).unwrap());

    // 4. Or let the destination grow -- the strcat that cannot overflow, because
    //    the Vec owns its storage and reallocates.
    let mut v = Vec::new();
    v.extend_from_slice(name);
    println!("Vec::extend_from_slice: len {}, holds {:?}", v.len(), std::str::from_utf8(&v).unwrap());
}
