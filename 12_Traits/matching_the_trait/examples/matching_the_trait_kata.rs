//! Kata solution: four places an impl disagreed with its trait, four error
//! codes, and the trait left exactly as it was. The code each mistake earned is
//! named beside its fix.
//!
//!   rustc --edition 2024 matching_the_trait_kata.rs -o /tmp/mttk && /tmp/mttk

trait Shape {
    fn area(&self) -> f64;
    fn describe(&self) -> String {
        format!("area {:.1}", self.area())
    }
}

struct Square {
    side: f32,
}

struct Circle {
    radius: f64,
}

struct Strip {
    width: f64,
    height: f64,
}

impl Shape for Square {
    // E0053 — was `-> f32`. The help line changes only the arrow, and the body
    // then fails with E0308, because `self.side * self.side` is still an f32.
    // f32 to f64 loses nothing, so `From` does the widening.
    fn area(&self) -> f64 {
        f64::from(self.side * self.side)
    }
}

impl Shape for Circle {
    // E0449 — was `pub fn`. A trait method is exactly as visible as its trait,
    // so there is nothing to declare.
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Strip {
    // E0053 — was `fn area(self)`. The trait's callers lend the value; an impl
    // cannot demand to be handed it instead.
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // E0050 — was `describe(&self, digits: usize)`. The trait fixes the arity,
    // so the extra parameter moves to an inherent method with its own name.
    fn describe(&self) -> String {
        format!("strip, {}", self.describe_to(2))
    }
}

impl Strip {
    fn describe_to(&self, digits: usize) -> String {
        format!("area {:.digits$}", self.area())
    }
}

fn main() {
    let square = Square { side: 2.0 };
    let circle = Circle { radius: 1.0 };
    let strip = Strip { width: 3.0, height: 0.25 };

    println!("square.describe()    = {}", square.describe());
    println!("circle.describe()    = {}", circle.describe());
    println!("strip.describe()     = {}", strip.describe());
    println!("strip.describe_to(4) = {}", strip.describe_to(4));
}
