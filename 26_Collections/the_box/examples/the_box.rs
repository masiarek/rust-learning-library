//! `Box<T>`: one value, moved off the stack, with a pointer left behind.
//!
//!   rustc --edition 2024 the_box.rs -o /tmp/box && /tmp/box

/// A type big enough that moving it around matters.
#[derive(Debug)]
struct Ballot {
    scores: [u32; 64],
}

/// The type that cannot exist without a Box — see section 3.
#[derive(Debug)]
enum Round {
    Final(&'static str),
    Then(&'static str, Box<Round>),
}

fn winner(r: &Round) -> &'static str {
    match r {
        Round::Final(name) => name,
        Round::Then(_, rest) => winner(rest),
    }
}

/// Everyone eliminated on the way to the winner, outermost first.
fn eliminated(r: &Round) -> Vec<&'static str> {
    match r {
        Round::Final(_) => Vec::new(),
        Round::Then(name, rest) => {
            let mut out = vec![*name];
            out.extend(eliminated(rest));
            out
        }
    }
}

fn main() {
    println!("1. A Box is a pointer, whatever it points at");
    println!("   size_of::<Ballot>()      = {}", size_of::<Ballot>());
    println!("   size_of::<Box<Ballot>>() = {}", size_of::<Box<Ballot>>());
    println!("   size_of::<Box<u8>>()     = {}", size_of::<Box<u8>>());
    println!("   The 256 bytes moved to the heap; 8 bytes stayed on the stack.");
    println!("   Moving a Box copies those 8 bytes and nothing else.");

    println!();
    println!("2. It behaves like the value it holds");
    let boxed = Box::new(Ballot { scores: [0; 64] });
    println!("   boxed.scores.len() = {} — no explicit deref needed", boxed.scores.len());
    let unboxed: Ballot = *boxed;
    println!("   *boxed moves the value back out: {} scores", unboxed.scores.len());
    println!("   `Box<T>` implements `Deref<Target = T>`, so field access, method");
    println!("   calls and `&*b` all reach through. `*b` on its own MOVES the value");
    println!("   out and drops the box — the one operation that is not a borrow.");

    println!();
    println!("3. The reason Box exists: a type that contains itself");
    println!("   enum Round {{ Final(&str), Then(&str, Round) }}      <- E0072");
    println!("   \"recursive type `Round` has infinite size\". Each `Then` would");
    println!("   contain a whole `Round`, which contains a whole `Round`…");
    println!("   Box breaks the chain, because a pointer has a size the compiler");
    println!("   can write down before it knows what is on the other end.");
    let rounds = Round::Then("Ada", Box::new(Round::Then("Ben", Box::new(Round::Final("Cara")))));
    println!("   size_of::<Round>() = {} — one tag plus the largest variant",
             size_of::<Round>());
    println!("   eliminated(&rounds) = {:?}", eliminated(&rounds));
    println!("   winner(&rounds) = {}", winner(&rounds));

    println!();
    println!("4. And the other reason: a size known only at run time");
    let named: Box<dyn Fn(u32) -> u32> = Box::new(|n| n * 2);
    let shifted: Box<dyn Fn(u32) -> u32> = Box::new(|n| n + 100);
    println!("   two closures with different captures, one type: Box<dyn Fn>");
    println!("   named(21) = {}, shifted(21) = {}", named(21), shifted(21));
    println!("   size_of::<Box<dyn Fn(u32) -> u32>>() = {} — pointer to the value",
             size_of::<Box<dyn Fn(u32) -> u32>>());
    println!("   AND pointer to its vtable. A `dyn` box is fat.");
    println!("   So is any box over something with no size of its own: the");
    println!("   missing number rides beside the pointer, a vtable for dyn");
    println!("   and a length for a slice.");
    println!("   size_of::<Box<[u32; 64]>>() = {}  — an array's length is in its type",
             size_of::<Box<[u32; 64]>>());
    println!("   size_of::<Box<[u32]>>()     = {} — a slice's is not, so it rides along",
             size_of::<Box<[u32]>>());
    println!("   size_of::<Box<str>>()       = {} — the same shape, over UTF-8 bytes",
             size_of::<Box<str>>());

    println!();
    println!("5. What it is not");
    println!("   Box is single ownership. One owner, dropped when that owner goes");
    println!("   out of scope, moved rather than copied. For two owners you want");
    println!("   Rc; for two threads, Arc. Reaching for Box to \"put it on the heap\"");
    println!("   when nothing needs the heap just adds an allocation and an");
    println!("   indirection: a Vec, a String and a HashMap are already heap-backed.");
}
