// The same bug with the signature problem removed: a lifetime IS available
// here, because it comes from the argument. The function still cannot return a
// reference to its own local, and now the compiler says so by name.
//
//   rustc --edition 2024 dead_frame_named.rs
//   error[E0515]: cannot return reference to local variable `b`
//    --> dead_frame_named.rs:7:5

struct Ballot { precinct: u32 }

fn cast<'a>(source: &'a u32) -> &'a Ballot {
    let b = Ballot { precinct: *source };
    &b
}

fn main() {
    let n = 7;
    let kept = cast(&n);
    println!("{}", kept.precinct);
}
