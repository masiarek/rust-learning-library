//! The two traits behind rev() and len(), and the contracts they add.
//!
//!   rustc --edition 2024 double_ended_and_exact_size.rs -o /tmp/dees && /tmp/dees

/// A sequence with no collection behind it, so both traits have to be written.
struct Countdown {
    hi: u32,
    lo: u32,
}

impl Countdown {
    fn new(n: u32) -> Self {
        Countdown { hi: n, lo: 1 }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.lo > self.hi {
            return None;
        }
        let v = self.hi;
        self.hi -= 1;
        Some(v)
    }

    // Written, not defaulted: the default is (0, None), and `collect` believes it.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.hi + 1).saturating_sub(self.lo) as usize;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for Countdown {
    fn next_back(&mut self) -> Option<u32> {
        if self.lo > self.hi {
            return None;
        }
        let v = self.lo;
        self.lo += 1;
        Some(v)
    }
}

// Allowed only because size_hint is exact. The trait is the promise; len() reads it.
impl ExactSizeIterator for Countdown {}

fn main() {
    println!("1. `rev()` is not a method on Iterator's items — it needs the other end");
    let scores = [5, 3, 0, 4, 2, 1];
    println!("   forwards  {:?}", scores.iter().collect::<Vec<_>>());
    println!("   .rev()    {:?}", scores.iter().rev().collect::<Vec<_>>());
    println!("   A slice iterator knows where it ends, so it can walk backwards.");
    println!("   An iterator that only has `next` cannot, and `.rev()` on one is");
    println!("   E0277: the trait bound `Countdown: DoubleEndedIterator` is not");
    println!("   satisfied — a compile error, never a run-time surprise.");

    println!();
    println!("2. Writing `next_back` is what buys it");
    println!("   Countdown::new(5)          {:?}", Countdown::new(5).collect::<Vec<_>>());
    println!("   .rev()                     {:?}", Countdown::new(5).rev().collect::<Vec<_>>());
    println!("   .rfind(|n| n % 2 == 0)     {:?}", Countdown::new(5).rfind(|n| n % 2 == 0));

    println!();
    println!("3. The contract: both ends eat from ONE sequence, and meet in the middle");
    let mut c = Countdown::new(6);
    let a = c.next();
    let b = c.next_back();
    let d = c.next();
    let e = c.next_back();
    println!("   next {a:?}  next_back {b:?}  next {d:?}  next_back {e:?}");
    println!("   remaining: {:?}", c.collect::<Vec<_>>());
    println!("   Six items, four taken from alternating ends, two left. An impl");
    println!("   where the two ends can hand out the same element twice is a bug");
    println!("   the compiler cannot catch — this is the part you must get right.");

    println!();
    println!("4. ExactSizeIterator: len() without walking anything");
    let c = Countdown::new(9);
    println!("   size_hint()  {:?}", c.size_hint());
    println!("   len()        {}", c.len());
    println!("   `len` is not a count — it reads `size_hint` and trusts it. That");
    println!("   is why implementing the trait is a promise rather than a method:");
    println!("   the impl block is empty, and what you are signing for is that");
    println!("   size_hint's two numbers are equal and correct.");
    let collected: Vec<u32> = Countdown::new(9).collect();
    println!("   and an exact hint pays: collect capacity {}", collected.capacity());

    println!();
    println!("5. Which adapters keep which trait");
    let v = [1u8, 2, 3, 4];
    println!("   map      rev {:?}  len {}", v.iter().map(|n| n * 2).rev().collect::<Vec<_>>(),
             v.iter().map(|n| n * 2).len());
    println!("   filter   rev {:?}  len: no — filter cannot know how many pass",
             v.iter().filter(|n| **n % 2 == 0).rev().collect::<Vec<_>>());
    println!("   skip     rev {:?}  len {}", v.iter().skip(1).rev().collect::<Vec<_>>(),
             v.iter().skip(1).len());
    println!("   chars    rev {:?}  len: no — a char is 1 to 4 bytes, so the count",
             "abc".chars().rev().collect::<String>());
    println!("            is not known without walking the string");
    println!("   Both are structural: an adapter keeps a trait only if it can");
    println!("   still honour the contract. `filter` keeps rev and loses len.");

    println!();
    println!("6. The trap: rev() and enumerate() do not commute");
    let names = ["Ada", "Ben", "Cara"];
    println!("   .enumerate().rev()  {:?}", names.iter().enumerate().rev().collect::<Vec<_>>());
    println!("   .rev().enumerate()  {:?}", names.iter().rev().enumerate().collect::<Vec<_>>());
    println!("   Same three names, same order on screen, DIFFERENT numbers.");
    println!("   `enumerate` counts the position it hands out, so putting it first");
    println!("   preserves the original index and putting it last renumbers from");
    println!("   the new front. If the index means \"row in the file\", it goes first.");
}
