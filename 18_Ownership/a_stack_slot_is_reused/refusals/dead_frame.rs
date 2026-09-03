// The C bug, written in Rust. It does not compile, which is why it is here and
// not in examples/ -- `run_examples.py` compiles and runs everything it finds
// there, so a file that must FAIL to compile cannot live beside the ones that
// must succeed.
//
// The transcript on the lesson page came from this file, under this name:
//
//   rustc --edition 2024 dead_frame.rs
//   error[E0106]: missing lifetime specifier
//    --> dead_frame.rs:4:27

struct Ballot { precinct: u32 }

fn cast(precinct: u32) -> &Ballot {
    let b = Ballot { precinct };
    &b
}

fn main() {
    let kept = cast(7);
    println!("{}", kept.precinct);
}
