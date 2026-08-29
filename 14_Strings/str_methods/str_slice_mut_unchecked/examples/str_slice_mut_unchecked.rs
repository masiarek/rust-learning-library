#![allow(deprecated)]

fn main() {
    let mut owned = String::from("hello world");
    unsafe { owned.as_mut_str().slice_mut_unchecked(0, 5) }.make_ascii_uppercase();
    println!("{owned:?}");

    let mut same = String::from("hello world");
    unsafe { same.as_mut_str().get_unchecked_mut(0..5) }.make_ascii_uppercase();
    println!("{same:?}");
    println!("identical: {}", owned == same);
}
