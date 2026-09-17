use state_machine::StateMachine;

/// A door that can be locked only while closed.
#[derive(StateMachine)]
pub enum Door {
    #[state(initial, to(Closed))]
    Open,
    #[state(to(Open, Locked))]
    Closed,
    #[state(to(Closed))]
    Locked,
}
