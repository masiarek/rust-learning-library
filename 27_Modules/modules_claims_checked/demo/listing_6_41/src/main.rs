//! Listing 6.41, with the package's real name where the book wrote
//! `my_package_book`. Listing 6.40's `Customer::new` has no `pub`.

use pg_store_listing_6_41::{Category, Customer, Product};

fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
}
