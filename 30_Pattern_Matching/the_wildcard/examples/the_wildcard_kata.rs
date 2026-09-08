//! Kata solution: `_` binds nothing, so it drops nothing.
//!
//!   rustc --edition 2024 the_wildcard_kata.rs -o /tmp/twk && /tmp/twk

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("    <- dropped {}", self.0);
    }
}

fn main() {
    println!("1. `let _ = x;` DROPS IMMEDIATELY");
    println!("  let _ = Noisy(\"A\");");
    let _ = Noisy("A");
    println!("  ...and the drop already happened, above this line.");
    println!();

    println!("2. `let _name = x;` DROPS AT THE END OF SCOPE");
    println!("  let _b = Noisy(\"B\");");
    let _b = Noisy("B");
    println!("  ...still alive here.");
    println!();

    println!("  The two look like the same 'I do not need this' and they are");
    println!("  opposite. `_` is a PATTERN that binds nothing, so the value is");
    println!("  never given a name, never owned by anything, and is dropped at");
    println!("  the end of the statement. `_b` is a NAME beginning with an");
    println!("  underscore -- an ordinary binding that silences the unused");
    println!("  warning and keeps the value alive to the end of the block.");
    println!();
    println!("  This is the classic lock bug: `let _ = mutex.lock();` releases");
    println!("  the guard instantly and protects nothing, while");
    println!("  `let _guard = mutex.lock();` holds it for the block.");
    println!();

    println!("3. `_` MOVES NOTHING");
    let owned = String::from("still here");
    match &owned {
        _ => println!("  matched with `_`, and owned is untouched: {owned:?}"),
    }
    let tuple = (String::from("left"), String::from("right"));
    let (first, _) = &tuple;
    println!("  destructured with `_`: first = {first:?}, tuple intact = {:?}", tuple.1);
    println!("  Because it binds nothing, there is nothing to move -- so `_`");
    println!("  can appear where a real binding would take ownership away.");
    println!();

    println!("4. `_` IN A MATCH IS A COMMITMENT");
    println!("  `_ => ...` makes a match exhaustive forever, which is right for");
    println!("  a type you do not own and wrong for one you do: the next");
    println!("  variant will take that arm instead of being reported.");
    println!("  `..` is the version that says 'the rest of these FIELDS', and");
    println!("  it has the same trade-off inside a struct pattern.");
    println!();

    println!("5. AND IT IS NOT A NAME");
    println!("  `_` cannot be read: `let _ = 5; println!(\"{{}}\", _);` is a");
    println!("  syntax error, because there is no binding to name. That is the");
    println!("  single sentence every surprise above follows from.");
    println!();
    println!("  end of main:");

    assert_eq!(owned, "still here");
}
