// Overflow is a defined event with a name you pick, not a licence for the
// optimizer to assume it never happens.

use std::hint::black_box;
use std::panic;

fn main() {
    let x = black_box(i32::MAX);                // hidden from const-folding

    println!("checked_add(1)     -> {:?}", x.checked_add(1));
    println!("wrapping_add(1)    -> {}", x.wrapping_add(1));
    println!("saturating_add(1)  -> {}", x.saturating_add(1));
    println!("overflowing_add(1) -> {:?}", x.overflowing_add(1));

    // Plain `x + 1` panics in this build and wraps in a release one, which is
    // the trap. Caught here only so the run finishes.
    panic::set_hook(Box::new(|_| {}));
    let plain = panic::catch_unwind(|| x + 1);
    let _ = panic::take_hook();

    let why = plain.unwrap_err();
    if let Some(message) = why.downcast_ref::<&str>() {
        println!("x + 1              -> panicked: {message}");
    }
}
