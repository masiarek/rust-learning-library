//! `todo!()` compiles in place of any body and panics when it runs.
//! The page is 28_Testing/a_first_test_step_by_step/a_stub_that_panics/README.md.
//!
//!   rustc --edition 2024 a_stub_that_panics.rs -o /tmp/t && /tmp/t

use std::panic;

// The stub from the chapter's second run: a real signature, no body yet.
// `_csv` starts with an underscore, so an unused parameter draws no warning.
fn parse_rows(_csv: String) -> Vec<Vec<f32>> {
    todo!()
}

// todo!() has type `!`, which fits wherever any type is expected.
fn a_count() -> usize {
    todo!("count the rows")
}

fn a_label() -> String {
    unimplemented!()
}

/// Runs `f`, catches its panic, and returns the panic message.
fn message_of<T>(f: impl FnOnce() -> T + panic::UnwindSafe) -> String {
    match panic::catch_unwind(f) {
        Ok(_) => "no panic".to_string(),
        Err(payload) => {
            if let Some(text) = payload.downcast_ref::<&str>() {
                text.to_string()
            } else if let Some(text) = payload.downcast_ref::<String>() {
                text.clone()
            } else {
                "a panic with a payload that is not text".to_string()
            }
        }
    }
}

fn main() {
    // Silence the default hook, which would print each panic to stderr.
    panic::set_hook(Box::new(|_| {}));

    println!("Each stub compiles, and each one panics when it is called:");
    println!("  parse_rows(..)  -> {:?}", message_of(|| parse_rows(String::from("1,1,1\n"))));
    println!("  a_count()       -> {:?}", message_of(a_count));
    println!("  a_label()       -> {:?}", message_of(a_label));

    let _ = panic::take_hook();
}
