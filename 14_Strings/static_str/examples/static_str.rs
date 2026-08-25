//! `&'static str`: three spellings of one type, and the claim that is not true.
//!
//!   rustc --edition 2024 static_str.rs -o /tmp/st && /tmp/st

use std::any::TypeId;
use std::sync::OnceLock;

const GREETING: &str = "hi";
static BANNER: &str = "hi";
static BUILT: OnceLock<String> = OnceLock::new();

#[derive(Debug)]
enum MyError {
    Io,
    Parse(&'static str),
}

fn greeting(hour: u8) -> &'static str {
    if hour < 12 {
        "Good morning!"
    } else if hour < 18 {
        "Good afternoon!"
    } else {
        "Good evening!"
    }
}

/// Runtime text that outlives the function anyway — the buffer is never freed.
fn leaked(n: u8) -> &'static str {
    format!("built at runtime from {n}").leak()
}

fn from_a_static() -> &'static str {
    BUILT.get_or_init(|| String::from("stored in a static")).as_str()
}

fn takes_any_static<T: 'static>(label: &str) {
    println!("      {label} satisfies T: 'static");
}

fn main() {
    println!("1. Three spellings, one type");
    let s1 = "hi";
    let s2: &str = "hi";
    let s3: &'static str = "hi";
    println!("   let s1 = \"hi\";                  {s1:?}");
    println!("   let s2: &str = \"hi\";            {s2:?}");
    println!("   let s3: &'static str = \"hi\";    {s3:?}");
    println!("   TypeId::of::<&str>() == TypeId::of::<&'static str>() -> {}",
             TypeId::of::<&str>() == TypeId::of::<&'static str>());
    println!("   ...and that comparison is vacuous: TypeId::of<T> requires T: 'static,");
    println!("   so `&str` written there ALREADY means `&'static str`. One type, twice.");

    println!("\n2. The annotation only bites when the text is not a literal");
    let owned = String::from("a local String");
    let view: &str = &owned;
    println!("   let view: &str = &owned;          {view:?}   <- compiles");
    println!("   let view: &'static str = &owned;  E0597: `owned` does not live long enough");
    println!("   On a literal the two annotations agree. On a borrow they do not.");

    println!("\n3. const, static, let");
    println!("   const  GREETING: &str = \"hi\";   {GREETING:?}   inlined at each use, no address");
    println!("   static BANNER:   &str = \"hi\";   {BANNER:?}   one address, lives the whole run");
    println!("   let    s1             = \"hi\";   {s1:?}   a local name for the same bytes");
    println!("   All three point into the binary. `const` and `static` differ in whether");
    println!("   there is one object or a copy per use — not in how long the text lives.");

    println!("\n4. \"A String can never be borrowed as &'static str\" — three ways it can");
    println!("   String::leak()          {:?}", leaked(7));
    println!("   Box::leak(into_boxed)   {:?}", Box::leak(String::from("boxed then leaked").into_boxed_str()));
    println!("   a String in a static    {:?}", from_a_static());
    println!("   The first two never free the buffer — that is the price, and it is");
    println!("   deliberate: 'static means \"never dropped\", not \"in the binary\".");

    println!("\n5. 'static the BOUND is not 'static the reference");
    takes_any_static::<String>("String");
    takes_any_static::<&'static str>("&'static str");
    takes_any_static::<i32>("i32");
    println!("   T: 'static means \"contains no borrow that could expire\" — every owned");
    println!("   type qualifies, String included. It does NOT mean \"lives forever\".");

    println!("\n6. Where you actually write it");
    println!("   in a return type:  greeting(9) = {:?}", greeting(9));
    println!("                      greeting(20) = {:?}", greeting(20));
    for e in [MyError::Parse("expected a digit"), MyError::Io] {
        let detail = match &e {
            MyError::Parse(why) => *why,
            MyError::Io => "no detail carried",
        };
        println!("   in an enum:        {e:?}  ->  {detail:?}");
    }
    println!("   Both work because every arm is a literal. Return a String instead the");
    println!("   moment one arm needs to say which digit it expected.");
}
