//! The crate users depend on. It holds the trait the derive implements, the
//! error type, and the derive itself, re-exported under the trait's name.

use std::{fmt, mem};

pub use state_machine_derive::StateMachine;

/// An enum whose variants are states. The derive writes the first three items;
/// the last two are written once, here, for every machine.
pub trait StateMachine: Sized + 'static {
    /// The state a new machine starts in.
    const INITIAL: Self;

    /// The variant's name, for messages.
    fn name(&self) -> &'static str;

    /// The states a declared transition leads to from this one.
    fn next_states(&self) -> &'static [Self];

    /// Whether a declared transition leads from this state to `next`.
    fn can_transition_to(&self, next: &Self) -> bool {
        let next = mem::discriminant(next);
        self.next_states().iter().any(|state| mem::discriminant(state) == next)
    }

    /// Moves to `next`, or hands both states back if no transition allows it.
    fn transition_to(self, next: Self) -> Result<Self, InvalidTransition<Self>> {
        if self.can_transition_to(&next) {
            Ok(next)
        } else {
            Err(InvalidTransition { from: self, to: next })
        }
    }
}

/// One error type for every machine, generic over the machine, so the derive
/// never has to invent a type name in the user's module.
pub struct InvalidTransition<S> {
    pub from: S,
    pub to: S,
}

impl<S: StateMachine> fmt::Display for InvalidTransition<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no transition from {} to {}", self.from.name(), self.to.name())
    }
}

impl<S: StateMachine> fmt::Debug for InvalidTransition<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvalidTransition")
            .field("from", &self.from.name())
            .field("to", &self.to.name())
            .finish()
    }
}

impl<S: StateMachine> std::error::Error for InvalidTransition<S> {}
