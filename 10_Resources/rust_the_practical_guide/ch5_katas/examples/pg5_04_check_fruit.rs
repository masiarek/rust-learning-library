//! Exercise 4 (§5.7): is the fruit in the basket?
//!
//! The fix is one line, `None` after the loop. Then the same search as
//! clippy's `manual_find` suggests it, with the parameter as `&str`.
//!
//!   rustc --edition 2024 pg5_04_check_fruit.rs -o /tmp/pg504 && /tmp/pg504

/// The book's function with the missing `None`. Kept as a loop on purpose:
/// clippy's `manual_find` would rewrite it into `find_fruit` below.
#[allow(clippy::manual_find)]
fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}

/// The same search as an iterator: `find` returns the first match or `None`,
/// so the fall-through case cannot be left out. `&str` accepts a literal too.
fn find_fruit(input_fruit: &str) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    fruit_basket.into_iter().find(|fruit| fruit == input_fruit)
}

fn main() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("Fruit found: {fruit}");
    }

    for wanted in ["banana", "kiwi"] {
        match find_fruit(wanted) {
            Some(fruit) => println!("Fruit found: {fruit}"),
            None => println!("No {wanted} in the basket"),
        }
    }
}
