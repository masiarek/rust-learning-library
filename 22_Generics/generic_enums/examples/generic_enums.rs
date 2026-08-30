// An enum takes type parameters exactly as a struct does.

use std::fmt::Display;

// Every variant shares the parameter list; only one variant uses it.
#[derive(Debug)]
enum Slot<T> {
    Filled(T),
    Reserved,
    Empty,
}

impl<T> Slot<T> {
    fn is_filled(&self) -> bool {
        matches!(self, Slot::Filled(_))
    }
}

// The bound goes on the impl that needs it, not on the enum.
impl<T: Display> Slot<T> {
    fn describe(&self) -> String {
        match self {
            Slot::Filled(v) => format!("filled with {v}"),
            Slot::Reserved => String::from("reserved"),
            Slot::Empty => String::from("empty"),
        }
    }
}

// Two parameters, one per variant. This is Result<T, E> with the names changed.
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn parse_score(text: &str) -> Either<u8, String> {
    match text.parse::<u8>() {
        Ok(n) if n <= 5 => Either::Left(n),
        Ok(n) => Either::Right(format!("{n} is above the 0-5 range")),
        Err(e) => Either::Right(format!("{text:?}: {e}")),
    }
}

fn main() {
    let seats = [Slot::Filled("Ada"), Slot::Reserved, Slot::Empty];
    for slot in &seats {
        println!("{:<24} filled: {}", slot.describe(), slot.is_filled());
    }
    println!();

    // A variant that carries no T still belongs to Slot<T>, so the type has
    // to come from somewhere:
    // let nothing = Slot::Empty;              // error[E0282]: type annotations needed
    let nothing = Slot::<u8>::Empty;
    println!("Slot::<u8>::Empty is {nothing:?}, filled: {}", nothing.is_filled());
    println!();

    for text in ["4", "9", "five"] {
        match parse_score(text) {
            Either::Left(score) => println!("{text:>6} -> score {score}"),
            Either::Right(why) => println!("{text:>6} -> rejected: {why}"),
        }
    }
    println!();

    // An enum is its largest variant plus a tag, so T decides the size.
    println!("size_of::<Slot<u8>>()           {}", size_of::<Slot<u8>>());
    println!("size_of::<Slot<u64>>()          {}", size_of::<Slot<u64>>());
    println!("size_of::<Either<u8, String>>() {}", size_of::<Either<u8, String>>());
    println!("size_of::<String>()             {}", size_of::<String>());
    println!();

    // Both parameters may be filled with the same type; they are still two.
    let same: Result<u8, u8> = Err(3);
    println!("Result<u8, u8> is a real type: {same:?}");
}
