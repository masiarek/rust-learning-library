//! A block is an expression.
//!
//! `{ }` does two jobs, and most people only ever meet the first. It opens a
//! SCOPE — a name declared inside stops existing at the closing brace. It is
//! also an EXPRESSION — its value is its last line written without a semicolon.
//! The second job is why a function body needs no `return`, why `if` can sit on
//! the right-hand side of a `let`, and why adding one semicolon changes what a
//! block is worth from an `i32` to `()`.
//!
//!   rustc --edition 2024 a_block_is_an_expression.rs -o /tmp/abie && /tmp/abie

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// A function body IS a block, and `n * 2` is its tail expression.
fn double(n: i32) -> i32 {
    n * 2
}

/// Something to call when the point is the semicolon, not the arithmetic.
fn tally() -> i32 {
    6
}

fn main() {
    banner("Job 1: it opens a scope, anywhere you like");

    let n = 5;
    println!("  outer n is: {n}");
    {
        let n = 10; //      a second variable; the brace below ends it
        println!("    inner n is: {n}");
    }
    println!("  outer n is: {n}   <- the outer one was never touched");

    banner("...and a name declared inside is gone at the brace");

    println!("  {{ let a = 1; }}");
    println!("  println!(\"{{a}}\");   <- error[E0425]: cannot find value `a`");
    println!("  No subtlety: past the brace, the name does not exist.");

    banner("Job 2: it has a VALUE — its last line, with no semicolon");

    let quorum = {
        let voters = 9;
        let half = voters / 2;
        half + 1 //         no semicolon: this is what the block is worth
    };
    println!("  quorum = {quorum}");

    banner("Both jobs at once — the snippet that circulates as a quiz");

    let x = 10;
    let y = {
        let x = 3; //   job 1: a second `x`, ending at the brace below
        x + 1 //        job 2: no semicolon, so this is what `y` is worth
    };
    println!("  x: {x}, y: {y}");
    println!("  Two semicolons, two jobs: the one after `}}` ends the `let`,");
    println!("  and the one MISSING after `x + 1` is what gives `y` a value.");

    banner("The semicolon is the switch");

    let with_tail = { tally() };
    let with_semi = { tally(); };
    println!("  {{ tally() }}    is {with_tail}");
    println!("  {{ tally(); }}   is {with_semi:?}      <- the unit value");
    println!("  Same block, one character apart, two different types.");

    banner("So a function body was a block all along");

    println!("  fn double(n: i32) -> i32 {{ n * 2 }}");
    println!("  double(4) = {}   <- a tail expression, not a `return`", double(4));

    banner("What it is FOR (1): scoping the `mut` to the building");

    let ballots = {
        let mut v = Vec::new();
        v.push(5);
        v.push(3);
        v.push(4);
        v //            hand the finished Vec out; the `mut` stays behind
    };
    println!("  ballots = {ballots:?}");
    println!("  `ballots` is not `mut`, and no line below here can grow it.");

    banner("What it is FOR (2): giving a shadow an end");

    let name = String::from("ada");
    {
        let name = name.to_uppercase();
        println!("    inside:  {name}");
    }
    println!("  outside: {name}   <- the shadow ended at the brace");

    banner("What it is FOR (3): the branch that decides a value");

    let turnout = 61;
    let verdict = if turnout >= 50 { "quorate" } else { "short" };
    println!("  turnout {turnout}% -> {verdict}");
    println!("  `if` is an expression because its arms are blocks.");
}
