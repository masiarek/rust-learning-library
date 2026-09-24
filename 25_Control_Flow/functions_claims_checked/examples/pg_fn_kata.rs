//! Kata solution: six functions and blocks — which compile, and what each
//! one returns. The two that do not compile are described, not written;
//! their transcripts are on the page.

fn triple(n: i32) -> i32 {
    n * 3
}

fn b(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    0
}

fn c(x: i32) -> &'static str {
    if x % 2 == 0 { "even" } else { "odd" }
}

fn e() -> (i32, i32) {
    (1, 2)
}

fn main() {
    println!("1. fn a() -> i32 {{ 5; }}");
    println!("   does not compile: E0308, expected `i32`, found `()` -- the ; made the tail a statement");

    println!("2. fn b(x: i32) -> i32 {{ if x > 0 {{ return x; }} 0 }}");
    println!("   compiles: b(4) = {}, b(-4) = {}", b(4), b(-4));

    println!("3. fn c(x: i32) -> &'static str {{ if x % 2 == 0 {{ \"even\" }} else {{ \"odd\" }} }}");
    println!("   compiles: c(7) = {}, c(8) = {}", c(7), c(8));

    println!("4. fn d() {{ 5 }}");
    println!("   does not compile: E0308, expected `()`, found integer -- no -> means ()");

    println!("5. fn e() -> (i32, i32) {{ (1, 2) }}");
    let (p, q) = e();
    println!("   compiles: e() = {:?}, one value; p + q = {}", e(), p + q);

    println!("6. let v = {{ let x = 2; triple(x) }}; and let w = {{ let x = 2; triple(x); }};");
    let v = {
        let x = 2;
        triple(x)
    };
    let w = {
        let x = 2;
        triple(x);
    };
    println!("   both compile: v = {v}, w = {w:?}");
}
