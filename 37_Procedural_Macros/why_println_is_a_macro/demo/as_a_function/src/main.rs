//! `format!` written as an ordinary function. It compiles, and each mistake the
//! macro would refuse is found only when the line runs, if at all.

use std::fmt::Display;

/// Replaces each `{}` in `template` with the next argument.
///
/// The template is a `&str` like any other, so it is read while the program
/// runs. The arguments have to share one type, so every one is a `&dyn Display`.
fn format_function(template: &str, args: &[&dyn Display]) -> Result<String, String> {
    let holes = template.matches("{}").count();
    if holes != args.len() {
        return Err(format!("expected {holes} arguments, got {}", args.len()));
    }
    let mut out = String::new();
    let mut rest = template;
    for arg in args {
        let (before, after) = rest.split_once("{}").expect("counted above");
        out.push_str(before);
        out.push_str(&arg.to_string());
        rest = after;
    }
    out.push_str(rest);
    Ok(out)
}

fn main() {
    let name = "Ada";
    let items = 3;
    let right = format_function("{} has {} items in the cart", &[&name, &items]);
    let one_short = format_function("{} has {} items in the cart", &[&name]);
    let by_name = format_function("{name} has {items} items in the cart", &[]);
    println!("{right:?}");
    println!("{one_short:?}");
    println!("{by_name:?}");
}
