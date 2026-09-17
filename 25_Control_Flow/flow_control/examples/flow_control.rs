//! Flow control: every construct is an expression, and every way out of one
//! is a jump of a known distance.
//!
//!   rustc --edition 2024 flow_control.rs -o /tmp/fc && /tmp/fc

use std::any::type_name_of_val;
use std::fmt::Debug;

/// Print what a construct evaluated to, and the type the compiler gave it.
fn show<T: Debug>(construct: &str, value: T) {
    let ty = type_name_of_val(&value).replace("core::option::", "");
    println!("  {construct:<22} {ty:<12} {value:?}");
}

/// `return` leaves the loop AND the function, in one step.
fn first_even(readings: &[i32]) -> Option<i32> {
    for &r in readings {
        if r % 2 == 0 {
            return Some(r);
        }
    }
    None
}

/// `?` is a `return` too: the first unparsable text leaves the function.
fn total(texts: &[&str]) -> Result<i32, String> {
    let mut sum = 0;
    for t in texts {
        sum += t.parse::<i32>().map_err(|e| format!("{t:?}: {e}"))?;
    }
    Ok(sum)
}

fn main() {
    let n = 7;
    let readings = [3, 8, 5, 12];

    println!("1. What each construct evaluates to");
    let size = if n < 10 { "small" } else { "large" };
    show("if … else", size);

    let parity = match n % 2 {
        0 => "even",
        _ => "odd",
    };
    show("match", parity);

    let mut sum = 0;
    let for_value = for r in readings {
        sum += r;
    };
    show("for", for_value);

    let mut i = 0;
    let while_value = while i < 3 {
        i += 1;
    };
    show("while", while_value);

    let mut tries = 0;
    let loop_value = loop {
        tries += 1;
        if tries == 3 {
            break tries * 10;
        }
    };
    show("loop + break value", loop_value);

    let over_ten = 'search: {
        for r in readings {
            if r > 10 {
                break 'search r;
            }
        }
        0
    };
    show("labelled block", over_ten);
    println!("  (the for loop added up to {sum}; the while loop counted to {i})");

    println!();
    println!("2. How far each way out jumps");

    let mut kept = Vec::new();
    for r in readings {
        if r % 2 == 1 {
            continue; // to the next pass of this loop
        }
        kept.push(r);
    }
    println!("  continue        next pass          kept the evens: {kept:?}");

    let mut seen = Vec::new();
    for r in readings {
        if r > 6 {
            break; // past the end of this loop
        }
        seen.push(r);
    }
    println!("  break           end of this loop   saw before the first r > 6: {seen:?}");

    let mut pair = None;
    'rows: for a in 1..=4 {
        for b in 1..=4 {
            if a * b == 6 {
                pair = Some((a, b));
                break 'rows; // past the end of the OUTER loop
            }
        }
    }
    println!("  break 'rows     end of outer loop  first a * b == 6: {pair:?}");

    println!("  return          out of the fn      first_even: {:?}", first_even(&readings));
    println!("  ?               out of the fn      total(1, 2, 3): {:?}", total(&["1", "2", "3"]));
    println!("  ?               out of the fn      total(1, x, 3): {:?}", total(&["1", "x", "3"]));
}
