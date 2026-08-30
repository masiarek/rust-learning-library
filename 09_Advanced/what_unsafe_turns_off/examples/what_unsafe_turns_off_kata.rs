//! Kata solution: the safe line the unsafe block depends on.
//!
//!   rustc --edition 2024 what_unsafe_turns_off_kata.rs -o /tmp/uk && /tmp/uk

mod ring {
    /// A fixed ring buffer whose reads skip the bounds check.
    pub struct Ring {
        slots: [u32; 4],
        head: usize,   // INVARIANT: head < slots.len()
    }

    impl Ring {
        pub fn new(slots: [u32; 4]) -> Self {
            Ring { slots, head: 0 }
        }

        /// Safe: it maintains the invariant itself.
        pub fn advance(&mut self) {
            self.head = (self.head + 1) % self.slots.len();
        }

        /// The unchecked read the whole type exists for.
        pub fn current(&self) -> u32 {
            // SAFETY: `head` is always < slots.len(), maintained by `new` and
            // `advance`, the only two places that write it.
            unsafe { *self.slots.as_ptr().add(self.head) }
        }

        /// The door. A SAFE function, in the same module, that can break the
        /// invariant the unsafe block above depends on.
        pub fn set_head(&mut self, head: usize) {
            self.head = head;
        }

        /// What it should have been.
        pub fn try_set_head(&mut self, head: usize) -> bool {
            if head < self.slots.len() {
                self.head = head;
                true
            } else {
                false
            }
        }
    }
}

use ring::Ring;

fn split_at_mut(v: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    let len = v.len();
    assert!(mid <= len, "mid must be within the slice");
    let ptr = v.as_mut_ptr();
    // SAFETY: mid <= len, checked directly above, so the two ranges are
    // disjoint and both lie inside the one allocation we hold a &mut to.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(f);
    std::panic::set_hook(hook);
    match r {
        Ok(()) => "(no panic)".into(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "(non-string panic)".into()),
    }
}

fn main() {
    println!("1. Your split_at_mut, checked against std's");
    let mut a = [5u32, 3, 0, 4, 2, 1];
    let mut b = a;
    let (l1, r1) = split_at_mut(&mut a, 3);
    let (l2, r2) = b.split_at_mut(3);
    println!("   mine: {l1:?} | {r1:?}");
    println!("   std : {l2:?} | {r2:?}");
    println!("   agree: {}", l1 == l2 && r1 == r2);

    println!();
    println!("2. The safe line the unsafe block depends on");
    println!("   split_at_mut(&mut a, 99) -> {}", caught(|| {
        let mut v = [1u32, 2, 3];
        let _ = split_at_mut(&mut v, 99);
    }));
    println!("   The assert turned a bad argument into a panic. DELETE it and the");
    println!("   same unsafe block — unchanged, still with its SAFETY comment —");
    println!("   builds a slice out of memory the allocation does not own. That is");
    println!("   undefined behaviour, not a panic and not a wrong number: the");
    println!("   Reference lists producing an invalid value as UB, and there is no");
    println!("   defined outcome to demonstrate.");

    println!();
    println!("3. The same shape, one module wide");
    let mut r = Ring::new([10, 20, 30, 40]);
    println!("   current() = {}", r.current());
    r.advance();
    println!("   after advance(), current() = {}", r.current());
    println!("   `current` reads without a bounds check, and is SAFE to call,");
    println!("   because `head < 4` is maintained by `new` and `advance`.");

    println!();
    println!("4. And the door in the same module");
    r.set_head(0);   // 0 happens to be in range; 99 would compile identically
    println!("   `pub fn set_head(&mut self, head: usize)` is a safe function that");
    println!("   assigns head directly. set_head(0) is called above and is fine —");
    println!("   `current()` = {} — and `r.set_head(99)` compiles exactly the same,", r.current());
    println!("   needs no unsafe at the call site, and makes the NEXT `current()`");
    println!("   read out of bounds. The unsafe block is unchanged and now unsound,");
    println!("   and the difference between the two calls is one literal.");
    println!("   So `set_head` is wrong in one of two ways, and both fixes are one");
    println!("   line:");
    println!("     - make it private, or");
    println!("     - make it check: try_set_head(9) = {}, try_set_head(2) = {}",
             r.try_set_head(9), r.try_set_head(2));
    println!("   current() after try_set_head(2) = {}", r.current());
    println!("   Marking it `unsafe fn` is the third option, and the honest one");
    println!("   when the caller genuinely can know more than the type does.");

    println!();
    println!("5. What this makes the unit of review");
    println!("   Not the `unsafe` block: every line that can reach the same");
    println!("   private state. `slots` and `head` are private, so the audit is");
    println!("   the MODULE — and that is the argument for keeping a module that");
    println!("   contains `unsafe` small enough to read in one sitting.");
    println!("   The `// SAFETY:` comment is the other half: it names the");
    println!("   invariant, so the person deleting an assert three lines away has");
    println!("   something to notice.");
}
