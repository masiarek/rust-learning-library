//! Kata solution: four spellings, four error codes — and the one that compiles.
//!
//!   rustc --edition 2024 a_type_is_not_a_constructor_kata.rs -o /tmp/atinack && /tmp/atinack

#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
}

#[derive(Debug)]
struct Meters(u32);

#[derive(Debug)]
struct Sealed;

// The four bindings as they arrive:
//
//     let v: Player();     // 1
//     let s = Sealed();    // 2
//     let w = Player;      // 3
//     let p = Meters;      // 4  <- this one COMPILES
//
// Written correctly below, with what each refusal was actually about.

fn main() {
    println!("Four spellings of 'make me one'. Three refuse, and they refuse differently.\n");

    println!("1. let v: Player();     TWO errors, not one");
    println!("     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait");
    println!("     error[E0381]: used binding `v` isn't initialized");
    println!("   After `:` rustc reads a TYPE. `Player()` in type position is the");
    println!("   `Fn(A) -> B` sugar, and only the Fn traits may use it. Then, separately,");
    println!("   no `=` ever appeared, so nothing was assigned.");
    let v = Player { name: String::from("Ada"), level: 7 };
    println!("   fixed:  let v = Player {{ .. }};       {v:?}\n");

    println!("2. let s = Sealed();    E0618 — expected function, found struct `Sealed`");
    println!("   help: `Sealed` is a unit struct, and does not take parentheses");
    println!("         to be constructed");
    println!("   A unit struct declares a type AND a value of that type sharing one name.");
    let s = Sealed;
    println!("   fixed:  let s = Sealed;               {s:?}\n");

    println!("3. let w = Player;      E0423 — expected value, found struct `Player`");
    println!("   help: use struct literal syntax instead: `Player {{ name: val, level: val }}`");
    println!("   A named-field struct declares ONLY the type. The bare name is not a value.");
    let w = Player { name: String::from("Ben"), level: 3 };
    println!("   fixed:  let w = Player {{ .. }};       {w:?}");
    println!("   (fields read the ordinary way: w.name = {:?}, w.level = {})\n", w.name, w.level);

    println!("4. let p = Meters;      COMPILES — and gives you the wrong thing");
    println!("   `p` is not a Meters. It is the tuple struct's constructor function,");
    println!("   of type `fn(u32) -> Meters {{Meters}}`, and the next line is where");
    println!("   you find out:");
    println!("     error[E0277]: `fn(u32) -> Meters {{Meters}}` doesn't implement `Debug`");
    println!("     help: use parentheses to construct this tuple struct: `Meters(/* u32 */)`");
    let p = Meters(431);
    println!("   fixed:  let p = Meters(431);        {p:?}   (p.0 = {})\n", p.0);

    println!("5. Which name can be passed straight to `.map(..)`?");
    let ms: Vec<Meters> = vec![12u32, 40, 431].into_iter().map(Meters).collect();
    println!("   .map(Meters) works: {ms:?}");
    println!("   Only the tuple struct. `Meters` IS a function value, which is exactly");
    println!("   what refusal 4 was complaining about — the same fact, once as a bug and");
    println!("   once as a feature. `Player` is a type only, and `Sealed` is a type plus a");
    println!("   value that takes no arguments; neither is callable.");
    println!("   .map(Player) -> E0423 expected value, found struct `Player`");
    println!("   .map(Sealed) -> E0618 expected function, found struct `Sealed`\n");

    println!("6. Deferred initialization needs no `mut`");
    let high_score = 512;
    let level: u8;
    if high_score > 500 {
        level = 1;
    } else {
        level = 2;
    }
    println!("   let level: u8;  then one assignment per path  ->  level = {level}");
    println!("   `mut` permits a SECOND write. There is no second write here, so the");
    println!("   binding stays immutable and the compiler still proves it was set");
    println!("   before use. That is what `let v: Player();` was reaching for — it just");
    println!("   put a call in the type slot and never supplied the value.");
}
