//! Two clocks: `Instant` answers "how long", `SystemTime` answers "what time".
//!
//! Nothing here prints a reading or a measured duration, because those change
//! on every run. It prints only what is true on every run, and the type each
//! operation hands back, which is where the two clocks differ.
//!
//!   rustc --edition 2024 two_clocks.rs -o /tmp/two_clocks && /tmp/two_clocks

use std::any::type_name_of_val;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    println!("1. Instant: a stopwatch");
    let a = Instant::now();
    let b = Instant::now();
    println!("   Instant::now() <= Instant::now()   {}", a <= b);

    let mut last = Instant::now();
    let mut never_backwards = true;
    for _ in 0..1_000 {
        let next = Instant::now();
        never_backwards &= next >= last;
        last = next;
    }
    println!("   1,000 readings, each >= the last   {never_backwards}");
    println!("   b - a        is a {}", type_name_of_val(&(b - a)));
    println!("   a.elapsed()  is a {}", type_name_of_val(&a.elapsed()));

    println!();
    println!("2. SystemTime: a calendar");
    let now = SystemTime::now();
    println!("   now.duration_since(UNIX_EPOCH).is_ok()   {}", now.duration_since(UNIX_EPOCH).is_ok());
    println!("   now.duration_since(UNIX_EPOCH) is a");
    println!("       {}", type_name_of_val(&now.duration_since(UNIX_EPOCH)));
    println!("   now.elapsed() is a");
    println!("       {}", type_name_of_val(&now.elapsed()));

    println!();
    println!("3. The epoch is a fixed point, so a SystemTime can be built from it");
    let one_day_in = UNIX_EPOCH + Duration::from_secs(86_400);
    println!("   (UNIX_EPOCH + 86_400 s).duration_since(UNIX_EPOCH) = {:?}", one_day_in.duration_since(UNIX_EPOCH));
}
