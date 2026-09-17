use describe_shapes::{Meters, Origin, Point, Shape};
use std::fmt::Debug;

/// The shape on one line, then each field on a line of its own.
fn show((shape, fields): (&str, Vec<(&str, &dyn Debug)>)) {
    println!("{shape}");
    for (name, value) in fields {
        println!("   {name} = {value:?}");
    }
}

fn main() {
    show(Point { x: 1, y: 2 }.describe());
    show(Meters(5.5).describe());
    show(Origin.describe());
    show(Shape::Circle { radius: 1.5 }.describe());
    show(Shape::Rect(3, 4).describe());
    show(Shape::Empty.describe());
}
