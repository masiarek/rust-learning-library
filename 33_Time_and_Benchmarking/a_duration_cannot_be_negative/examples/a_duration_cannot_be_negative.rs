//! A `Duration` cannot be negative: whole seconds plus nanoseconds, unsigned,
//! and one type for every unit.
//!
//!   rustc --edition 2024 a_duration_cannot_be_negative.rs -o /tmp/adn && /tmp/adn

use std::panic;
use std::time::Duration;

fn main() {
    println!("1. One type: whole seconds plus nanoseconds");
    let d = Duration::from_millis(1_999);
    println!("   Duration::from_millis(1_999) = {d:?}");
    println!("   .as_secs()       = {:<10} whole seconds: the fraction is dropped", d.as_secs());
    println!("   .subsec_nanos()  = {:<10} the fraction, on its own", d.subsec_nanos());
    println!("   .as_secs_f64()   = {:<10} an f64", d.as_secs_f64());
    println!("   .as_millis()     = {:<10} a u128", d.as_millis());
    println!("   from_secs(2) == from_millis(2_000)   {}", Duration::from_secs(2) == Duration::from_millis(2_000));

    println!();
    println!("2. {{:?}} picks the unit; there is no Display");
    for d in [
        Duration::from_secs(90),
        Duration::from_millis(1_500),
        Duration::from_micros(250),
        Duration::from_nanos(7),
        Duration::ZERO,
    ] {
        println!("   {d:?}");
    }

    println!();
    println!("3. Why as_millis returns a u128");
    println!("   Duration::MAX             = {:?}", Duration::MAX);
    println!("   Duration::MAX.as_millis() = {}", Duration::MAX.as_millis());
    println!("   u64::MAX                  = {}", u64::MAX);

    println!();
    println!("4. Below zero: ask, clamp, or panic");
    let one = Duration::from_secs(1);
    let two = Duration::from_secs(2);
    println!("   one.checked_sub(two)     = {:?}", one.checked_sub(two));
    println!("   one.saturating_sub(two)  = {:?}", one.saturating_sub(two));
    println!("   one.abs_diff(two)        = {:?}", one.abs_diff(two));

    // Silence the default hook so the message lands on stdout, where the
    // recorded answer key can see it, instead of on stderr.
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let caught = panic::catch_unwind(|| one - two);
    panic::set_hook(hook);
    match caught {
        Ok(d) => println!("   one - two                = {d:?}"),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("?");
            println!("   one - two                = panicked: {msg}");
        }
    }
}
