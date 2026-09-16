//! Step 3 of the `ToOwned` path: the owned type and the borrowed type are two
//! different types, and the borrowed half is the type behind the `&`.
//!
//!   rustc --edition 2024 owned_and_borrowed_types.rs -o /tmp/oabt && /tmp/oabt

use std::any::{type_name, type_name_of_val};
use std::ops::Deref;
use std::path::{Path, PathBuf};

fn main() {
    println!("Checkpoint. For s: String and v: Vec<i32>, what types are *s and *v?");
    let s = String::from("Ada");
    let v = vec![1_i32, 2, 3];
    println!("   *s : {}", type_name_of_val(&*s));
    println!("   *v : {}", type_name_of_val(&*v));

    println!();
    println!("Each owned type names its borrowed half as Deref::Target");
    println!("   String   -> {}", type_name::<<String as Deref>::Target>());
    println!("   Vec<i32> -> {}", type_name::<<Vec<i32> as Deref>::Target>());
    println!("   PathBuf  -> {}", type_name::<<PathBuf as Deref>::Target>());

    println!();
    println!("Lending the borrowed half copies nothing: same bytes, shorter handle");
    let p = PathBuf::from("notes.txt");
    let (bs, bv, bp): (&str, &[i32], &Path) = (&s, &v, &p);
    println!("   &s as &str   = {bs:?}, same bytes as s: {}", bs.as_ptr() == s.as_ptr());
    println!("   &v as &[i32] = {bv:?}, same bytes as v: {}", bv.as_ptr() == v.as_ptr());
    println!("   &p as &Path  = {bp:?}");

    println!();
    println!("Going the other way has to build a new owner");
    let copy: String = bs.to_owned();
    println!("   bs.to_owned() = {copy:?}, same bytes as s: {}", copy.as_ptr() == s.as_ptr());
}
