//! What your editor's ghost text is claiming — written so the compiler checks it.
//!
//! Every editor compared on the page beside this file draws the inferred type
//! after a `let` that states none. The hint is worth trusting; it is also worth
//! knowing what it is. It is not a rule about the literal on the right. It is a
//! conclusion the compiler reached after reading the whole function, including
//! the lines below the one you are looking at.

fn main() {
    // Nothing here states a type. Your editor draws one after each name.
    let seats = 3;
    let quota = 0.5;
    let winner = "Ada";

    // The same three claims, written as annotations the compiler must check.
    // Change any one of them to another type and this file stops compiling.
    let _: i32 = seats;
    let _: f64 = quota;
    let _: &str = winner;

    println!("alone on the page, an integer literal settles on i32: {seats}");
    println!("a float literal settles on f64: {quota}");
    println!("a bare string literal is a borrowed &str: {winner}");

    // Now the part that reading top-to-bottom will not give you.
    let votes = 3; // written exactly like `seats` above...
    let total: u64 = 10;
    let sum = total + votes; // ...until this line, two lines later

    let _: u64 = votes; // and this is what the hint says now
    let _: u64 = sum;

    println!();
    println!("`votes` is the same literal on the same shape of line as `seats`.");
    println!("Adding it to a u64 two lines later makes it a u64: {votes} + {total} = {sum}");
    println!("Inference runs over the whole body, so the hint on a `let` can be");
    println!("a conclusion drawn from a line you have not read yet. That is the");
    println!("thing an editor tells you and a printout of the source does not.");
}
