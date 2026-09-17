//! `#[derive(StateMachine)]`: read each variant's `#[state(…)]`, check the
//! machine it describes, and implement `state_machine::StateMachine` for it.

mod machine;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use machine::Machine;

#[proc_macro_derive(StateMachine, attributes(state))]
pub fn derive_state_machine(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match Machine::from_input(&input) {
        Ok(machine) => generate(&machine).into(),
        // One `compile_error!` per problem, each carrying its own span.
        Err(errors) => errors.write_errors().into(),
    }
}

/// The trait impl. It reaches the trait and `str` by absolute paths, adds
/// nothing but the trait's own items, and binds no variables: every other name
/// in it is the user's enum, its variants, or a keyword.
fn generate(machine: &Machine) -> proc_macro2::TokenStream {
    let name = &machine.name;
    let initial = &machine.initial;
    let states = machine.states.iter().map(|state| &state.ident);
    let labels = machine.states.iter().map(|state| state.ident.to_string());
    let from = machine.states.iter().map(|state| &state.ident);
    let targets = machine.states.iter().map(|state| &state.to);

    quote! {
        impl ::state_machine::StateMachine for #name {
            const INITIAL: Self = Self::#initial;

            fn name(&self) -> &'static ::core::primitive::str {
                match self {
                    #(Self::#states => #labels,)*
                }
            }

            fn next_states(&self) -> &'static [Self] {
                match self {
                    #(Self::#from => &[#(Self::#targets),*],)*
                }
            }
        }
    }
}
