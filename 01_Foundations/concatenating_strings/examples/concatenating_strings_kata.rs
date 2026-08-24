//! Kata solution: greet two people, and make the compiler refuse it three ways first.
//!
//!   rustc --edition 2024 concatenating_strings_kata.rs -o /tmp/csk && /tmp/csk

fn main() {
    let first = "Adam";
    let last = "Masiarek";

    println!("PART 1 — the three refusals");
    //
    //   let x = first + last;
    //   error[E0369]: cannot add `&str` to `&str`
    //     = note: string concatenation requires an owned `String` on the left
    //
    //   let a = String::from("Adam");
    //   let b = String::from("Masiarek");
    //   let y = a + b;
    //   error[E0308]: mismatched types — expected `&str`, found `String`
    //   help: consider borrowing here:  a + &b
    //
    //   let mut t = "Hello, ";
    //   t += first;
    //   error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
    //
    println!("   &str  + &str    E0369  neither side owns a buffer to grow");
    println!("   String + String E0308  the ONE impl is Add<&str>; borrow the right side");
    println!("   &str  += &str   E0368  same missing buffer, assignment spelling");
    println!("   All three are the same fact: `+` grows the left operand, so the left");
    println!("   operand has to be something that owns bytes.");

    println!("\nPART 2 — the greeting, three ways");

    let by_format = format!("Hello, {first} {last}!");
    println!("   format!      {by_format:?}");
    println!("                first = {first:?}, last = {last:?} — both still usable");
    println!("                1 buffer, sized from the finished string");

    let by_plus = String::from("Hello, ") + first + " " + last + "!";
    println!("   chained +    {by_plus:?}");
    println!("                first = {first:?}, last = {last:?} — only borrowed on the right");
    println!("                1 buffer, but grown in four steps — see PART 3");

    let by_join = ["Hello,", first, last].join(" ") + "!";
    println!("   join         {by_join:?}");
    println!("                join sizes the buffer up front, then + adds the '!'");

    println!("\n   agree: {}", by_format == by_plus && by_plus == by_join);

    println!("\nPART 3 — why the LEADING piece is the one that must be owned");
    // `+` can only ever append to its left operand, so the first piece of the
    // sentence is the one that has to own a buffer. That is the whole reason
    // this reads String::from("Hello, ") + ... and not "Hello, " + ...
    let mut left = String::with_capacity(64);
    left.push_str("Hello, ");
    let ptr = left.as_ptr();
    let cap = left.capacity();
    let grown = left + first + " " + last + "!";
    println!("   capacity {cap} before, {} after", grown.capacity());
    println!("   same buffer the whole way: {}", grown.as_ptr() == ptr);
    println!("   {grown:?}");
    println!("   Four appends, zero new allocations — because the left operand was");
    println!("   pre-sized and every `+` handed the same buffer to the next one.");

    println!("\nPART 4 — what to write");
    println!("   Reach for format! by default: it does not care who owns what, and it");
    println!("   reads as the sentence it produces. Reach for + only when you already");
    println!("   hold an owned String on the LEFT and are finished with it.");
}
