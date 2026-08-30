//! `unsafe` grants five powers and turns off nothing else.
//!
//!   rustc --edition 2024 what_unsafe_turns_off.rs -o /tmp/uns && /tmp/uns

/// Two mutable slices from one, which cannot be written in safe Rust: the
/// borrow checker cannot see that the halves do not overlap.
fn split_at_mut(v: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    let len = v.len();
    assert!(mid <= len, "mid must be within the slice");   // <- the proof
    let ptr = v.as_mut_ptr();
    // SAFETY: mid <= len is checked above, so the two ranges are disjoint and
    // both lie inside one allocation that we hold a &mut to for 'a.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// An `unsafe fn` announces a contract the CALLER must keep.
///
/// # Safety
/// `i` must be less than `v.len()`.
unsafe fn get_unchecked(v: &[u32], i: usize) -> u32 {
    // Edition 2024: the body of an unsafe fn is NOT implicitly an unsafe
    // block, so this needs its own — which is the point of the change.
    unsafe { *v.as_ptr().add(i) }
}

fn main() {
    println!("1. The five things `unsafe` lets you do, and nothing else");
    println!("   a. dereference a raw pointer");
    println!("   b. call an `unsafe fn`");
    println!("   c. implement an `unsafe trait`  (Send, Sync, and your own)");
    println!("   d. read a field of a `union`");
    println!("   e. access or modify a mutable `static`");
    println!("   That is the whole list. Everything else — the borrow checker,");
    println!("   type checking, lifetimes on references, move semantics — is still");
    println!("   on inside an `unsafe` block. It is not \"turn off Rust\".");

    println!();
    println!("2. Making a raw pointer is safe; using one is not");
    let x = 5u32;
    let p: *const u32 = &x;
    println!("   let p: *const u32 = &x;   <- safe, no unsafe needed");
    println!("   p.is_null() = {}", p.is_null());
    println!("   *p                        <- E0133 without an unsafe block");
    println!("   unsafe {{ *p }} = {}", unsafe { *p });
    println!("   Creating a pointer cannot break anything. DEREFERENCING one is");
    println!("   where the promise is made, which is why that is the operation the");
    println!("   keyword guards.");

    println!();
    println!("3. What the block is actually for");
    let mut scores = [5u32, 3, 0, 4, 2, 1];
    let (left, right) = split_at_mut(&mut scores, 3);
    left[0] = 50;
    right[0] = 40;
    println!("   split_at_mut(&mut scores, 3) -> two &mut into one array");
    println!("   after writing through both: {scores:?}");
    println!("   Safe Rust cannot express this: the borrow checker sees two &mut");
    println!("   borrows of the same slice and refuses, because it cannot know the");
    println!("   ranges are disjoint. The `assert!` above is the proof it lacks —");
    println!("   and the SAFETY comment says which invariant the block depends on.");
    println!("   (std has this as slice::split_at_mut, written the same way.)");

    println!();
    println!("4. `unsafe fn` versus `unsafe {{}}` — two different jobs");
    println!("   unsafe {{ … }}  I have checked the contract here.");
    println!("   unsafe fn      CALLING me has a contract; read the # Safety doc.");
    println!("   In edition 2024 the body of an `unsafe fn` is no longer implicitly");
    println!("   an unsafe block, so the two are finally separate. Omit the inner");
    println!("   block and rustc says so, with the sentence that explains the whole");
    println!("   change: \"an unsafe function restricts its caller, but its body is");
    println!("   safe by default\" — the `unsafe_op_in_unsafe_fn` lint, warn by");
    println!("   default under rust_2024_compatibility.");
    println!("   get_unchecked(&scores, 1) = {}", unsafe { get_unchecked(&scores, 1) });

    println!();
    println!("5. The unit of review is the module, not the block");
    println!("   `split_at_mut` is sound because of the `assert!` three lines above");
    println!("   the block — safe code. Delete the assert, and the unsafe block is");
    println!("   unchanged and now unsound. So an `unsafe` block's correctness can");
    println!("   depend on any safe code that can reach the same private state,");
    println!("   which is every line in its module.");
    println!("   Two consequences worth acting on: keep modules containing `unsafe`");
    println!("   SMALL, and write a `// SAFETY:` comment naming the invariant, so");
    println!("   the next reader knows what they must not break.");

    println!();
    println!("6. And what it still does not buy you");
    let v = vec![1u32, 2, 3];
    println!("   v = {v:?} — inside an unsafe block, `let a = v; let b = v;` is");
    println!("   still E0382 use-after-move; `&mut v` twice is still E0499; a");
    println!("   reference still cannot outlive its referent. `unsafe` does not");
    println!("   relax one rule of the borrow checker. It moves five specific");
    println!("   obligations from the compiler to you, and leaves the rest.");
}
