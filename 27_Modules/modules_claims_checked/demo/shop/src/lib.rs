//! The chapter's online store in the layout of Listing 6.29: one file per
//! module, and `product.rs` beside a `product/` folder for its submodule.
//! Nothing here is `pub mod`; the three `pub use` lines are the whole public
//! surface, so a user writes `pg_store_shop::Product` and never sees `product`.

mod customer;
mod order;
mod product;

pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};
