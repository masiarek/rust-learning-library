//! `Some` is a constructor, not a flag — so `Some(None)` is a type, not a mood.
//!
//!   rustc --edition 2024 some_is_a_constructor.rs -o /tmp/sic && /tmp/sic

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    /// Two shapes, and there is no third one meaning "present but empty" —
    /// which is the shape `Some(None)` is reaching for.
    age: Option<u8>,
}

fn greet(p: &Person) -> String {
    let age = p
        .age
        .map_or("age not on file".to_string(), |a| format!("{a} years old"));
    format!("{} {}, {age}", p.first_name, p.last_name)
}

fn main() {
    println!("1. Two shapes, and neither of them is 'present but empty'");
    let people = [
        Person {
            first_name: "Alfredo".to_string(),
            last_name: "Sanchez".to_string(),
            age: None,
        },
        Person {
            first_name: "Bianca".to_string(),
            last_name: "Rossi".to_string(),
            age: Some(31),
        },
    ];
    for p in &people {
        println!("   {}", greet(p));
    }
    println!("   the field itself:  {:?}  and  {:?}", people[0].age, people[1].age);

    println!("\n2. `Some` is a function, so it has a type you can write down");
    let wrap: fn(u8) -> Option<u8> = Some;
    println!("   let wrap: fn(u8) -> Option<u8> = Some;");
    println!("   wrap(31)                  -> {:?}", wrap(31));
    let wrapped: Vec<Option<u8>> = [31u8, 44, 7].into_iter().map(Some).collect();
    println!("   [31, 44, 7].map(Some)     -> {wrapped:?}");
    println!("   So `Some(None)` is a call with the wrong argument type. The");
    println!("   compiler says `expected u8, found Option<_>` because that is");
    println!("   literally what happened: you passed an Option to fn(u8).");

    println!("\n3. Where `Some(None)` is exactly right: two questions, not one");
    let rows: [(&str, Option<Option<u8>>); 3] = [
        ("never asked", None),
        ("asked, declined", Some(None)),
        ("asked, answered", Some(Some(31))),
    ];
    for (label, cell) in rows {
        // Pad the rendered string, not the value: a width on `{:?}` is handed
        // to the *inner* type, so `{:<14?}` on Some(Some(31)) pads the 31.
        let shown = format!("{cell:?}");
        println!("   {label:<16} {shown:<14}  flatten -> {:?}", cell.flatten());
    }
    println!("   `.flatten()` collapses the two absences back into one, which is");
    println!("   the right move only once you no longer need to tell them apart.");
}
