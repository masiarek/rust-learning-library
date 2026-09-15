//! Checkpoint answers for the `ToOwned` learning path, one section per step.
//! Predict each line before you read it.
//!
//!   rustc --edition 2024 how_to_learn_to_owned.rs -o /tmp/hlto && /tmp/hlto

use std::any::type_name_of_val;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

/// Deliberately not `Clone`: steps 3 and 5 are about what a reference to it
/// can still do.
struct Ticket {
    seat: u32,
}

/// Step 10's pair: a view that derives `Clone`, and its owned twin.
#[derive(Clone)]
struct NameRef<'a> {
    first: &'a str,
}

struct Name {
    first: String,
}

impl NameRef<'_> {
    /// An inherent method named like the trait's.
    fn to_owned(&self) -> Name {
        Name { first: self.first.to_owned() }
    }
}

fn main() {
    println!("Step 1. Is &str the same size as &String?");
    println!("   size_of::<&str>() == size_of::<&String>():     {}", size_of::<&str>() == size_of::<&String>());
    println!("   size_of::<&str>() == 2 * size_of::<&String>(): {}   (address + length)", size_of::<&str>() == 2 * size_of::<&String>());

    println!();
    println!("Step 2. For s: String and v: Vec<i32>, what types are *s and *v?");
    let s = String::from("Ada");
    let v = vec![1_i32, 2, 3];
    println!("   *s : {}", type_name_of_val(&*s));
    println!("   *v : {}", type_name_of_val(&*v));

    println!();
    println!("Step 3. Ticket is not Clone. After `let b = a;` is `a` still usable?");
    let ticket = Ticket { seat: 12 };
    let a = &ticket;
    let b = a;
    println!("   yes: a.seat = {}, b.seat = {}   (&Ticket is Copy)", a.seat, b.seat);

    println!();
    println!("Step 4. What type does each clone return?");
    let name: &str = "Adam";
    let r: &String = &s;
    // rustc warns on this one (`noop_method_call`): cloning a &str copies the
    // reference. The warning is the lesson, so it is silenced here, not fixed.
    #[allow(noop_method_call)]
    let from_str = name.clone();
    println!("   name.clone()      name: &str    -> {}", type_name_of_val(&from_str));
    println!("   r.clone()         r: &String    -> {}", type_name_of_val(&r.clone()));
    println!("   Clone::clone(&r)                -> {}", type_name_of_val(&Clone::clone(&r)));

    println!();
    println!("Step 5. What is 42_i32.to_owned()? Is (&ticket).to_owned() a new ticket?");
    let n = 42_i32.to_owned();
    println!("   42_i32.to_owned() -> {} {n}", type_name_of_val(&n));
    let same = a.to_owned();
    println!("   a.to_owned() is the same address as a: {}   (it is a &Ticket)", std::ptr::eq(a, same));

    println!();
    println!("Step 6. What does each .to_owned() turn into?");
    println!("   \"hi\".to_owned()                   -> {}", type_name_of_val(&"hi".to_owned()));
    println!("   [1_i32, 2][..].to_owned()         -> {}", type_name_of_val(&[1_i32, 2][..].to_owned()));
    println!("   Path::new(\"notes.txt\").to_owned() -> {}", type_name_of_val(&Path::new("notes.txt").to_owned()));

    println!();
    println!("Step 7. Strong count after Rc::to_owned? Variant after Cow::to_owned?");
    let shared = Rc::new(String::from("ballot"));
    let again = shared.to_owned();
    println!("   Rc::strong_count = {}, same allocation: {}", Rc::strong_count(&shared), Rc::ptr_eq(&shared, &again));
    let cow: Cow<'_, str> = Cow::Borrowed("ballot");
    let variant = match cow.to_owned() {
        Cow::Borrowed(_) => "Cow::Borrowed",
        Cow::Owned(_) => "Cow::Owned",
    };
    println!("   Cow::Borrowed(..).to_owned() -> {variant}");

    println!();
    println!("Step 8. Does seats.get(\"Ada\") compile on a HashMap<String, u32>?");
    let mut seats: HashMap<String, u32> = HashMap::new();
    seats.insert(String::from("Ada"), 3);
    println!("   yes, because String: Borrow<str> -> {:?}", seats.get("Ada"));

    println!();
    println!("Step 9. Does clone_into change a roomy buffer's capacity?");
    let mut buf = String::with_capacity(64);
    let before = buf.capacity();
    "reuse me".clone_into(&mut buf);
    println!("   capacity unchanged: {}, buf = {buf:?}", buf.capacity() == before);

    println!();
    println!("Step 10. NameRef derives Clone and has an inherent to_owned. What comes back?");
    let owner = String::from("Ada");
    let view = NameRef { first: &owner };
    let mine: Name = view.to_owned();
    let blanket: NameRef<'_> = ToOwned::to_owned(&view);
    println!("   view.to_owned()          -> Name    {{ first: {:?} }}   the inherent method", mine.first);
    println!("   ToOwned::to_owned(&view) -> NameRef {{ first: {:?} }}   the blanket impl", blanket.first);
}
