// Exercise 3.3: an assembly line makes 221 cars per hour per unit of speed, and
// some of them fail. `as i32` is what the book prints with; `round()` is what it meant.
fn success_rate(speed: u8) -> f32 {
    match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        _ => 0.77,
    }
}

fn total_production(hours: u8, speed: u8) -> f32 {
    hours as f32 * 221.0 * speed as f32 * success_rate(speed)
}

fn cars_produced_per_minute(_hours: u8, speed: u8) -> f32 {
    221.0 * speed as f32 * success_rate(speed) / 60.0
}

fn main() {
    let total = total_production(6, 5);
    let per_minute = cars_produced_per_minute(6, 5);
    println!("total_production(6, 5) = {total}"); // total_production(6, 5) = 5967
    println!("cars_produced_per_minute(6, 5) = {per_minute}"); // cars_produced_per_minute(6, 5) = 16.575
    println!("the book prints them `as i32`: {} and {}", total as i32, per_minute as i32); // the book prints them `as i32`: 5967 and 16
    println!("rounded, as the comment says: {} and {}", total.round(), per_minute.round()); // rounded, as the comment says: 5967 and 17
    println!("the hours cancel out of the per-minute rate: {} {} {}", cars_produced_per_minute(1, 5), cars_produced_per_minute(6, 5), cars_produced_per_minute(24, 5)); // 16.575 16.575 16.575
    println!("speed 0 is off: {}", total_production(6, 0)); // speed 0 is off: 0
    println!("nothing rejects speed 11: {}", total_production(1, 11)); // nothing rejects speed 11: 1871.87
}
