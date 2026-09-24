//! Listing 6.17: `pub enum Category` (Listing 6.15) and the `use` lines of
//! Listings 6.16 and 6.17. The first version of the store that compiles.

mod product {
    use category::Category;

    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    mod category {
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }

    impl Product {
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }
}

mod Order {
    use crate::product::Product;
    use crate::customer::Customer;

    struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 { 0.1 } else { 0.0 }
        }

        fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
