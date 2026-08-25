//! Kata solution: two places, or one?
//!
//! Part 1 is the reference test, both ways — the shadow compiles, the `mut`
//! spelling does not, and the error is the point.
//! Part 2 repeats it on a `String`, so `Copy` cannot be blamed.
//! Part 3 times the drops, which is how you find out that whole-value
//! assignment destroys the old value rather than editing it.
//! Part 4 is the guessing-game shape where every Rust learner first meets
//! this, and the warning that sends them here.
//! Part 5 is the three-way multiple choice that circulates as a quiz, with
//! the distractor its answer key gets wrong.
//!
//!   rustc --edition 2024 a_name_is_not_a_place_kata.rs -o /tmp/anipk && /tmp/anipk

struct Tracked(&'static str);

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("      drop: {}", self.0);
    }
}

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// Part 1 — the four-line test that settles it.
fn two_places() {
    banner("1. Two places, one name");

    let x = 5;
    let y = &x; //   borrows the first place
    let x = 6; //    declares a second place and moves the name onto it
    println!("      y = {y}, x = {x}   <- both alive, so there are two places");

    println!("      The `mut` spelling of the same four lines is rejected:");
    println!("        let mut x = 5;");
    println!("        let y = &x;");
    println!("        x = 6;   error[E0506]: cannot assign to `x`");
    println!("                              because it is borrowed");
    println!("      One place. `y` is reading it. The write cannot be allowed.");
}

/// Part 2 — the same shape on a type that is emphatically not `Copy`.
fn not_a_copy_trick() {
    banner("2. Not a `Copy` trick");

    let name = String::from("Ada");
    let seen = &name; //                 borrows the first String
    let name = name.to_uppercase(); //   allocates a second String
    println!("      seen = {seen}   <- the first String, still owned, still allocated");
    println!("      name = {name}   <- what the name means from here on");
}

/// Part 3 — when does the first value actually die?
fn when_does_it_die() {
    banner("3. Assignment drops; a shadow does not");

    println!("    mut:");
    {
        let mut slot = Tracked("first");
        println!("      holding {}", slot.0);
        slot = Tracked("second"); //   "first" dies on THIS line
        println!("      assigned — and the drop above has already run");
        println!("      holding {}", slot.0);
    }

    println!("    shadow:");
    {
        let slot = Tracked("first");
        println!("      holding {}", slot.0);
        let slot = Tracked("second"); //   nothing dies here
        println!("      shadowed — nothing has dropped");
        println!("      holding {}", slot.0);
    } //                                   both die here, newest first
}

/// Part 4 — the guessing game, with the keyboard simulated so the output is
/// an answer key rather than a transcript of whoever ran it.
fn the_guess_that_needs_no_mut() {
    banner("4. Why `let guess: u32` needs no `mut`");

    const SECRET: u32 = 42;
    const TYPED: [&str; 3] = ["  fifty\n", " 7 \n", "42\n"];

    for typed in TYPED {
        // `mut` here is real: `read_line` writes into this buffer.
        let mut guess = String::new();
        guess.push_str(typed); //   stands in for io::stdin().read_line(&mut guess)

        // A second variable, of a second type, in a second place. It is
        // written once at birth and only read afterwards — so `mut` on it
        // would earn `unused_mut`, which is the warning that starts the
        // confusion this page is about.
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("      {:?} -> not a number, next", typed.trim());
                continue;
            }
        };

        if guess == SECRET {
            println!("      {guess} -> correct");
        } else if guess < SECRET {
            println!("      {guess} -> too small");
        } else {
            println!("      {guess} -> too big");
        }
    }

    println!("      Two variables named `guess`, and neither is ever reassigned:");
    println!("      the String is mutated through `&mut`, the u32 is initialized once.");
}


/// Part 5 — three ways to write one line, and only one of them to ship.
///
/// The circulating quiz gives a string, asks for a number, and offers
/// `let mut age = …`, `let age = …` and a bare `age = …`. Its answer (the
/// middle one) is right; its reason for rejecting the first one is not.
fn three_ways_to_write_one_line() {
    banner("5. String in, number out — which line do you write?");

    let age = "30"; //   a &str, and the rest of the function wants a number
    println!("      A)  let mut age = age.parse::<u32>().unwrap();   compiles — with a warning");
    println!("      B)  let age     = age.parse::<u32>().unwrap();   compiles clean   <- ship this");
    println!("      C)      age     = age.parse::<u32>().unwrap();   error[E0308]");

    // B, for real. A second variable, of a second type, wearing the same name.
    let age = age.parse::<u32>().unwrap();
    println!("      B runs: age = {age}, and its type is now u32");

    println!("    C is not a style question — it does not compile:");
    println!("        error[E0308]: mismatched types");
    println!("          |     let age = \"30\";");
    println!("          |               ---- expected due to this value");
    println!("          |     age = age.parse::<u32>().unwrap();");
    println!("          |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `u32`");
    println!("      And `mut` does not rescue it. Write `let mut age = \"30\";` on the");
    println!("      first line and C reports the SAME E0308 — assignment writes into a");
    println!("      place, and that place is &str-shaped for as long as it exists.");
    println!("      (E0384, `cannot assign twice to immutable variable`, is the error");
    println!("      people expect here. You only reach it once the types match.)");

    println!("    A is the one the answer key gets wrong. It compiles:");
    println!("        warning: variable does not need to be mutable");
    println!("          |     let mut age = age.parse::<u32>().unwrap();");
    println!("          |         ----^^^");
    println!("          |         help: remove this `mut`");
    println!("      That is `unused_mut`, and it is a narrow, checkable complaint:");
    println!("      nothing below ever reassigns `age`. It is not a ruling against");
    println!("      shadowing and `mut` in one line — that combination is ordinary:");

    // The same shape as A, with the `mut` earned — so no warning fires.
    let birthday = "30";
    let mut birthday = birthday.parse::<u32>().unwrap(); //   shadow AND mut
    birthday += 1; //                                        this is what earns it
    println!("      let mut birthday = birthday.parse::<u32>().unwrap();");
    println!("      birthday += 1;   ->  {birthday}   <- one shadow, one mut, no warning");

    println!("      So: pick B because the compiler asked you to, not because the");
    println!("      two keywords cannot be spelled on one line.");
}

fn main() {
    two_places();
    not_a_copy_trick();
    when_does_it_die();
    the_guess_that_needs_no_mut();
    three_ways_to_write_one_line();
}
