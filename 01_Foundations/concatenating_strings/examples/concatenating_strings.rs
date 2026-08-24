//! Joining two pieces of text: the `+` that refuses two literals, and the four that work.
//!
//!   rustc --edition 2024 concatenating_strings.rs -o /tmp/cs && /tmp/cs

fn main() {
    println!("1. The refusal");
    // This is the program that does not compile:
    //
    //   fn main() {
    //       let s1 = "Adam";
    //       let s2 = "Masiarek";
    //       println!("Hello, {}!", s1 + s2);
    //   }
    //
    //   error[E0369]: cannot add `&str` to `&str`
    //    --> adam.rs:5:31
    //     |
    //   5 |     println!("Hello, {}!", s1 + s2);
    //     |                            -- ^ -- &str
    //     |                            |  |
    //     |                            |  `+` cannot be used to concatenate two `&str` strings
    //     |                            &str
    //     |
    //     = note: string concatenation requires an owned `String` on the left
    //   help: create an owned `String` from a string reference
    //     |
    //   5 |     println!("Hello, {}!", s1.to_owned() + s2);
    //     |                              +++++++++++
    let s1 = "Adam";
    let s2 = "Masiarek";
    println!("   s1 and s2 are both &str — a pointer and a length, owning nothing.");
    println!("   There is nowhere to put the joined bytes, so there is no `+` to call.");

    println!("\n2. The one impl there is: String + &str");
    println!("   left        right      compiles   why");
    println!("   &str        &str       NO         E0369 — neither side owns a buffer");
    println!("   &str        String     NO         E0369 — the left side still owns nothing");
    println!("   String      String     NO         E0308 — expected `&str`, found `String`");
    println!("   String      &str       yes        the left buffer is taken and grown");

    println!("\n3. Three ways to say what Adam meant");
    let a = format!("{s1} {s2}");
    println!("   format!(\"{{s1}} {{s2}}\")   {a:?}   s1, s2 both still usable");
    let b = s1.to_owned() + " " + s2;
    println!("   s1.to_owned() + \" \" + s2  {b:?}   the compiler's own suggestion");
    let c = String::from(s1) + " " + s2;
    println!("   String::from(s1) + \" \" + s2  {c:?}   same thing, spelled differently");
    println!("   All three allocate exactly one buffer. Only format! reads as the sentence it builds.");

    println!("\n4. What `+` actually does to its left operand");
    let mut owner = String::with_capacity(32);
    owner.push_str("Adam");
    let before_ptr = owner.as_ptr();
    let before_cap = owner.capacity();
    let full = owner + " " + s2;
    // `owner` is gone here — it was moved into the result.
    println!("   capacity before {before_cap}, after {}", full.capacity());
    println!("   same heap buffer reused: {}", full.as_ptr() == before_ptr);
    println!("   {full:?}");
    println!("   That is why `+` consumes the left side: the answer IS the left buffer, grown.");
    println!("   The right side is only borrowed, so s2 is still usable: {s2:?}");

    println!("\n5. `+=` works where `+` does not — on a String");
    let mut greeting = String::from("Hello, ");
    greeting += s1;        // AddAssign<&str> for String — this is push_str with an operator
    greeting += "!";
    println!("   {greeting:?}");
    println!("   `let mut t = \"Hello, \"; t += s1;` is E0368 — a &str has no buffer to append to.");

    println!("\n6. More than two pieces");
    let parts = ["Ada", "Ben", "Cara"];
    println!("   parts.concat()        {:?}", parts.concat());
    println!("   parts.join(\", \")      {:?}", parts.join(", "));
    let owned: Vec<String> = parts.iter().map(|p| p.to_string()).collect();
    println!("   owned.join(\" | \")     {:?}", owned.join(" | "));
    println!("   Both take a slice of &str OR of String, and both allocate once,");
    println!("   sized up front — which a chain of `+` cannot do.");

    println!("\n7. Which to reach for");
    println!("   two or three known pieces        -> format!");
    println!("   a whole collection               -> .join(sep) / .concat()");
    println!("   a left value you are done with   -> + , and let it eat the buffer");
    println!("   appending in a loop              -> push_str — see Building a String");
}
