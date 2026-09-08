//! Kata solution: the bug that does not crash.
//!
//!   rustc --edition 2024 iterator_invalidation_kata.rs -o /tmp/iik && /tmp/iik

fn main() {
    println!("THE C++ SHAPE");
    println!("  for (auto it = v.begin(); it != v.end(); ++it)");
    println!("      if (*it % 2 == 0) v.erase(it);");
    println!("  erase invalidates `it`, and ++it then advances a dangling");
    println!("  iterator. With a vector this usually does not crash -- the");
    println!("  memory is still mapped -- it just SKIPS the element after each");
    println!("  removal and returns a wrong answer. No sanitizer objects to a");
    println!("  read that is in bounds.");
    println!();

    println!("THE SAME LOOP IN RUST");
    println!("  for x in &v {{ if *x % 2 == 0 {{ v.retain(...) }} }}");
    println!("  E0502: cannot borrow `v` as mutable because it is also borrowed");
    println!("  as immutable. `&v` in the for loop is a shared borrow held for");
    println!("  the whole loop body, and mutation needs an exclusive one.");
    println!();
    println!("  The rule doing the work is the ordinary one -- many readers or");
    println!("  one writer -- applied to a loop. There is no iterator-specific");
    println!("  machinery, and no runtime modification counter of the kind");
    println!("  Java's ConcurrentModificationException needs.");
    println!();

    println!("WHAT TO WRITE INSTEAD");
    let mut v: Vec<u32> = (1..=10).collect();
    println!("  start           {v:?}");
    v.retain(|x| x % 2 != 0);
    println!("  retain(odd)     {v:?}     <- one pass, the library owns the loop");

    let source: Vec<u32> = (1..=10).collect();
    let kept: Vec<u32> = source.iter().copied().filter(|x| x % 2 != 0).collect();
    println!("  filter+collect  {kept:?}     <- a new Vec, the old one untouched");

    let mut idx: Vec<u32> = (1..=10).collect();
    let mut i = 0;
    while i < idx.len() {
        if idx[i] % 2 == 0 { idx.remove(i); } else { i += 1; }
    }
    println!("  index loop      {idx:?}     <- correct, and O(n^2); the bug the");
    println!("                                C++ version had is impossible here");
    println!("                                because there is no iterator to");
    println!("                                invalidate -- only an index you");
    println!("                                are responsible for");
    println!();

    println!("WHY THIS ONE IS THE MOST INSTRUCTIVE ON THE LIST");
    println!("  Every other bug in this chapter has a chance of announcing");
    println!("  itself: a segfault, a corrupted allocator, a sanitizer report.");
    println!("  This one returns a plausible answer. It is the case where");
    println!("  'undefined behaviour' costs you a wrong number in a report");
    println!("  rather than a crash you can debug -- and the compile error is");
    println!("  the only thing that would ever have told you.");

    assert_eq!(v, vec![1, 3, 5, 7, 9]);
    assert_eq!(kept, v);
    assert_eq!(idx, v);
}
