//! Step 1 of the `ToOwned` path: `Clone` hands back the type you started
//! with, `ToOwned` may hand back a different one — and the impls std writes by
//! hand sit on the type behind the `&`: `str`, not `&str`.
//!
//!   rustc --edition 2024 clone_vs_to_owned.rs -o /tmp/cvto && /tmp/cvto

use std::any::type_name;
use std::ffi::{CStr, OsStr};
use std::path::Path;

/// The owned type that borrowed type `B` names in its `ToOwned` impl.
fn owned_of<B: ToOwned + ?Sized>() -> &'static str {
    type_name::<B::Owned>()
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("Clone: the copy is the type you started with");
    let name = String::from("Ada");
    let votes = vec![3_i32, 1];
    println!("   Clone::clone(&name)   name: String   -> {}", type_of(&Clone::clone(&name)));
    println!("   Clone::clone(&votes)  votes: Vec<i32> -> {}", type_of(&Clone::clone(&votes)));

    println!();
    println!("Checkpoint. What does each .to_owned() turn into?");
    println!("   \"hi\".to_owned()                   -> {}", type_of(&"hi".to_owned()));
    println!("   [1_i32, 2][..].to_owned()         -> {}", type_of(&[1_i32, 2][..].to_owned()));
    println!("   Path::new(\"notes.txt\").to_owned() -> {}", type_of(&Path::new("notes.txt").to_owned()));

    println!();
    println!("Which type carries the impl: the one behind the &");
    println!("   <str as ToOwned>::Owned   = {}", owned_of::<str>());
    println!("   <[i32] as ToOwned>::Owned = {}", owned_of::<[i32]>());
    println!("   <Path as ToOwned>::Owned  = {}", owned_of::<Path>());
    println!("   <OsStr as ToOwned>::Owned = {}", owned_of::<OsStr>());
    println!("   <CStr as ToOwned>::Owned  = {}", owned_of::<CStr>());
}
