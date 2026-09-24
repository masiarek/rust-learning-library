//! Kata solution: five `pub` decisions for the store, and what each one
//! refuses when it is missing.
//!
//!   rustc --edition 2024 pg_mod_kata.rs -o /tmp/pgk && /tmp/pgk

mod shop {
    mod product {
        mod category {
            #[derive(Debug)]
            pub enum Category {
                Electronics,
                Clothing,
                Books,
            }
        }

        pub use category::Category; // decision 5: through the private module

        pub struct Product { // decision 2: the type, not the fields
            id: u64,
            name: String,
            price: f64,
            category: Category,
        }

        impl Product {
            pub fn new(id: u64, name: String, price: f64, category: Category) -> Self { // decision 3
                Self { id, name, price, category }
            }

            fn calculate_tax(&self) -> f64 {
                self.price * 0.1
            }

            pub fn product_price(&self) -> f64 { // decision 4
                self.price + self.calculate_tax()
            }

            pub fn label(&self) -> String {
                format!("{} ({:?}), product #{}", self.name, self.category, self.id)
            }
        }
    }

    mod customer {
        pub struct Customer {
            id: u64,
            name: String,
        }

        impl Customer {
            pub fn new(id: u64, name: String) -> Self {
                Self { id, name }
            }

            pub fn label(&self) -> String {
                format!("{}, customer #{}", self.name, self.id)
            }
        }
    }

    mod order {
        use super::customer::Customer;
        use super::product::Product;

        pub struct Order {
            product: Product,
            customer: Customer,
            quantity: u32,
        }

        impl Order {
            pub fn new(product: Product, customer: Customer, quantity: u32) -> Self {
                Self { product, customer, quantity }
            }

            fn calculate_discount(&self) -> f64 {
                if self.quantity > 5 { 0.1 } else { 0.0 }
            }

            pub fn total_bill(&self) -> f64 {
                let before = self.product.product_price() * self.quantity as f64;
                before - before * self.calculate_discount()
            }

            pub fn bill(&self) -> String {
                format!("{} x {} for {} -> {:.2}", self.quantity, self.product.label(), self.customer.label(), self.total_bill())
            }
        }
    }

    // decision 1: re-export, and keep the three modules private
    pub use customer::Customer;
    pub use order::Order;
    pub use product::{Category, Product};
}

use shop::{Category, Customer, Order, Product};

fn main() {
    println!("1. The store, used only through what `shop` re-exports");
    let product = Product::new(7, String::from("Novel"), 20.0, Category::Books);
    let customer = Customer::new(3, String::from("Alice"));
    let order = Order::new(product, customer, 6);
    println!("   {}", order.bill());

    println!();
    println!("2. The five decisions, and what rustc says when each is missing");
    println!("   1. `pub use` at shop for Product, Category, Customer, Order; the modules stay private");
    println!("      missing: `use shop::Product` -> E0432 unresolved import, or through");
    println!("      `shop::product::Product` -> E0603: module `product` is private");
    println!("   2. `pub struct Product` (and Customer, Order); the fields stay private");
    println!("      missing: E0603: struct `Product` is private");
    println!("      fields written from main as a literal: E0451: field `id` of struct `Product` is private");
    println!("   3. `pub fn new` on each type: the only way in, since the fields are private");
    println!("      missing: E0624: associated function `new` is private");
    println!("   4. `pub fn product_price`, `pub fn total_bill`, `pub fn label`; calculate_tax and");
    println!("      calculate_discount stay private, since only their own module calls them");
    println!("      missing: E0624: method `product_price` is private");
    println!("   5. `pub use category::Category` inside product, so `Category` is reachable without");
    println!("      `pub mod category`; `pub enum` already published all three variants");
    println!("      missing, with `pub use product::{{category::Category, Product}}` at shop:");
    println!("      E0603: module `category` is private");

    println!();
    println!("3. What stayed private");
    println!("   the three modules, every field, calculate_tax, calculate_discount, and the");
    println!("   category module: {} pub keywords in the tree, none on a mod", 15);
}
