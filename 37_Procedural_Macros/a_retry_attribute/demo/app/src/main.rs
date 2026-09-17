use std::cell::Cell;
use std::num::ParseIntError;
use std::time::Duration;

use retry::retry;

/// Stands in for `std::thread::sleep`: prints the delay it was asked for and
/// returns at once, so this program never waits.
fn record_sleep(delay: Duration) {
    println!("  asked to sleep {delay:?}");
}

/// What a sensor returns on each read: two bad readings, then a number.
const READINGS: [&str; 3] = ["", "n/a", "21"];

#[retry(times = 3, delay_ms = 100, sleep = record_sleep)]
fn read_sensor(reads: &Cell<usize>) -> Result<u32, ParseIntError> {
    let text = READINGS[reads.get()];
    reads.set(reads.get() + 1);
    println!("attempt {}: {text:?}", reads.get());
    let value = text.parse::<u32>()?;
    Ok(value)
}

#[retry(times = 2, delay_ms = 250, sleep = record_sleep)]
fn always_refused(tries: &Cell<u32>) -> Result<(), String> {
    tries.set(tries.get() + 1);
    println!("attempt {}", tries.get());
    Err(format!("refused on attempt {}", tries.get()))
}

fn main() {
    let reads = Cell::new(0);
    println!("= {:?}\n", read_sensor(&reads));

    let tries = Cell::new(0);
    println!("= {:?}", always_refused(&tries));
}
