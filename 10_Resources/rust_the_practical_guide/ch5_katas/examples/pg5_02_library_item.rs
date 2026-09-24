//! Exercise 2 (§5.7): a library item, and a function that describes it.
//!
//! The field is `item_type`, as the task names it, because `type` is a
//! keyword. The function borrows the item, so the caller keeps it.
//!
//!   rustc --edition 2024 pg5_02_library_item.rs -o /tmp/pg502 && /tmp/pg502

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

#[derive(Debug)]
struct Item {
    id: u32,
    title: String,
    year: u16,
    item_type: ItemType,
}

/// Prints the four fields, and says in words which kind of item this is.
fn display_item_info(item: &Item) {
    let kind = match item.item_type {
        ItemType::Book => "book",
        ItemType::Magazine => "magazine",
    };
    println!("ID: {}", item.id);
    println!("Title: {}", item.title);
    println!("Year: {}", item.year);
    println!("This item is a {kind}");
}

fn main() {
    let rust_book = Item {
        id: 1,
        title: String::from("The Rust Programming Language Book"),
        year: 2021,
        item_type: ItemType::Book,
    };
    let rust_magazine = Item {
        id: 2,
        title: String::from("Rust Magazine"),
        year: 2022,
        item_type: ItemType::Magazine,
    };
    display_item_info(&rust_book);
    display_item_info(&rust_magazine);

    // `&Item` left both items with their owners, so they can still be used.
    println!();
    println!("{{}} on the title:   {}", rust_book.title);
    println!("{{:?}} on the title: {:?}", rust_book.title);
    println!("{{:?}} on the type:  {:?}", rust_book.item_type);
    println!("the whole record:  {rust_magazine:?}");
}
