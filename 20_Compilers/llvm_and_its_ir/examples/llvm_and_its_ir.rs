//! The function whose LLVM IR and control-flow graph the lesson shows.
//!
//! Three branches and one exit: small enough to read as IR, big enough to have
//! a graph worth drawing.

#[unsafe(no_mangle)]
pub extern "C" fn classify(n: i32) -> i32 {
    if n < 0 {
        -1
    } else if n == 0 {
        0
    } else {
        1
    }
}

fn main() {
    for n in [-5, 0, 7] {
        println!("classify({n:>2}) = {:>2}", classify(n));
    }
}
