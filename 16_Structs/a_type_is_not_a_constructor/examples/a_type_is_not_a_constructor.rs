//! A type is not a constructor: where a value actually comes from, and why
//! `let b: Ballot();` is two errors rather than one.
//!
//!   rustc --edition 2024 a_type_is_not_a_constructor.rs -o /tmp/atinac && /tmp/atinac

#[derive(Debug)]
struct Ballot {
    voter: String,
    score: u8,
}

#[derive(Debug)]
struct Precinct(u32); // tuple struct

#[derive(Debug)]
struct Sealed; // unit struct

impl Ballot {
    // Nothing in the language knows the name `new`. These are two ordinary
    // associated functions that happen to return Self.
    fn new(voter: &str, score: u8) -> Self {
        Ballot { voter: voter.to_string(), score }
    }
    fn blank() -> Self {
        Ballot { voter: String::from("(unmarked)"), score: 0 }
    }
}

fn main() {
    println!("1. Three flavors, three spellings — and only ONE of them is a call");
    let named = Ballot { voter: String::from("Ada"), score: 5 };
    let tuple = Precinct(7);
    let unit = Sealed;
    println!("   named-field   Ballot {{ voter, score }}  ->  {named:?}");
    println!("   tuple struct  Precinct(7)              ->  {tuple:?}");
    println!("   unit struct   Sealed                   ->  {unit:?}");
    println!(
        "   ...ordinary values, with ordinary fields: named.voter = {:?}, named.score = {}, tuple.0 = {}",
        named.voter, named.score, tuple.0
    );
    println!("   The braces are a literal; the bare name is a value. `Ballot()` is");
    println!("   not a shorter spelling of either one:");
    println!("     error[E0423]: expected function, tuple struct or tuple variant,");
    println!("                   found struct `Ballot`");
    println!("     help: use struct literal syntax instead: `Ballot {{ voter: val, score: val }}`");

    println!("\n2. The tuple struct's name really IS a function — the other two are not");
    let make: fn(u32) -> Precinct = Precinct;
    println!("   let make: fn(u32) -> Precinct = Precinct;   make(12) = {:?}", make(12));
    let ps: Vec<Precinct> = vec![3u32, 7, 12].into_iter().map(Precinct).collect();
    println!("   [3, 7, 12].map(Precinct) = {ps:?}");
    println!("   `.map(Ballot)` is E0423: `expected value, found struct `Ballot``.");
    println!("   So 'constructor' is literal for one flavor and a figure of speech for the rest.");

    println!("\n3. `Sealed` is two declarations sharing a name");
    let s: Sealed = Sealed;
    println!("   let s: Sealed = Sealed;   {s:?}");
    println!("   Left of the `=` is the TYPE namespace; right of it is the VALUE namespace.");
    println!("   `struct AlsoEmpty {{}}` declares only the type, so it needs `AlsoEmpty {{}}`.");

    println!("\n4. `new` is a convention, and std does not even keep to it");
    println!("   Ballot::new(\"Ben\", 3)   {:?}", Ballot::new("Ben", 3));
    println!("   Ballot::blank()         {:?}", Ballot::blank());
    println!("   Vec::<u8>::new()        {:?}", Vec::<u8>::new());
    println!("   String::from(\"Cara\")    {:?}", String::from("Cara"));
    println!("   u8::default()           {:?}", u8::default());
    println!("   Four names for 'make me one'. None of them is a keyword.");

    println!("\n5. `:` supplies a TYPE, `=` supplies a VALUE — and only the second is required");
    let turnout = 431;
    let decided: Ballot; // declared, not initialized — legal on its own
    if turnout > 400 {
        decided = Ballot::new("Cara", 4);
    } else {
        decided = Ballot::blank();
    }
    println!("   let decided: Ballot;  assigned on every path, exactly once");
    println!("   turnout = {turnout}  ->  {decided:?}");
    println!("   No `mut`: one assignment is not a mutation. Read it before assigning");
    println!("   and the compiler stops you:");
    println!("     error[E0381]: used binding `decided` isn't initialized");

    println!("\n6. So `let b: Ballot();` is both mistakes in eight characters");
    println!("     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait");
    println!("       after `:` rustc is reading a TYPE, and `Name(..)` in type position is");
    println!("       the `Fn(A) -> B` sugar, which only the Fn traits may use.");
    println!("     error[E0381]: used binding `b` isn't initialized");
    println!("       and no value was ever supplied, because `=` never appeared.");
    println!("   The fix is one character and one pair of braces: `let b = Ballot {{ .. }};`");
}
