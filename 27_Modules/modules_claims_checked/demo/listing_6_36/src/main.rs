//! Listing 6.36: a struct literal for a public struct whose fields are not.
//! The `use` line is the book's, `Customer` included and unused.

use pg_store_listing_6_36::{Category, Customer, Product};

fn main() {
    let product = Product {
        id: 1,
        name: String::from("Laptop"),
        price: 799.99,
        category: Category::Electronics,
    };
}
