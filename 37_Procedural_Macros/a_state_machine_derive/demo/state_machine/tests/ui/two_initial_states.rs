use state_machine::StateMachine;

#[derive(StateMachine)]
enum Door {
    #[state(initial, to(Closed))]
    Open,
    #[state(initial, to(Open))]
    Closed,
}

fn main() {}
