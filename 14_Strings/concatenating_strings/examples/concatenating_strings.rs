//! Joining two pieces of text: what to write, and the `+` that refuses two views.
//!
//!   rustc --edition 2024 concatenating_strings.rs -o /tmp/cs && /tmp/cs

fn main() {
    let first = "Ada";
    let last = "Lovelace";

    println!("1. Joining two names");
    let by_format = format!("{first} {last}");
    println!("   format!(\"{{first}} {{last}}\")     {by_format:?}");
    let by_owned = first.to_owned() + " " + last;
    println!("   first.to_owned() + \" \" + last   {by_owned:?}");
    let by_from = String::from(first) + " " + last;
    println!("   String::from(first) + \" \" + last  {by_from:?}");
    println!("   All three allocate exactly one buffer, and all three leave");
    println!("   first = {first:?} and last = {last:?} usable afterwards.");
    println!("   Only format! reads in the order the finished sentence does.");

    println!("\n2. The one impl there is: String + &str");
    println!("   left        right      compiles   why");
    println!("   String      &str       yes        the left buffer is taken and grown");
    println!("   String      String     NO         E0308 — expected `&str`, borrow the right");
    println!("   &str        &str       NO         E0369 — neither side owns a buffer");
    println!("   &str        String     NO         E0369 — the left side still owns nothing");

    println!("\n3. What `+` does to its left operand");
    let mut owner = String::with_capacity(32);
    owner.push_str(first);
    let before_ptr = owner.as_ptr();
    let before_cap = owner.capacity();
    let full = owner + " " + last;
    // `owner` is gone here — it was moved into the result.
    println!("   capacity before {before_cap}, after {}", full.capacity());
    println!("   same heap buffer reused: {}", full.as_ptr() == before_ptr);
    println!("   {full:?}");
    println!("   The answer IS the left buffer, grown — which is why `+` consumes it,");
    println!("   and why the right side, being only borrowed, survives: {last:?}");

    println!("\n4. `+=` is push_str wearing an operator");
    let mut greeting = String::from("Hello, ");
    greeting += first;     // AddAssign<&str> for String
    greeting += "!";
    println!("   {greeting:?}");

    println!("\n5. More than two pieces");
    let parts = ["Ada", "Ben", "Cara"];
    println!("   parts.concat()        {:?}", parts.concat());
    println!("   parts.join(\", \")      {:?}", parts.join(", "));
    let owned: Vec<String> = parts.iter().map(|p| p.to_string()).collect();
    println!("   owned.join(\" | \")     {:?}", owned.join(" | "));
    println!("   Both take a slice of &str OR of String, and both size the buffer");
    println!("   up front — which a chain of `+` cannot do.");

    println!("\n6. The three refusals, and the one fact under them");
    //   let x = first + last;              E0369: cannot add `&str` to `&str`
    //   let y = String::from(first) + String::from(last);
    //                                      E0308: expected `&str`, found `String`
    //   let mut t = "Hello, "; t += first; E0368: `+=` cannot be applied to `&str`
    println!("   &str  + &str     E0369   neither side owns a buffer to grow");
    println!("   String + String  E0308   the one impl is Add<&str>; borrow the right");
    println!("   &str  += &str    E0368   same missing buffer, assignment spelling");
    println!("   All three say one thing: `+` grows its LEFT operand, so the left");
    println!("   operand has to be something that owns bytes.");

    println!("\n6b. The right operand is usually a &String, not a &str");
    let left = String::from("Hello, ");
    let right = String::from("world!");
    let joined = left + &right;          // &right is &String — it coerces
    println!("   String + &String -> {joined:?}");
    println!("   The impl is Add<&str>. &String is not &str, but deref coercion");
    println!("   converts it at the argument position, so the call goes through.");
    println!("   Coercion is a call-site conversion, NOT an extra impl — which is");
    println!("   why <String as Add<&String>>::Output cannot be named: E0277.");

    println!("\n7. Which to reach for");
    println!("   two or three known pieces        -> format!");
    println!("   a whole collection               -> .join(sep) / .concat()");
    println!("   a left value you are done with   -> + , and let it eat the buffer");
    println!("   appending in a loop              -> push_str — see Building a String");
}
