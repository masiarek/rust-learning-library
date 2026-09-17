//! Kata solution: reference to a local variable.
//!
//! The refused program, from ch. 5 of Programming Rust, 2nd ed.:
//!
//!     let r;
//!     {
//!         let x = 1;
//!         r = &x;              // loan on `x` starts
//!     }                        // `x` dropped: a use of the place, condition (2)
//!     assert_eq!(*r, 1);       // a use of `r`, so condition (1) stretches the loan here
//!
//! E0597. Condition (1) makes the loan last until the assert; the drop at `}`
//! falls inside it, and condition (2) forbids that. Two ways out: move the drop
//! past the last use, or end the loan before the drop.
//!
//!   rustc --edition 2024 a_borrow_is_a_loan_kata.rs -o /tmp/a_borrow_is_a_loan_kata && /tmp/a_borrow_is_a_loan_kata

/// Fix 1: declare `x` outside the block. It now drops at the end of this
/// function, after the last use of `r`, so no use of `x` falls inside the loan.
fn move_x_out() -> i32 {
    let x = 1;
    let r;
    {
        r = &x;
    }
    assert_eq!(*r, 1);
    *r
}

/// Fix 2: keep `x` in the block, but keep an `i32` instead of a `&i32`.
/// The reference is last used inside the block, so its loan has ended by the `}`.
fn copy_the_value_out() -> i32 {
    let n;
    {
        let x = 1;
        let r = &x;
        n = *r; // i32 is Copy: the value leaves the block, the loan does not
    }
    assert_eq!(n, 1);
    n
}

fn main() {
    println!("fix 1, `x` declared outside the block: *r = {}", move_x_out());
    println!("fix 2, value copied out of the block:  n  = {}", copy_the_value_out());
}
