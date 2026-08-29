fn main() {
    let mut owned = String::from("hello world");

    if let Some(part) = owned.get_mut(6..) {
        part.make_ascii_uppercase();
    }
    println!("{owned:?}");

    // Refused for the same two reasons as get.
    let mut accented = String::from("héllo");
    println!("{:?}", accented.get_mut(0..2).is_none());
    println!("{:?}", accented.get_mut(0..99).is_none());

    // A fixed-layout record: uppercase one column, in place.
    let mut record = String::from("id=42 name=alice");
    if let Some(name) = record.get_mut(11..) {
        name.make_ascii_uppercase();
    }
    println!("{record:?}");
}
