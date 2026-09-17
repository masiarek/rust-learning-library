use state_machine::StateMachine;

#[derive(StateMachine)]
enum Door {
    #[state(initial, to(Clsoed))]
    Open,
    #[state(to(Open, Lockd))]
    Closed,
    #[state(to(Closed))]
    Locked,
}

fn main() {}
