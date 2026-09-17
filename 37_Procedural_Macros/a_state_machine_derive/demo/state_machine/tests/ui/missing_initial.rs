use state_machine::StateMachine;

#[derive(StateMachine)]
enum Door {
    #[state(to(Closed))]
    Open,
    #[state(to(Open))]
    Closed,
}

fn main() {}
