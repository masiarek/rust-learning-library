//! Kata solution: four spellings, four error codes — and the one that compiles.
//!
//!   rustc --edition 2024 a_type_is_not_a_constructor_kata.rs -o /tmp/atinack && /tmp/atinack

#[derive(Debug)]
struct Voter {
    name: String,
    seat: u8,
}

#[derive(Debug)]
struct Precinct(u32);

#[derive(Debug)]
struct Sealed;

// The four bindings as they arrive:
//
//     let v: Voter();      // 1
//     let s = Sealed();    // 2
//     let w = Voter;       // 3
//     let p = Precinct;    // 4  <- this one COMPILES
//
// Written correctly below, with what each refusal was actually about.

fn main() {
    println!("Four spellings of 'make me one'. Three refuse, and they refuse differently.\n");

    println!("1. let v: Voter();      TWO errors, not one");
    println!("     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait");
    println!("     error[E0381]: used binding `v` isn't initialized");
    println!("   After `:` rustc reads a TYPE. `Voter()` in type position is the");
    println!("   `Fn(A) -> B` sugar, and only the Fn traits may use it. Then, separately,");
    println!("   no `=` ever appeared, so nothing was assigned.");
    let v = Voter { name: String::from("Ada"), seat: 7 };
    println!("   fixed:  let v = Voter {{ .. }};        {v:?}\n");

    println!("2. let s = Sealed();    E0618 — expected function, found struct `Sealed`");
    println!("   help: `Sealed` is a unit struct, and does not take parentheses");
    println!("         to be constructed");
    println!("   A unit struct declares a type AND a value of that type sharing one name.");
    let s = Sealed;
    println!("   fixed:  let s = Sealed;               {s:?}\n");

    println!("3. let w = Voter;       E0423 — expected value, found struct `Voter`");
    println!("   help: use struct literal syntax instead: `Voter {{ name: val, seat: val }}`");
    println!("   A named-field struct declares ONLY the type. The bare name is not a value.");
    let w = Voter { name: String::from("Ben"), seat: 3 };
    println!("   fixed:  let w = Voter {{ .. }};        {w:?}");
    println!("   (fields read the ordinary way: w.name = {:?}, w.seat = {})\n", w.name, w.seat);

    println!("4. let p = Precinct;    COMPILES — and gives you the wrong thing");
    println!("   `p` is not a Precinct. It is the tuple struct's constructor function,");
    println!("   of type `fn(u32) -> Precinct {{Precinct}}`, and the next line is where");
    println!("   you find out:");
    println!("     error[E0277]: `fn(u32) -> Precinct {{Precinct}}` doesn't implement `Debug`");
    println!("     help: use parentheses to construct this tuple struct: `Precinct(/* u32 */)`");
    let p = Precinct(431);
    println!("   fixed:  let p = Precinct(431);        {p:?}   (p.0 = {})\n", p.0);

    println!("5. Which name can be passed straight to `.map(..)`?");
    let ps: Vec<Precinct> = vec![12u32, 40, 431].into_iter().map(Precinct).collect();
    println!("   .map(Precinct) works: {ps:?}");
    println!("   Only the tuple struct. `Precinct` IS a function value, which is exactly");
    println!("   what refusal 4 was complaining about — the same fact, once as a bug and");
    println!("   once as a feature. `Voter` is a type only, and `Sealed` is a type plus a");
    println!("   value that takes no arguments; neither is callable.");
    println!("   .map(Voter)  -> E0423 expected value, found struct `Voter`");
    println!("   .map(Sealed) -> E0618 expected function, found struct `Sealed`\n");

    println!("6. Deferred initialization needs no `mut`");
    let turnout = 512;
    let seat: u8;
    if turnout > 500 {
        seat = 1;
    } else {
        seat = 2;
    }
    println!("   let seat: u8;  then one assignment per path  ->  seat = {seat}");
    println!("   `mut` permits a SECOND write. There is no second write here, so the");
    println!("   binding stays immutable and the compiler still proves it was set");
    println!("   before use. That is what `let v: Voter();` was reaching for — it just");
    println!("   put a call in the type slot and never supplied the value.");
}
