//! A borrow is a loan: creating a reference records a loan on the place it
//! borrows from, with the reference's lifetime, and the borrow checker checks
//! (1) the reference is used only while that lifetime lasts, and
//! (2) the place is not used against the loan until the loan expires.
//!
//! Every program below is one the checker accepts. The first three are from
//! section 2 of Jung, Dang, Kang and Dreyer, "Stacked Borrows" (POPL 2020); the
//! fourth is the fix to the local-variable example in ch. 5 of Programming
//! Rust, 2nd ed. Their refused twins are rustc transcripts on the page.
//!
//!   rustc --edition 2024 a_borrow_is_a_loan.rs -o /tmp/a_borrow_is_a_loan && /tmp/a_borrow_is_a_loan

static FOREVER: i32 = 1;

fn words<T: ?Sized>(r: &T) -> usize {
    size_of_val(r) / size_of::<usize>()
}

fn main() {
    println!("──── 1. push first, then the loan: nothing is outstanding at the push");
    let mut v = vec![10, 11];
    v.push(12); // Vec::push(&mut v, 12): a loan that ends with the call
    let vptr = &mut v[1]; // a new loan on `v`, lifetime 'a
    println!("  v[1] = {}", *vptr); // last use of vptr: 'a ends here
    println!("  v    = {v:?}   (the owner is free again)");
    println!("  Swap the push and the `let`: E0499, the push uses `v` during 'a.");

    println!();
    println!("──── 2. A reborrow: vptr's loan sits inside v2's");
    let mut v = vec![10, 11];
    let v2 = &mut v; // loan on `v`, lifetime 'a
    let vptr = &mut (*v2)[1]; // loan on `*v2`, lifetime 'b, inside 'a
    println!("  v[1] = {}", *vptr); // last use of vptr: 'b ends here
    Vec::push(v2, 12); // a use of `*v2`, after 'b: allowed
    println!("  v    = {v:?}");
    println!("  One more read of vptr after the push: E0499 on `*v2`.");

    println!();
    println!("──── 3. Two shared loans on one element, used interleaved");
    let v = vec![10, 11];
    let vptr = &v[1]; // shared loan, lifetime 'a
    let vptr2 = &v[1]; // a non-mutating use of v: no conflict, a second loan 'b
    println!("  v[1] = {}", *vptr);
    println!("  v.len() = {}   (a read of the owner under shared loans)", v.len());
    println!("  v[1] = {}", *vptr2);
    println!("  With `let mut v` and v.push(12) before the first read: E0502, a mutation during 'a.");

    println!();
    println!("──── 4. The referent outlives the loan: `x` declared outside the block");
    let x = 1;
    let r;
    {
        r = &x; // loan on `x`
    }
    assert_eq!(*r, 1); // last use of r
    println!("  *r = {}", *r);
    println!("  With `let x = 1;` inside the block: E0597, `x` dropped while borrowed.");

    println!();
    println!("──── Checkpoint. `r` declared outside, `x` inside, last read inside. Compiles?");
    let r;
    {
        let x = 1;
        r = &x; // loan on `x`
        println!("  yes: *r = {}   (read before the `}}`, so the loan ended first)", *r);
    } // `x` dropped: no loan outstanding
    println!("  Where `r` is declared does not matter. Its last read sets the loan's end.");

    println!();
    println!("──── 5. At run time a reference carries no lifetime");
    let short: &i32 = &x; // borrowed for a few lines
    let forever: &'static i32 = &FOREVER;
    let slice: &[i32] = &v;
    let text: &str = "loan";
    println!("  &i32 borrowed for a few lines   {} word", words(&short));
    println!("  &'static i32                    {} word", words(&forever));
    println!("  &[i32]  address + length        {} words", words(&slice));
    println!("  &str    address + length        {} words", words(&text));
    println!("  The length is run-time data; the lifetime was checked and erased.");
}
