//! Five ways to get a `String`, and the trait that gives you the last one free.
//!
//!   rustc --edition 2024 making_a_string.rs -o /tmp/ms && /tmp/ms

use std::fmt;

/// Implement Display, and `to_string()` arrives on its own.
struct Score(u8);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} star{}", self.0, if self.0 == 1 { "" } else { "s" })
    }
}

fn main() {
    println!("1. Five ways to turn a &str into a String");
    let literal = "equal vote";
    let a = literal.to_string();
    let b = literal.to_owned();
    let c = String::from(literal);
    let d: String = literal.into();
    let e = format!("{literal}");
    println!("   literal.to_string()   {a:?}");
    println!("   literal.to_owned()    {b:?}");
    println!("   String::from(literal) {c:?}");
    println!("   literal.into()        {d:?}   <- needs the annotation to pick a target");
    println!("   format!(\"{{literal}}\")   {e:?}   <- the only one that can also reshape");
    println!("   all equal? {}", a == b && b == c && c == d && d == e);

    println!("\n2. They are not all the same call");
    println!("   to_owned()   the borrowed -> owned conversion, defined on str itself");
    println!("   String::from the From impl, same machinery, reads as a constructor");
    println!("   to_string()  goes through Display — universal, and the one to reach for");
    println!("                on ANY printable type, not just &str");
    println!("   into()       From, backwards; fine when the target type is already known");
    println!("   format!()    allocates and runs the formatter — use it when you are");
    println!("                building, not merely converting");

    println!("\n3. to_string() works on anything that prints");
    println!("   42.to_string()        {:?}", 42.to_string());
    println!("   3.5_f64.to_string()   {:?}", 3.5_f64.to_string());
    println!("   true.to_string()      {:?}", true.to_string());
    println!("   'x'.to_string()       {:?}", 'x'.to_string());
    println!("   Score(4).to_string()  {:?}   <- our own type, no ToString impl written", Score(4).to_string());
    println!("   Score(1).to_string()  {:?}", Score(1).to_string());

    println!("\n4. Why you never write `impl ToString`");
    println!("   alloc already has:  impl<T: Display + ?Sized> ToString for T");
    println!("   so writing your own is E0119: conflicting implementations.");
    println!("   Implement Display. ToString, and `{{}}`, and format!, all follow.");

    println!("\n5. The one that is not a conversion");
    let owned = String::from("already owned");
    let copy = owned.to_string();
    println!("   owned.to_string() on a String   {copy:?}");
    println!("   That is a full clone — a second heap buffer. It compiles, it is silent,");
    println!("   and in a loop it is the allocation nobody meant to write. Wanted a view?");
    println!("   &owned is free. Wanted the value? Move it.");

    println!("\n6. Coming back the other way");
    let text = "42";
    let n: i32 = text.parse().unwrap();
    let m = text.parse::<i32>().unwrap();
    let bad = "forty-two".parse::<i32>();
    println!("   \"42\".parse::<i32>()        {m:?}      annotation or turbofish, pick one");
    println!("   let n: i32 = text.parse()  {n:?}      same call, type from the binding");
    println!("   \"forty-two\".parse::<i32>() {bad:?}");
    println!("   parse() returns Result, because text is input and input lies.");
}
