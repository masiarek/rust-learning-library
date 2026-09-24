//! What *Rust: The Practical Guide* chapter 6 says about modules, run as one
//! file: the store's module tree in the shape of Listing 6.17, with `main`
//! standing where `src/main.rs` would. The same tree as files is in demo/.
//!
//!   rustc --edition 2024 pg_mod_claims.rs -o /tmp/pgm && /tmp/pgm

mod product {
    mod category {
        #[derive(Debug)]
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }

        /// A child reading its parent's private constant through `super::`.
        pub fn tax_rate_seen_from_the_child() -> f64 {
            super::TAX_RATE
        }

        /// `pub(super)`: visible to `product` and its descendants, not to the
        /// crate root.
        pub(super) fn only_for_product() -> &'static str {
            "pub(super)          reachable from product; E0603 from the crate root"
        }

        /// `pub(in crate::product)` names the same reach as `pub(super)` here,
        /// by the ancestor's path.
        pub(in crate::product) fn only_for_product_by_path() -> &'static str {
            "pub(in crate::product) the same reach, by the ancestor's name"
        }
    }

    pub use category::Category;

    const TAX_RATE: f64 = 0.1;

    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    impl Product {
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Self {
            Self { id, name, price, category }
        }

        fn calculate_tax(&self) -> f64 {
            self.price * TAX_RATE
        }

        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }

        pub fn label(&self) -> String {
            format!("{} ({:?}), product #{}", self.name, self.category, self.id)
        }
    }

    /// `pub(self)` written out: the same reach as no keyword at all.
    pub(self) fn private_by_two_spellings() -> &'static str {
        "pub(self)           the same reach as no keyword: this module and its descendants"
    }

    /// `pub(crate)`: anywhere in this crate, nowhere outside it.
    pub(crate) fn crate_wide() -> &'static str {
        "pub(crate)          every module of this crate, nothing outside it"
    }

    pub fn tax_rate_via_the_child() -> f64 {
        category::tax_rate_seen_from_the_child()
    }

    pub fn reached_from_product() -> [&'static str; 3] {
        [
            private_by_two_spellings(),
            category::only_for_product(),
            category::only_for_product_by_path(),
        ]
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Self {
            Self { id, name, email }
        }

        pub fn label(&self) -> String {
            format!("{} <{}>, customer #{}", self.name, self.email, self.id)
        }
    }
}

mod order {
    // Listing 6.17 writes `crate::product::Product`. A relative path through
    // `super::` names the same item; the module tree does not care which.
    use super::customer::Customer;
    use super::product::Product;

    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Self {
            Self { id, product, customer, quantity }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 { 0.1 } else { 0.0 }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }

        pub fn bill(&self) -> String {
            format!(
                "order #{}: {} x {:.2} for {}, {:.0}% off -> {:.2}",
                self.id,
                self.quantity,
                self.product.product_price(),
                self.customer.label(),
                self.calculate_discount() * 100.0,
                self.total_bill(),
            )
        }
    }
}

/// Listing 6.9, as printed: a relative path with no prefix, and one with `self::`.
mod utilities {
    pub mod math {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }

    pub fn calculate() {
        let result = math::multiply(3, 4);
        println!("   Multiplication result: {}", result);
        let result_self = self::math::multiply(5, 6);
        println!("   Result using `self`: {}", result_self);
    }
}

use product::{Category, Product};

fn main() {
    println!("1. `crate` is the root module: an absolute path and a relative one name one item");
    let absolute = crate::product::Product::new(1, String::from("Laptop"), 799.99, crate::product::Category::Electronics);
    let relative = product::Product::new(1, String::from("Laptop"), 799.99, product::Category::Electronics);
    println!("   crate::product::Product::new(..).label() = {}", absolute.label());
    println!("   product::Product::new(..).label()        = {}", relative.label());

    println!();
    println!("2. Listing 6.9, run");
    utilities::calculate();

    println!();
    println!("3. \"you must specify their absolute paths\": `order` above uses super:: and builds");
    let customer = customer::Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
    let order = order::Order::new(1, absolute, customer, 6);
    println!("   {}", order.bill());
    println!("   the same numbers as `cargo run` on the file layout in demo/");

    println!();
    println!("4. Private means this module and its descendants");
    println!("   product::category reads product's private TAX_RATE through super:: -> {}", product::tax_rate_via_the_child());
    println!("   the root reaching a child's pub item, Category::Books -> {:?}", Category::Books);
    println!("   \"parent modules cannot access the items within their child modules\" is");
    println!("   true of the child's private items, and not of its pub ones");

    println!();
    println!("5. Four narrower forms of pub, each measured from where it is written");
    for line in product::reached_from_product() {
        println!("   {line}");
    }
    println!("   {}", product::crate_wide());
    println!("   product::private_by_two_spellings() from here -> E0603: function is private");

    println!();
    println!("6. `use` binds a name; the item was already reachable");
    let with_use = Product::new(2, String::from("Novel"), 20.0, Category::Books);
    let without_use = product::Product::new(2, String::from("Novel"), 20.0, product::Category::Books);
    println!("   Product::new(..).product_price()          = {:.2}   <- through `use product::Product`", with_use.product_price());
    println!("   product::Product::new(..).product_price() = {:.2}   <- the full path, no `use` needed", without_use.product_price());

    println!();
    println!("7. `pub enum` publishes every variant; `pub` on one variant is E0449");
    println!("   Category::Clothing named from the root -> {:?}", Category::Clothing);

    println!();
    println!("8. `pub struct` publishes the type, not the fields");
    println!("   Product::new(..).label() = {}   <- the constructor and a method are the doors", with_use.label());
    println!("   Product {{ id: 1, .. }} written here -> E0451: field `id` of struct `Product` is private");
}
