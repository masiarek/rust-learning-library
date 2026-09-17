//! A program that already uses the names the derive's output relies on.

use ::state_machine::StateMachine;

/// The user's own module, named like the facade crate. In this file
/// `state_machine::` means this module, and `::state_machine::` the crate.
mod state_machine {
    pub fn whose() -> &'static str {
        "the user's module"
    }
}

/// A type named like a primitive. From here on, `str` in this module means
/// this struct (not inside `mod state_machine`, which is a module of its own).
#[allow(dead_code, non_camel_case_types)]
struct str;

#[derive(StateMachine)]
enum Order {
    #[state(initial, to(Paid))]
    Placed,
    #[state(to(Shipped))]
    Paid,
    Shipped,
}

/// Inherent items with the trait's names.
impl Order {
    const INITIAL: Order = Order::Paid;

    fn name(&self) -> String {
        format!("order ({})", StateMachine::name(self))
    }
}

fn main() {
    let order = <Order as StateMachine>::INITIAL;
    println!("state_machine::whose()            {}", state_machine::whose());
    println!("Order::INITIAL                    {}", StateMachine::name(&Order::INITIAL));
    println!("<Order as StateMachine>::INITIAL  {}", StateMachine::name(&order));
    println!("order.name()                      {}", order.name());

    let shipped = order.transition_to(Order::Paid).and_then(|paid| paid.transition_to(Order::Shipped));
    if let Err(refused) = shipped.unwrap().transition_to(Order::Placed) {
        println!("refused                           {refused}");
    }
}
