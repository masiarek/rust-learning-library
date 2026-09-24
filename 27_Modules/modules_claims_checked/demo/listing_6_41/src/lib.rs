//! Listing 6.35: the re-export the book settles on. `#![allow(dead_code)]`
//! is not in the book; it keeps the answer key to the error under test, since
//! nothing in the book ever reads `id`, `name` or `category`.

#![allow(dead_code)]

mod customer;
mod product;

pub use customer::Customer;
pub use product::Product;
pub use crate::product::Category;
