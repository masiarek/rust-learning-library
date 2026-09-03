//! Kata solution: count the drops on four paths, then remove the flag.
//!
//!   rustc --edition 2024 the_drop_flag_kata.rs -o /tmp/tdfk && /tmp/tdfk

struct Order(&'static str);

impl Drop for Order {
    fn drop(&mut self) {
        println!("       drop {}", self.0);
    }
}

fn ship_it(b: Order) {
    println!("       filed {}", b.0);
}

/// Returns early while it still owns the value.
fn early_return_owning(short: bool) {
    let b = Order("A");
    if short {
        println!("       returning early, still owning it:");
        return;
    }
    ship_it(b);
}

/// Returns early after handing the value away.
fn early_return_moved(short: bool) {
    let b = Order("B");
    if short {
        ship_it(b);
        println!("       returning early, having moved it out:");
        return;
    }
    ship_it(b);
}

/// Moves the value out on one iteration, then leaves the loop.
fn moved_in_a_loop() {
    let b = Order("C");
    for round in 0..3 {
        println!("       round {round}");
        if round == 1 {
            ship_it(b);
            break;
        }
    }
    println!("       after the loop:");
}

/// The same shape with the emptiness written down.
fn moved_in_a_loop_visibly() {
    let mut slot = Some(Order("D"));
    for round in 0..3 {
        println!("       round {round}, holding = {}", slot.is_some());
        if round == 1 {
            if let Some(b) = slot.take() {
                ship_it(b);
            }
        }
    }
    println!("       after the loop, holding = {}:", slot.is_some());
}

fn main() {
    println!("A. early return while still owning it        -> dropped AT the return");
    early_return_owning(true);

    println!("\nB. early return after moving it out          -> nothing at the return");
    early_return_moved(true);

    println!("\nC. moved out on round 1, then `break`        -> nothing at the brace");
    moved_in_a_loop();

    println!("\nD. the same, with the emptiness in the type  -> same schedule, askable");
    moved_in_a_loop_visibly();

    println!("\nFour paths, four values, four drops — one each, never two, never");
    println!("none. A and C are the ones a flag is for: the SAME closing brace");
    println!("has to drop in one execution and not in another, so the answer");
    println!("cannot be baked into the code at that brace. D is C with the");
    println!("question moved into the type, where `is_some()` can answer it.");
}
