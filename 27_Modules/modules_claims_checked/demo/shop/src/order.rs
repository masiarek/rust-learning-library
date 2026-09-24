use crate::customer::Customer;
use crate::product::Product;

pub struct Order {
    id: u64,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    /// The book never builds an `Order`; this constructor is what lets
    /// `main.rs` print a bill.
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
