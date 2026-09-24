//! The binary crate of the same package. It sees the library only through the
//! library's public surface, by the package's name, never through `crate::`.

use pg_store_shop::{Category, Customer, Order, Product};

fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
    println!("{}", product.label());
    println!("price with tax: {:.2}", product.product_price());
    let order = Order::new(1, product, customer, 6);
    println!("{}", order.bill());
}
