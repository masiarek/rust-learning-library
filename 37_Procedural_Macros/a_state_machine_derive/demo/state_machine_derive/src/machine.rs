//! What the attributes declare, read by darling and then checked.

use darling::ast::Data;
use darling::util::Flag;
use darling::{Error, FromDeriveInput, FromVariant};
use syn::{DeriveInput, Ident};

/// The enum as darling reads it. `supports(enum_unit)` accepts only an enum
/// whose variants have no fields, so `take_enum` below cannot fail.
#[derive(FromDeriveInput)]
#[darling(supports(enum_unit))]
struct Input {
    ident: Ident,
    data: Data<State, ()>,
}

/// One variant and its `#[state(initial, to(A, B))]`. Both parts are optional:
/// a `Flag` is absent unless written, and `to` defaults to no transitions.
#[derive(FromVariant)]
#[darling(attributes(state))]
pub struct State {
    pub ident: Ident,
    initial: Flag,
    #[darling(default)]
    pub to: Vec<Ident>,
}

/// A machine that passed every check, so generating code from it cannot fail.
pub struct Machine {
    pub name: Ident,
    pub initial: Ident,
    pub states: Vec<State>,
}

impl Machine {
    pub fn from_input(input: &DeriveInput) -> darling::Result<Machine> {
        let Input { ident: name, data } = Input::from_derive_input(input)?;
        let states = data.take_enum().expect("supports(enum_unit) lets only enums through");
        // Collect every problem, so the user fixes them in one pass.
        let mut errors = Error::accumulator();

        // A transition to a state that does not exist: the error goes on its name.
        for target in states.iter().flat_map(|state| &state.to) {
            if !states.iter().any(|state| state.ident == *target) {
                let message = format!("`{name}` has no state named `{target}`");
                errors.push(Error::custom(message).with_span(target));
            }
        }

        // Exactly one initial state. A second `initial` is underlined itself.
        let initials: Vec<&State> = states.iter().filter(|state| state.initial.is_present()).collect();
        for extra in initials.iter().skip(1) {
            let message = format!("`{}` is already the initial state", initials[0].ident);
            errors.push(Error::custom(message).with_span(&extra.initial.span()));
        }
        let Some(initial) = initials.first().map(|state| state.ident.clone()) else {
            let message = "no initial state: mark one variant `#[state(initial)]`";
            errors.push(Error::custom(message).with_span(&name));
            return Err(Error::multiple(errors.into_inner()));
        };

        errors.finish_with(Machine { name, initial, states })
    }
}
