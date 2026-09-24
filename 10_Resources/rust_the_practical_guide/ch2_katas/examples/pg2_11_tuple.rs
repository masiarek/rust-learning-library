// Exercise 2.11: the function takes one argument, a tuple, so the call wraps
// the two numbers in a second pair of parentheses.
fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}

fn main() {
    println!("The distance of the point from the origin is {}", print_distance((5.0, 4.0))); // The distance of the point from the origin is 6.4031243
    println!("the same in f64: {}", (5.0f64.powi(2) + 4.0f64.powi(2)).sqrt()); // the same in f64: 6.4031242374328485
    println!("what std offers for it: {}", 5.0f32.hypot(4.0)); // what std offers for it: 6.4031243
}
