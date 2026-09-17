//! Kata solution for step 6: own every borrow in a `&[&T]`. The obvious
//! closure lands on the blanket impl for `&T` — a hard error where the return
//! type asks for `T::Owned`, and a quiet `Vec<&T>` where nothing asks.
//!
//!   rustc --edition 2024 the_blanket_to_owned_kata.rs -o /tmp/tbtok && /tmp/tbtok

use std::any::type_name_of_val;
use std::path::Path;

// Written as `xs.iter().map(|x| x.to_owned()).collect()` this is E0277:
// `x` is a `&&T`, the dot matches the blanket impl for `&T` first, and a
// `Vec<T::Owned>` cannot be built from an iterator over `&T`.

/// Dereference to the rung you want: `*x` is a `&T`, so `Self = T`.
fn by_deref<T: ?Sized + ToOwned>(xs: &[&T]) -> Vec<T::Owned> {
    xs.iter().map(|x| (*x).to_owned()).collect()
}

/// Let the pattern do the dereference: `|&x|` binds `x: &T`.
fn by_pattern<T: ?Sized + ToOwned>(xs: &[&T]) -> Vec<T::Owned> {
    xs.iter().map(|&x| x.to_owned()).collect()
}

/// No dot, so no search: `T::to_owned` takes a `&T`, and `copied` yields one.
fn by_path<T: ?Sized + ToOwned>(xs: &[&T]) -> Vec<T::Owned> {
    xs.iter().copied().map(T::to_owned).collect()
}

fn main() {
    println!("1. Where nothing asks for T::Owned, the obvious closure is quiet");
    let words: [&str; 2] = ["ada", "grace"];
    let quiet: Vec<_> = words.iter().map(|x| x.to_owned()).collect();
    println!("   words.iter().map(|x| x.to_owned()) -> {}", type_name_of_val(&quiet));

    println!();
    println!("2. Three bodies that reach T's impl, four kinds of T");
    let slices: [&[i32]; 2] = [&[1, 2], &[3]];
    let paths = [Path::new("/a"), Path::new("/b/c")];
    let numbers = [&1, &2];
    println!("   T = str    by_deref   -> {:<16} {}", format!("{:?}", by_deref(&words)), type_name_of_val(&by_deref(&words)));
    println!("   T = [i32]  by_pattern -> {:<16} {}", format!("{:?}", by_pattern(&slices)), type_name_of_val(&by_pattern(&slices)));
    println!("   T = Path   by_path    -> {:<16} {}", format!("{:?}", by_path(&paths)), type_name_of_val(&by_path(&paths)));
    println!("   T = i32    by_deref   -> {:<16} {}", format!("{:?}", by_deref(&numbers)), type_name_of_val(&by_deref(&numbers)));

    println!();
    println!("3. Why i32 works too: the blanket impl gives every Clone type Owned = Self");
    println!("   <i32 as ToOwned>::Owned is i32, so Vec<T::Owned> is Vec<i32>");
}
