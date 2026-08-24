//! Kata solution: greet two people three ways, then find the three refusals behind it.
//!
//!   rustc --edition 2024 concatenating_strings_kata.rs -o /tmp/csk && /tmp/csk

fn main() {
    let first = "Ada";
    let last = "Lovelace";

    println!("PART 1 — the greeting, three ways");

    let by_format = format!("Hello, {first} {last}!");
    println!("   format!      {by_format:?}");
    println!("                first = {first:?}, last = {last:?} — both still usable");
    println!("                1 buffer, sized from the finished string");

    let by_plus = String::from("Hello, ") + first + " " + last + "!";
    println!("   chained +    {by_plus:?}");
    println!("                first and last are only borrowed on the right, so both live");
    println!("                1 buffer, grown in four steps — see PART 3");

    let by_join = ["Hello,", first, last].join(" ") + "!";
    println!("   join         {by_join:?}");
    println!("                join sizes the buffer up front, then + adds the '!'");

    println!("\n   agree: {}", by_format == by_plus && by_plus == by_join);

    println!("\nPART 2 — why the LEADING piece is the one that must be owned");
    // `+` only ever appends to its left operand, so the first fragment of the
    // sentence is the one that has to own a buffer. That is why PART 1 reads
    // String::from("Hello, ") + ... and not "Hello, " + ...
    let mut left = String::with_capacity(64);
    left.push_str("Hello, ");
    let ptr = left.as_ptr();
    let cap = left.capacity();
    let grown = left + first + " " + last + "!";
    println!("   capacity {cap} before, {} after", grown.capacity());
    println!("   same buffer the whole way: {}", grown.as_ptr() == ptr);
    println!("   {grown:?}");
    println!("   Four appends, zero new allocations — the left operand was pre-sized");
    println!("   and every `+` handed the same buffer to the next one.");

    println!("\nPART 3 — the three refusals");
    //   let x = first + last;
    //   error[E0369]: cannot add `&str` to `&str`
    //     = note: string concatenation requires an owned `String` on the left
    //
    //   let a = String::from("Ada");
    //   let b = String::from("Lovelace");
    //   let y = a + b;
    //   error[E0308]: mismatched types — expected `&str`, found `String`
    //   help: consider borrowing here:  a + &b
    //
    //   let mut t = "Hello, ";
    //   t += first;
    //   error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
    println!("   &str  + &str     E0369   neither side owns a buffer to grow");
    println!("   String + String  E0308   the ONE impl is Add<&str>; borrow the right side");
    println!("   &str  += &str    E0368   same missing buffer, assignment spelling");
    println!("   One fact, three spellings: `+` grows the left operand, so the left");
    println!("   operand has to own bytes. PART 2 is that fact used deliberately.");

    println!("\nPART 4 — what to write");
    println!("   Reach for format! by default: it does not care who owns what, and it");
    println!("   reads as the sentence it produces. Reach for + only when you already");
    println!("   hold an owned String on the LEFT and are finished with it.");
}
