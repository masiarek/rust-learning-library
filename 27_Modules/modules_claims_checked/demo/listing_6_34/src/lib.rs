//! Listing 6.34: `product` now re-exports `Category`, but the root line still
//! spells the path through the private `category` module.

mod customer;
mod product;

pub use customer::Customer;
pub use product::{category::Category, Product};
