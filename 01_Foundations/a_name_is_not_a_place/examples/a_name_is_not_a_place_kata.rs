//! Kata solution: two places, or one?
//!
//! Part 1 is the reference test, both ways — the shadow compiles, the `mut`
//! spelling does not, and the error is the point.
//! Part 2 repeats it on a `String`, so `Copy` cannot be blamed.
//! Part 3 times the drops, which is how you find out that whole-value
//! assignment destroys the old value rather than editing it.
//! Part 4 is the guessing-game shape where every Rust learner first meets
//! this, and the warning that sends them here.
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

fn main() {
    two_places();
    not_a_copy_trick();
    when_does_it_die();
    the_guess_that_needs_no_mut();
}
