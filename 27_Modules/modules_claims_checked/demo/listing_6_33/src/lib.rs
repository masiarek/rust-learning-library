//! Listing 6.33: the re-export path runs through `category`, which is private
//! to `product`. The `order` module is left out; it plays no part here.

mod customer;
mod product;

pub use customer::Customer;
pub use product::{category::Category, Product};
