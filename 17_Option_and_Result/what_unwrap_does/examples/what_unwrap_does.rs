//! `unwrap` takes the value out of an `Ok` or a `Some`, and panics on
//! anything else. Every panic below is caught so the program can keep going.
//! The page is 17_Option_and_Result/what_unwrap_does/README.md.
//!
//!   rustc --edition 2024 what_unwrap_does.rs -o /tmp/t && /tmp/t

use std::panic;

/// Runs `f`, catches a panic, and describes what happened.
fn outcome<T: std::fmt::Debug>(f: impl FnOnce() -> T + panic::UnwindSafe) -> String {
    match panic::catch_unwind(f) {
        Ok(value) => format!("returned {value:?}"),
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            format!("panicked: {message}")
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));

    println!("1. On a Result");
    println!("   \"1.5\".parse::<f32>().unwrap() {}", outcome(|| "1.5".parse::<f32>().unwrap()));
    println!("   \"x\".parse::<f32>().unwrap()   {}", outcome(|| "x".parse::<f32>().unwrap()));

    println!();
    println!("2. On an Option");
    let row = vec![1.0_f32, -1.0, 1.0];
    println!("   row.get(2).unwrap()  {}", outcome(|| *row.get(2).unwrap()));
    println!("   row.get(9).unwrap()  {}", outcome(|| *row.get(9).unwrap()));

    println!();
    println!("3. The same Err through the methods beside unwrap");
    println!("   .expect(\"a CSV field\")   {}", outcome(|| "x".parse::<f32>().expect("a CSV field")));
    println!("   .unwrap_or(0.0)          {}", outcome(|| "x".parse::<f32>().unwrap_or(0.0)));
    println!("   .unwrap_or_default()     {}", outcome(|| "x".parse::<f32>().unwrap_or_default()));
    println!("   .ok()                    {}", outcome(|| "x".parse::<f32>().ok()));

    println!();
    println!("4. The message uses the error's Debug form, not its Display form");
    let error = "x".parse::<f32>().unwrap_err();
    println!("   Debug:   {error:?}");
    println!("   Display: {error}");

    let _ = panic::take_hook();
}
