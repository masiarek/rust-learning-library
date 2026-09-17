use door::Door;
use state_machine::StateMachine;

fn main() {
    for state in [Door::Open, Door::Closed, Door::Locked] {
        let next: Vec<&str> = state.next_states().iter().map(|next| next.name()).collect();
        println!("{:<6} -> {}", state.name(), next.join(", "));
    }

    let mut door = Door::INITIAL;
    println!("\nstart: {}", door.name());
    // Locked -> Open is the one move below that no transition declares.
    for next in [Door::Closed, Door::Locked, Door::Open, Door::Closed, Door::Open] {
        door = match door.transition_to(next) {
            Ok(state) => {
                println!("  now: {}", state.name());
                state
            }
            Err(refused) => {
                println!("  refused: {refused}");
                refused.from
            }
        };
    }
}
