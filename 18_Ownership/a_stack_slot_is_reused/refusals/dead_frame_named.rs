// The same bug with the signature problem removed: a lifetime IS available
// here, because it comes from the argument. The function still cannot return a
// reference to its own local, and now the compiler says so by name.
//
//   rustc --edition 2024 dead_frame_named.rs
//   error[E0515]: cannot return reference to local variable `p`
//    --> dead_frame_named.rs:13:5

struct Point { x: u32 }

fn plot<'a>(source: &'a u32) -> &'a Point {
    let p = Point { x: *source };
    &p
}

fn main() {
    let n = 7;
    let kept = plot(&n);
    println!("{}", kept.x);
}
