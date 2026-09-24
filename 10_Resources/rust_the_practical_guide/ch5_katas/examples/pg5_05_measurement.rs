//! Exercise 5 (§5.7): area and perimeter, as a `Result`.
//!
//! The enum and its `impl` are the book's, untouched. The fix is in `main`:
//! each arm needs a pattern, `Ok(res)` and `Err(e)`, before its `=>`.
//!
//!   rustc --edition 2024 pg5_05_measurement.rs -o /tmp/pg505 && /tmp/pg505

enum Measurement {
    CircleArea(f64),
    RectangleArea(f64, f64),
    TriangleArea(f64, f64),
    Perimeter(Vec<f64>),
}

impl Measurement {
    fn calculate(self) -> Result<f64, String> {
        match self {
            Self::CircleArea(radius) => {
                if radius < 0.0 {
                    Err(String::from("Radius cannot be negative"))
                } else {
                    Ok(std::f64::consts::PI * radius * radius)
                }
            }
            Self::RectangleArea(length, width) => {
                if length < 0.0 || width < 0.0 {
                    Err(String::from("Length and width cannot be negative"))
                } else {
                    Ok(length * width)
                }
            }
            Self::TriangleArea(base, height) => {
                if base < 0.0 || height < 0.0 {
                    Err(String::from("Base and height cannot be negative"))
                } else {
                    Ok(0.5 * base * height)
                }
            }
            Self::Perimeter(sides) => {
                if sides.len() < 3 {
                    Err(String::from("A polygon must have at least 3 sides"))
                } else {
                    Ok(sides.iter().sum())
                }
            }
        }
    }
}

/// `calculate(self)` consumes the value, so each `Measurement` is built,
/// handed over, and gone: a `match` on the `Result` is all that is left.
fn report(label: &str, measurement: Measurement) {
    match measurement.calculate() {
        Ok(res) => println!("{label:<28} Result: {res}"),
        Err(e) => println!("{label:<28} Error: {e}"),
    }
}

fn main() {
    let user_input = Measurement::TriangleArea(5.0, 8.0);
    match user_input.calculate() {
        Ok(res) => println!("Result: {res}"),
        Err(e) => println!("Error: {e}"),
    }

    println!();
    report("CircleArea(1.0)", Measurement::CircleArea(1.0));
    report("CircleArea(-1.0)", Measurement::CircleArea(-1.0));
    report("RectangleArea(2.5, 4.0)", Measurement::RectangleArea(2.5, 4.0));
    report("Perimeter([3.0, 4.0])", Measurement::Perimeter(vec![3.0, 4.0]));
    report("Perimeter([3.0, 4.0, 5.0])", Measurement::Perimeter(vec![3.0, 4.0, 5.0]));
}
