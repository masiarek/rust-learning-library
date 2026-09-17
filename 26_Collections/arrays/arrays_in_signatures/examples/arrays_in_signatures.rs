//! An array in and out of a function: by value, by promoted `'static` slice,
//! and as a borrow of the caller's array.
//!
//!   rustc --edition 2024 arrays_in_signatures.rs -o /tmp/ais && /tmp/ais

use std::any::type_name_of_val;

/// Returns three `i32`s: twelve bytes travel back, no pointer.
fn create_array() -> [i32; 3] {
    [1, 2, 3]
}

/// Returns a borrow that outlives the call, because `[1, 2, 3]` is a constant
/// expression and the compiler promoted it to a static.
fn create_slice() -> &'static [i32] {
    &[1, 2, 3]
}

/// Returns part of the caller's array; the elided lifetime ties it to `arr`.
fn create_mut_slice(arr: &mut [i32]) -> &mut [i32] {
    &mut arr[1..3]
}

/// Takes exactly three.
fn print_array(arr: [i32; 3]) -> String {
    format!("{arr:?}")
}

/// Takes any length.
fn print_slice(arr: &[i32]) -> String {
    format!("{} elements: {arr:?}", arr.len())
}

/// One function for every length, and the length of the answer tied to it.
fn doubled<const N: usize>(xs: [i32; N]) -> [i32; N] {
    xs.map(|x| x * 2)
}

struct Readings {
    data: [i32; 3],
}

impl Readings {
    /// `[i32; 3]` is `Copy`, so this hands back a copy, not the field.
    fn get_array(&self) -> [i32; 3] {
        self.data
    }
}

fn main() {
    println!("1. Returning an array returns its elements");
    let a = create_array();
    println!("   create_array() = {a:?}, a {}", type_name_of_val(&a));

    println!();
    println!("2. Returning &'static [i32]: a promoted constant");
    let s = create_slice();
    println!("   create_slice() = {s:?}, a {}", type_name_of_val(&s));
    let promoted: &'static [i32] = &[1, 2, 3];
    println!("   let promoted: &'static [i32] = &[1, 2, 3];  -> {promoted:?}");
    println!("   With a run-time element, &[x, 1, 2] is a temporary: E0515.");

    println!();
    println!("3. Returning part of the caller's array");
    let mut arr = [1, 2, 3, 4];
    let slice = create_mut_slice(&mut arr);
    println!("   slice = {slice:?}, len {}", slice.len());
    slice[0] = 10;
    println!("   slice[0] = 10  ->  arr = {arr:?}");
    println!("   slice index 0 is array index 1: the view starts where the range did");

    println!();
    println!("4. A getter on a Copy array returns a copy");
    let r = Readings { data: [7, 8, 9] };
    let mut copy = r.get_array();
    copy[0] = 0;
    println!("   copy = {copy:?}, r.data = {:?}  (the field did not change)", r.data);
    println!("   With data: [String; 3] the same body is E0507: cannot move out");

    println!();
    println!("5. Parameters: exactly N, or any length");
    println!("   print_array([1, 2, 3])     -> {}", print_array([1, 2, 3]));
    println!("   print_array([1, 2, 3, 4])  -> E0308, expected an array with a size of 3");
    println!("   print_slice(&[1, 2, 3, 4]) -> {}", print_slice(&[1, 2, 3, 4]));
    println!("   print_slice(&arr[..2])     -> {}", print_slice(&arr[..2]));

    println!();
    println!("6. Const generics: every length, and the same length back");
    let two = doubled([1, 2]);
    let five = doubled([1, 2, 3, 4, 5]);
    println!("   doubled([1, 2])          = {two:?}, a {}", type_name_of_val(&two));
    println!("   doubled([1, 2, 3, 4, 5]) = {five:?}, a {}", type_name_of_val(&five));
}
