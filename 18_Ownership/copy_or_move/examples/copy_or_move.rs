//! Copy or move? One program, with the value swapped. `let b = a;` copies
//! `a`'s own bytes every time; the type decides whether `a` is still usable
//! afterwards. The verdict column is asked of the compiler, not typed in.
//!
//!   rustc --edition 2024 copy_or_move.rs -o /tmp/com && /tmp/com

use std::marker::PhantomData;

/// Asks the compiler whether `T` is `Copy`. Method lookup tries the inherent
/// `verdict` first; where `T: Copy` does not hold, that method does not apply
/// and the trait's default answers instead.
struct Probe<T>(PhantomData<T>);

trait Moves {
    fn verdict(&self) -> &'static str {
        "move"
    }
}

impl<T> Moves for Probe<T> {}

impl<T: Copy> Probe<T> {
    fn verdict(&self) -> &'static str {
        "copy"
    }
}

fn probe<T>(_: &T) -> Probe<T> {
    Probe(PhantomData)
}

/// One row: bind the value at the type written beside it, so the compiler
/// checks the type column as well, then ask the probe about that type.
macro_rules! row {
    ($value:expr => $ty:ty) => {{
        let a: $ty = $value;
        println!("  {:<26} {:<16} {}", stringify!($value), stringify!($ty), probe(&a).verdict());
    }};
}

/// Inside a generic function nothing says `T: Copy`, so the probe cannot see it.
fn verdict_inside_a_generic<T>(x: T) -> &'static str {
    probe(&x).verdict()
}

fn rule(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    rule("The program that was expected to fail");
    let a = "hi";
    let b = a;
    println!("  a = {a:?}, b = {b:?}   both still usable");
    println!("  \"hi\" is a &str: a string, but not a String.");

    rule("Swap the value, keep the program");
    println!("  {:<26} {:<16} {}", "let a = …;", "type", "let b = a;");
    let owned = String::from("hi");
    let mut n = 5;
    row!("hi" => &str);
    row!(5 => i32);
    row!(3.5 => f64);
    row!('x' => char);
    row!(true => bool);
    row!((1, "hi") => (i32, &str));
    row!([1, 2, 3] => [i32; 3]);
    row!(&owned => &String);
    row!(String::from("hi") => String);
    row!(vec![1, 2] => Vec<i32>);
    row!(Box::new(5) => Box<i32>);
    row!(&mut n => &mut i32);
    row!((1, String::from("hi")) => (i32, String));

    rule("A move is refused at the next use, not at the let");
    let a = String::from("hi");
    let b = a; // compiles: the move itself is legal
    // println!("{a:?}");  // error[E0382]: borrow of moved value: `a`
    println!("  b = {b:?}   only b is left");

    rule("Two ways to keep both names when the type moves");
    let a = String::from("hi");
    let b = &a; // a borrow: no second owner
    println!("  let b = &a;          a = {a:?}, b = {b:?}");
    let c = a.clone(); // a second String, with its own buffer
    println!("  let c = a.clone();   a = {a:?}, c = {c:?}");

    rule("The probe cannot see through a generic");
    let five: i32 = 5;
    println!("  i32 at the call site:        {}", probe(&five).verdict());
    println!("  i32 inside fn f<T>(x: T):    {}", verdict_inside_a_generic(five));
    println!("  Inside f nothing says T: Copy, so the fallback answers.");
}
