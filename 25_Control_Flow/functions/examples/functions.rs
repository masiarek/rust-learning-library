//! A function writes out every parameter type and its return type, and its
//! body's last expression is the value that comes back.
//! The page is 25_Control_Flow/functions/README.md.
//!
//!   rustc --edition 2024 functions.rs -o /tmp/functions && /tmp/functions

// `main` is first and calls functions defined further down the file: the order
// of items in a module does not matter.
fn main() {
    println!("=== the last expression is the return value ===");
    for n in [-5, 42, 250] {
        println!("  percent({n:>3}) = {}", percent(n));
    }

    println!("\n=== no -> means () ===");
    let receipt = log_line("saved");
    println!("  log_line returned {receipt:?}");

    println!("\n=== arguments are evaluated before the call, left to right ===");
    let total = add(traced("first", 2), traced("second", 3));
    println!("  add = {total}");
    println!("  so and(left, right) evaluates both:");
    let _ = and(traced_bool("left", false), traced_bool("right", true));
    println!("  and left && right stops at left:");
    let _ = traced_bool("left", false) && traced_bool("right", true);

    println!("\n=== no overloading, no default arguments ===");
    println!("  square_area(3)        = {}", square_area(3));
    println!("  rect_area(3, 4)       = {}", rect_area(3, 4));
    println!("  greeting(None)        = {}", greeting(None));
    println!("  greeting(Some(\"Ada\")) = {}", greeting(Some("Ada")));

    println!("\n=== a parameter is a pattern ===");
    println!("  swap((1, 2))      = {:?}", swap((1, 2)));
    println!("  first_of(7, true) = {}", first_of(7, true));

    println!("\n=== a function is a value ===");
    let tables: [(&str, fn(bool, bool) -> bool); 2] = [("and", and), ("or", or)];
    for (name, op) in tables {
        let row = [(true, true), (true, false), (false, true), (false, false)].map(|(a, b)| op(a, b));
        println!("  {name:<3} over TT TF FT FF = {row:?}");
    }
}

fn percent(n: i32) -> i32 {
    if n < 0 {
        return 0; // early exit
    }
    n.min(100) // the tail expression: no `return`, no semicolon
}

fn log_line(message: &str) {
    println!("  log: {message}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn traced(label: &str, value: i32) -> i32 {
    println!("  evaluating {label}");
    value
}

fn traced_bool(label: &str, value: bool) -> bool {
    println!("    evaluating {label}");
    value
}

fn and(a: bool, b: bool) -> bool {
    a && b
}

fn or(a: bool, b: bool) -> bool {
    a || b
}

fn square_area(side: u32) -> u32 {
    side * side
}

fn rect_area(width: u32, height: u32) -> u32 {
    width * height
}

fn greeting(name: Option<&str>) -> String {
    format!("Hello, {}!", name.unwrap_or("world"))
}

fn swap((a, b): (i32, i32)) -> (i32, i32) {
    (b, a)
}

fn first_of(n: i32, _: bool) -> i32 {
    n
}
