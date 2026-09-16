//! Kata solution for step 1: one generic function that hands back the owned
//! twin of whatever it is lent — and the argument's type decides which twin.
//!
//!   rustc --edition 2024 clone_vs_to_owned_kata.rs -o /tmp/cvtok && /tmp/cvtok

use std::any::type_name_of_val;
use std::ffi::OsStr;
use std::path::Path;

/// `?Sized` is what lets `B` be `str`, `[i32]`, `Path` or `OsStr`.
fn owner<B: ToOwned + ?Sized>(borrowed: &B) -> B::Owned {
    borrowed.to_owned()
}

fn main() {
    println!("One function, six arguments: the type of the argument picks B");
    let text: &str = "Ada";
    println!("   owner(text)                 B = str   -> {}", type_name_of_val(&owner(text)));
    println!("   owner(&text)                B = &str  -> {}", type_name_of_val(&owner(&text)));
    println!("   owner(&[1, 2][..])          B = [i32] -> {}", type_name_of_val(&owner(&[1, 2][..])));
    println!("   owner(Path::new(\"a.txt\"))   B = Path  -> {}", type_name_of_val(&owner(Path::new("a.txt"))));
    println!("   owner(OsStr::new(\"a.txt\"))  B = OsStr -> {}", type_name_of_val(&owner(OsStr::new("a.txt"))));
    println!("   owner(&7)                   B = i32   -> {}", type_name_of_val(&owner(&7)));

    println!();
    println!("The one that surprises: an extra & changes B, and so the answer");
    let same = owner(&text);
    println!("   owner(&text) is the same &str, same address: {}", std::ptr::eq(same, text));
}
