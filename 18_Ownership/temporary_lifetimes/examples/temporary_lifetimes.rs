//! Temporary lifetime extension: which `let` keeps a temporary alive, and which drops it
//! at the semicolon. `Drop` prints, so the transcript below IS the answer.
//!
//!   rustc --edition 2024 temporary_lifetimes.rs -o /tmp/tl && /tmp/tl

struct Noisy(&'static str);

impl Noisy {
    /// Returns a `&'static str`, so nothing here borrows from `self`.
    fn name(&self) -> &'static str {
        self.0
    }

    /// Always `None`, so the `if let` below always takes its `else`.
    fn seat(&self) -> Option<u8> {
        None
    }
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("      [drop {}]", self.0);
    }
}

fn make(tag: &'static str) -> Noisy {
    Noisy(tag)
}

struct Holder<'a> {
    r: &'a Noisy,
}

/// The same shape with no `Drop` impl — which is what lets it be promoted.
struct Quiet(&'static str);

struct Pair {
    first: Noisy,
    second: Noisy,
}

fn pair() -> Pair {
    let p = Pair { first: Noisy("P1"), second: Noisy("P2") };
    println!("   made a Pair            -> {} + {}", p.first.0, p.second.0);
    p
}

fn main() {
    println!("1. `&` in a let extends; anything else drops at the semicolon");
    {
        println!("   before both lets");
        let extended = &make("A"); // extending expression -> lives to the `}`
        let name = make("B").name(); // a method call is not -> B dies at the `;`
        println!("   after  both lets       -> A is {}, B was {}", extended.0, name);
        println!("   leaving the block");
    }

    println!();
    println!("2. A struct literal is an extending expression too");
    {
        let h = Holder { r: &make("H") };
        println!("   Holder built inline    -> {}", h.r.0);
        println!("   leaving the block");
    }

    println!();
    println!("3. Extending one field keeps the WHOLE temporary alive");
    {
        let f = &pair().first;
        println!("   &pair().first          -> {}", f.0);
        println!("   leaving the block      -> both fields are still here");
    }

    println!();
    println!("4. A match holds its scrutinee's temporaries through every arm");
    println!("   entering the match");
    match make("M").name() {
        "M" => println!("   inside the arm         -> M is still alive"),
        other => unreachable!("unexpected {other}"),
    }
    println!("   left the match");

    println!();
    println!("5. ...and the same expression in a let does not");
    let name = make("S").name();
    println!("   after the let          -> {name} was returned, S is already gone");

    println!();
    println!("6. An `if let` releases its scrutinee BEFORE the else — since edition 2024");
    println!("   entering the if let");
    if let Some(n) = make("IF").seat() {
        println!("   then branch            -> {n}");
    } else {
        println!("   else branch            -> IF is already gone");
    }
    println!("   left the if let");

    println!();
    println!("7. With no `Drop` and no interior mutability, there is nothing to extend");
    let promoted: &'static Quiet = &Quiet("Q");
    println!("   &Quiet(..) as &'static -> {}", promoted.0);
    println!("   the annotation is the proof: rvalue static promotion, not extension");
}
