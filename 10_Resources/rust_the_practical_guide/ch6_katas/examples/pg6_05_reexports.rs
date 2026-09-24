//! Exercise 5, solved: two `pub use` lines in `graphics`, written without the
//! `self::` the book's solution uses, and one `use` line in place of two.
//!
//!   rustc --edition 2024 pg6_05_reexports.rs -o /tmp/pg605 && /tmp/pg605

mod graphics {
    pub use display::show_area;
    pub use shapes::calculate_area;

    pub mod shapes {
        pub fn calculate_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }

    pub mod display {
        pub fn show_area(shape: &str, area: f64) {
            println!("The area of the {} is: {}", shape, area);
        }
    }
}

use graphics::{calculate_area, show_area};

fn main() {
    let radius = 3.0;
    let area = calculate_area(radius);
    show_area("circle", area); // The area of the circle is: 28.274333882308138

    // The long paths still work; the re-exports added names, they moved nothing.
    graphics::display::show_area("same circle", graphics::shapes::calculate_area(radius));
}
