//! Kata solution: five references where you meant five numbers.
//!
//!   rustc --edition 2024 double_ended_and_exact_size_kata.rs -o /tmp/deesk && /tmp/deesk
//!
//! Three refusals belong to this kata and cannot live in a file that compiles.
//! Each was produced by a scratch file named the way its transcript says.
//!
//! 1. Drop the annotation and `collect` has nothing to aim at (`no_annotation.rs`):
//!
//!        let reversed_vec = my_array.iter().rev().collect();
//!
//!    error[E0283]: type annotations needed
//!      = note: the type must implement `FromIterator<&i32>`
//!      help: consider giving `reversed_vec` an explicit type
//!        4 |     let reversed_vec: Vec<_> = my_array.iter().rev().collect();
//!
//!    Note what the suggestion would give you: `Vec<_>` infers `Vec<&i32>`,
//!    so following the compiler's advice lands on the reference vec too.
//!
//! 2. Summing it the obvious way is one `&` too many (`summing_refs.rs`):
//!
//!        let total: i32 = reversed_vec.iter().sum();
//!
//!    error[E0277]: a value of type `i32` cannot be made by summing an
//!                  iterator over elements of type `&&i32`
//!      = help: the trait `Sum<&&i32>` is not implemented for `i32`
//!
//! 3. `rev()` on a set is a compile error, not an ordering (`reversing_a_set.rs`):
//!
//!        let backwards: Vec<&i32> = seen.iter().rev().collect();
//!
//!    error[E0277]: the trait bound `std::collections::hash_set::Iter<'_, i32>:
//!                  DoubleEndedIterator` is not satisfied

fn main() {
    let my_array = [1, 2, 3, 4, 5];

    // 1 -----------------------------------------------------------------
    println!("1. The program as written prints the same thing twice");
    print!("   for num in my_array.iter().rev()   ");
    for num in my_array.iter().rev() {
        print!("{num} ");
    }
    println!();

    let reversed_vec: Vec<&i32> = my_array.iter().rev().collect();
    let reversed_val: Vec<i32> = my_array.iter().rev().copied().collect();
    println!("   Vec<&i32>                         {reversed_vec:?}");
    println!("   Vec<i32>                          {reversed_val:?}");
    println!("   Debug prints a &i32 by printing what it points at, so the");
    println!("   two lines are identical and the type is invisible on screen.");
    println!();

    // 2 -----------------------------------------------------------------
    println!("2. Three ways to the sum, once the items are references");
    let by_copied: i32 = reversed_vec.iter().copied().sum();
    let by_deref: i32 = reversed_vec.iter().map(|n| **n).sum();
    let by_into: i32 = reversed_vec.clone().into_iter().sum();
    println!("   .iter().copied().sum()            {by_copied}   &&i32 -> &i32");
    println!("   .iter().map(|n| **n).sum()        {by_deref}   &&i32 -> i32");
    println!("   .into_iter().sum()                {by_into}   items are &i32, vec consumed");
    println!("   `.iter().sum::<i32>()` is the one that does not compile: `iter`");
    println!("   on a Vec<&i32> hands out &&i32, and i32 sums &i32 but not &&i32.");
    println!();

    // 3 -----------------------------------------------------------------
    println!("3. Three routes to a Vec<i32>, and what the reference vec cost");
    let copied: Vec<i32> = my_array.iter().rev().copied().collect();
    let cloned: Vec<i32> = my_array.iter().rev().cloned().collect();
    let owned: Vec<i32> = my_array.into_iter().rev().collect();
    println!("   .iter().rev().copied()            {copied:?}   Copy items, free");
    println!("   .iter().rev().cloned()            {cloned:?}   same here, the general one");
    println!("   .into_iter().rev()                {owned:?}   arrays yield values (2021+)");
    println!(
        "   a &i32 is a pointer, and a pointer is usize-wide: {}",
        size_of::<&i32>() == size_of::<usize>()
    );
    println!("   so Vec<&i32> stores one pointer per number where Vec<i32>");
    println!("   stores the number — wider on any 64-bit target, and one");
    println!("   indirection on every read. The borrow is not the cheap option.");
    println!();

    // 4 -----------------------------------------------------------------
    println!("4. rev() does not commute with skip or take either");
    let a: Vec<i32> = my_array.iter().copied().skip(1).rev().collect();
    let b: Vec<i32> = my_array.iter().copied().rev().skip(1).collect();
    let c: Vec<i32> = my_array.iter().copied().take(2).rev().collect();
    let d: Vec<i32> = my_array.iter().copied().rev().take(2).collect();
    println!("   .skip(1).rev()                    {a:?}   dropped 1, then reversed");
    println!("   .rev().skip(1)                    {b:?}   reversed, then dropped 5");
    println!("   .take(2).rev()                    {c:?}   kept the first two");
    println!("   .rev().take(2)                    {d:?}   kept the last two");
    println!("   All four compile and none is wrong. \"The last three, newest");
    println!("   first\" is `.rev().take(3)`; `.take(3).rev()` is the first three");
    println!("   read backwards, which is a different set of rows.");
    println!();

    // 5 -----------------------------------------------------------------
    println!("5. Why a HashSet refuses rev() rather than picking an order");
    println!("   `seen.iter().rev()` is E0277: hash_set::Iter is not");
    println!("   DoubleEndedIterator. A hash set has no last element to walk");
    println!("   back from — its iteration order is unspecified — so there is");
    println!("   nothing for next_back to return. The refusal is the honest");
    println!("   answer, and it arrives at compile time.");
}
