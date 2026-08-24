//! Kata solution: three ways to make `Some(None)` compile, and the one to ship.
//!
//!   rustc --edition 2024 some_is_a_constructor_kata.rs -o /tmp/sick && /tmp/sick

/// Fixes 1 and 3 both live here. The field is singly optional, so there are
/// exactly two things you can write: `None`, or `Some(<a u8>)`.
#[derive(Debug)]
struct Person {
    name: String,
    age: Option<u8>,
}

/// Fix 2 changes the FIELD instead of the value. Now `Some(None)` is legal —
/// and it means something the other type could not say.
#[derive(Debug)]
struct Respondent {
    name: String,
    age: Option<Option<u8>>,
}

fn person_age(p: &Person) -> String {
    match p.age {
        Some(a) => format!("{a}"),
        None => "unknown".to_string(),
    }
}

fn respondent_age(r: &Respondent) -> String {
    match r.age {
        None => "not asked".to_string(),
        Some(None) => "asked, declined".to_string(),
        Some(Some(a)) => format!("{a}"),
    }
}

fn main() {
    println!("Fix 1 — delete the `Some`. The absence of a u8 is spelled `None`.");
    println!("Fix 3 — or pass a u8, which is what `Some` was asking for all along.");
    let people = [
        Person { name: "Alfredo".to_string(), age: None },
        Person { name: "Bianca".to_string(), age: Some(31) },
    ];
    for p in &people {
        println!("   {:<9} {}", p.name, person_age(p));
    }
    println!("   Two states, and no way to write a third. That is the point.");

    println!("\nFix 2 — widen the field, and `Some(None)` starts carrying a fact.");
    let respondents = [
        Respondent { name: "Alfredo".to_string(), age: None },
        Respondent { name: "Bianca".to_string(), age: Some(None) },
        Respondent { name: "Chidi".to_string(), age: Some(Some(31)) },
    ];
    for r in &respondents {
        println!("   {:<9} {}", r.name, respondent_age(r));
    }

    // The report the singly-optional type cannot produce.
    let asked = respondents.iter().filter(|r| r.age.is_some()).count();
    let answered = respondents.iter().filter(|r| r.age.flatten().is_some()).count();
    println!("   asked {asked} of {}, answered {answered}", respondents.len());
    println!("   An Option<u8> field folds 'not asked' and 'declined' into one");
    println!("   None, and no reader downstream can prise them apart again.");

    println!("\nWhich would you ship for \"we do not know Alfredo's age\"? Fix 1.");
    println!("   Fix 2 buys a distinction you were not making. It puts a third");
    println!("   state in front of every reader, and each one now owes a");
    println!("   `Some(None)` arm for a fact nobody collected. Widen the type");
    println!("   when the second question is real — not to make one line build.");
}
