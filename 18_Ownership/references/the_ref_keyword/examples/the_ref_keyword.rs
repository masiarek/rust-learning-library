//! `ref` in a pattern: bind by borrowing instead of moving — and never a change in what matches.
//!
//!   rustc --edition 2024 the_ref_keyword.rs -o /tmp/the_ref_keyword && /tmp/the_ref_keyword

use std::any::type_name_of_val;

struct Order {
    id: String,
    note: String,
}

/// `Some(x)` on an `Option<i32>`: `x` is a copy of the payload.
fn plain(o: Option<i32>) -> String {
    match o {
        Some(x) => format!("x: {} = {x}", type_name_of_val(&x)),
        None => String::from("no match"),
    }
}

/// `Some(ref x)`: the same values match, and `x` points at the payload.
fn with_ref(o: Option<i32>) -> String {
    match o {
        Some(ref x) => format!("x: {} = {x}", type_name_of_val(&x)),
        None => String::from("no match"),
    }
}

/// `Some(x)` on an `Option<&i32>`: `x` is the reference itself.
fn plain_on_refs(o: Option<&i32>) -> String {
    match o {
        Some(x) => format!("x: {} = {x}", type_name_of_val(&x)),
        None => String::from("no match"),
    }
}

/// `Some(&x)`: fits only because the payload is a reference, and strips it off.
fn with_amp(o: Option<&i32>) -> String {
    match o {
        Some(&x) => format!("x: {} = {x}", type_name_of_val(&x)),
        None => String::from("no match"),
    }
}

fn main() {
    println!("1. std's own example: `Some(ref n)` borrows, so `maybe_name` outlives the match");
    let maybe_name = Some(String::from("Alice"));
    match maybe_name {
        Some(ref n) => println!("   Hello, {n}"),
        _ => println!("   Hello, world"),
    }
    println!("   Hello again, {}", maybe_name.unwrap_or("world".into()));

    println!();
    println!("2. `ref` changes the binding's type, never which values match");
    println!("   {:<10} {:<17} {}", "value", "Some(x)", "Some(ref x)");
    for (label, o) in [("Some(3)", Some(3)), ("None", None)] {
        println!("   {:<10} {:<17} {}", label, plain(o), with_ref(o));
    }

    println!();
    println!("3. `&` is the opposite: part of what matches, and it removes a reference");
    let three = 3;
    println!("   {:<10} {:<17} {}", "value", "Some(x)", "Some(&x)");
    for (label, o) in [("Some(&3)", Some(&three)), ("None", None)] {
        println!("   {:<10} {:<17} {}", label, plain_on_refs(o), with_amp(o));
    }

    println!();
    println!("4. `let ref z = x;` is `let z = &x;`");
    let x: u32 = 12;
    let ref z = x;
    let z2 = &x;
    println!("   let ref z = x;   z:  {} = {z}", type_name_of_val(&z));
    println!("   let z2 = &x;     z2: {} = {z2}", type_name_of_val(&z2));
    println!("   same address: {}", std::ptr::eq(z, z2));

    println!();
    println!("5. `_` binds nothing, so it moves nothing");
    let maybe_name = Some(String::from("Alice"));
    match maybe_name {
        Some(_) => println!("   Some(_) matched"),
        None => println!("   None matched"),
    }
    println!("   maybe_name afterwards: {maybe_name:?}");

    println!();
    println!("6. One pattern, two modes: move `id`, borrow `note`");
    let order = Order { id: String::from("A-17"), note: String::from("fragile") };
    let Order { id, ref note } = order;
    println!("   id:   {} = {id}", type_name_of_val(&id));
    println!("   note: {} = {note}", type_name_of_val(&note));
    println!("   order.note is still there: {}", order.note);

    println!();
    println!("7. `ref mut` writes through the binding, into the place it matched");
    let mut tally = Some(42);
    if let Some(ref mut n) = tally {
        *n += 1;
    }
    println!("   Some(ref mut n) = tally       -> tally = {tally:?}");
    if let Some(n) = &mut tally {
        *n += 1;
    }
    println!("   Some(n) = &mut tally          -> tally = {tally:?}");

    println!();
    println!("8. `ref` on both sides of `@`: the whole and a part, neither moved");
    let maybe_name = Some(String::from("Alice"));
    if let ref whole @ Some(ref inner) = maybe_name {
        println!("   whole: {} = {whole:?}", type_name_of_val(&whole));
        println!("   inner: {} = {inner}", type_name_of_val(&inner));
    }
    println!("   maybe_name afterwards: {maybe_name:?}");

    println!();
    println!("Checkpoint. After `let (count, ref label) = pair;` is `pair` still usable?");
    let pair = (5, String::from("hi"));
    let (count, ref label) = pair;
    println!("   count: {} = {count}", type_name_of_val(&count));
    println!("   label: {} = {label}", type_name_of_val(&label));
    println!("   pair afterwards: {pair:?}");
}
