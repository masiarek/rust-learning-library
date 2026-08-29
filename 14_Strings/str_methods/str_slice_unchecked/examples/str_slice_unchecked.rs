#![allow(deprecated)]

fn main() {
    let s = "héllo";

    let old = unsafe { s.slice_unchecked(0, 3) };
    let new = unsafe { s.get_unchecked(0..3) };
    println!("{old:?} {new:?}");
    println!("identical: {}", old == new);
}
