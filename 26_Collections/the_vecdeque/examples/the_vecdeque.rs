//! `VecDeque`: a `Vec` with a fourth number, so both ends are cheap.
//!
//!   rustc --edition 2024 the_vecdeque.rs -o /tmp/vd && /tmp/vd

use std::collections::VecDeque;

fn main() {
    println!("1. The default usage: push_back to add, pop_front to remove");
    let mut jobs: VecDeque<&str> = VecDeque::from(["compile", "link", "test"]);
    jobs.push_back("package");
    while let Some(job) = jobs.pop_front() {
        println!("   run {job:<8} still queued: {jobs:?}");
    }
    println!("   Debug prints front to back, exactly like a Vec, and so does iter().");
    println!("   Nothing on the outside of the type shows that it is a ring.");

    println!();
    println!("2. Four numbers on the stack, not three");
    let word = size_of::<usize>();
    println!("   Vec<u32>            {} words   ptr, len, capacity", size_of::<Vec<u32>>() / word);
    println!("   VecDeque<u32>       {} words   head, len, ptr, capacity",
             size_of::<VecDeque<u32>>() / word);
    println!("   VecDeque<[u8; 999]> {} words   same four — the elements are never in it",
             size_of::<VecDeque<[u8; 999]>>() / word);
    println!("   `head` is the physical slot the front element sits in. That one extra");
    println!("   number is the whole difference, and it buys an O(1) pop_front.");

    println!();
    println!("3. The elements are allowed to wrap, and as_slices shows it");
    let mut ring: VecDeque<char> = VecDeque::with_capacity(4);
    for c in ['a', 'b', 'c'] {
        ring.push_back(c);
    }
    println!("   push_back a, b, c        {ring:?}  as_slices {:?}", ring.as_slices());
    ring.pop_front();
    ring.push_back('d');
    ring.push_back('e');
    println!("   pop_front, push_back d,e {ring:?}  as_slices {:?}", ring.as_slices());
    println!("   capacity is still {}, and nothing was copied or reallocated.", ring.capacity());
    println!();
    print_layout(&ring);
    println!("   The contents are in TWO pieces, and as_slices hands you both of them:");
    println!("   ({:?}, {:?}). Reading only the first is the bug this page is about.",
             ring.as_slices().0, ring.as_slices().1);
    println!();
    ring.pop_front();
    println!("   One more pop_front       {ring:?}       as_slices {:?}", ring.as_slices());
    println!();
    print_layout(&ring);
    println!("   Slot 1 is free now, and the contents still wrap around it: the");
    println!("   elements can be non-contiguous AND have a gap in the middle. Neither");
    println!("   the Debug output nor any index you write ever shows that.");

    println!();
    println!("4. Indexing is logical, never physical");
    println!("   ring[0] = {:?}  <- the front, which physically sits in slot 2", ring[0]);
    println!("   ring[2] = {:?}  <- the back, which physically sits in slot 0", ring[2]);
    println!("   front {:?}  back {:?}  get(9) {:?}", ring.front(), ring.back(), ring.get(9));
    println!("   contains(&'e') = {}, and iter() walks c, d, e in that order.",
             ring.contains(&'e'));

    println!();
    println!("5. make_contiguous rotates the ring and hands back a real slice");
    let flat: &mut [char] = ring.make_contiguous();
    println!("   make_contiguous() -> {flat:?}   the ring rotated, one piece now");
    flat.sort_by(|a, b| b.cmp(a));
    println!("   ...and slice methods apply to it: sort_by descending -> {ring:?}");
    println!("   as_slices is now ({:?}, {:?}) — the second piece is empty.",
             ring.as_slices().0, ring.as_slices().1);
    println!("   It costs O(n) and needs &mut, so it is not a free `as_slice()`.");

    println!();
    println!("6. Growth is the same amortised doubling a Vec uses");
    let mut grow: VecDeque<u32> = VecDeque::new();
    print!("   cap {}", grow.capacity());
    for n in 1..=9 {
        grow.push_back(n);
        print!(" -> {}", grow.capacity());
    }
    println!();
    println!("   with_capacity(9).capacity() = {}  <- exact, not rounded up",
             VecDeque::<u32>::with_capacity(9).capacity());
    println!("   Before Rust 1.67 that answer was 15: the buffer was rounded up to a");
    println!("   power of two (16) and one slot was always left empty, so that head ==");
    println!("   tail could mean \"empty\". Material written before 2023 describes that");
    println!("   layout, and it is gone.");

    println!();
    println!("7. Vec and VecDeque convert into each other");
    let v = vec![1, 2, 3, 4];
    let d = VecDeque::from(v);
    println!("   Vec -> VecDeque   {d:?}   O(1), guaranteed, no reallocation");
    let back = Vec::from(d);
    println!("   VecDeque -> Vec   {back:?}   never reallocates, but rotates first if wrapped");
    let mut wrapped: VecDeque<char> = VecDeque::with_capacity(4);
    for c in ['a', 'b', 'c'] {
        wrapped.push_back(c);
    }
    wrapped.pop_front();
    wrapped.push_back('d');
    wrapped.push_back('e');
    println!("   a wrapped one     {wrapped:?} in slots {:?}", wrapped.as_slices());
    println!("                     -> {:?}   the O(n) rotation, done for you",
             Vec::from(wrapped));
    println!("   A VecDeque compares equal to a Vec and to an array of the same");
    println!("   contents, so a test never has to convert one to assert on it:");
    let d2: VecDeque<i32> = VecDeque::from([1, 2, 3]);
    println!("   d == vec![1, 2, 3] is {}, d == [1, 2, 3] is {}",
             d2 == vec![1, 2, 3], d2 == [1, 2, 3]);
}

/// Draw where the elements physically sit, computed from `as_slices` rather than
/// asserted: the second slice always starts at slot 0, and the first one ends at
/// the end of the buffer — so `head` is `capacity - front.len()` whenever the
/// contents wrap.
fn print_layout(ring: &VecDeque<char>) {
    let (front, back) = ring.as_slices();
    let head = ring.capacity() - front.len();
    let mut slots = String::from("      physical slot ");
    let mut cells = String::from("                    ");
    for slot in 0..ring.capacity() {
        let c = if slot < back.len() {
            Some(back[slot])
        } else if slot >= head {
            front.get(slot - head).copied()
        } else {
            None
        };
        slots.push_str(&format!("{slot:^7}"));
        cells.push_str(&format!("{:^7}", c.map(|c| format!("'{c}'")).unwrap_or_default()));
    }
    println!("{}", slots.trim_end());
    println!("{}", cells.trim_end());
    println!("{:>width$}^ head = {head}, len = {}", "", ring.len(),
             width = 21 + head * 7 + 3);
    let logical: String = ring.iter().map(|c| format!("{c:^7}")).collect();
    println!("      logical order  {}", logical.trim_end());
    println!();
}
