// Exercise 2.8: a type alias gives the tuple's shape a name. It is an item, so
// it may sit inside main, and it makes no new type: the name expands to the tuple.
use std::any::type_name;

fn main() {
    type Book = (String, String, u32);
    let book1: Book = (
        String::from("Rust Programming Language"),
        String::from("RUST Community"),
        2010,
    );
    println!("Book name: {}, Author: {}, Year {}", book1.0, book1.1, book1.2); // Book name: Rust Programming Language, Author: RUST Community, Year 2010
    println!("Book is {}", type_name::<Book>()); // Book is (alloc::string::String, alloc::string::String, u32)
    let plain: (String, String, u32) = book1; // no conversion: same type
    println!("year of the plain tuple: {}", plain.2); // year of the plain tuple: 2010
}
