// Clang's -Wlifetime-safety and rustc's borrow checker apply one rule: a
// pointer that may still be used must not hold anything already destroyed.
// Each function below is a shape both checkers accept, and says why.

#[allow(unused_assignments)] // p's first value is never read -- which is the point
fn liveness() -> i32 {
    let mut p: &i32;
    {
        let x = Box::new(5);
        p = &x;
    } // x is freed here, while p still holds its address...
    let y = Box::new(42);
    p = &y; // ...but p is overwritten before anything reads it
    *p
}

fn either(cond: bool) -> i32 {
    let i = 1;
    let x = Box::new(5); // declared outside the `if`, so it outlives p
    let mut p = &i;
    if cond {
        p = &x;
    } // p may point at i or at x -- both still alive
    *p
}

// The signature says the result borrows from `s`: the contract C++ writes
// as [[clang::lifetimebound]] on the parameter. Here it is not optional.
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

fn main() {
    println!("liveness()        -> {}", liveness());        // 42
    println!("either(false)     -> {}", either(false));     // 1
    println!("either(true)      -> {}", either(true));      // 5

    let text = String::from("hello world");
    println!("first_word(&text) -> {}", first_word(&text)); // hello
}
