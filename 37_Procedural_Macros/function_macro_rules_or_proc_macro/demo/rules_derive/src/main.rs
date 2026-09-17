//! `macro_rules!` used as a derive and as an attribute, so the struct stays
//! where it is declared. `derive()` and `attr()` rules do that, and neither is
//! stable in 1.98.0.

macro_rules! FieldNames {
    derive() (struct $name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        impl $name {
            const FIELD_NAMES: &[&str] = &[$(stringify!($field)),*];
        }
    };
}

macro_rules! with_field_names {
    attr() (struct $name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        struct $name { $($field: $ty),* }

        impl $name {
            const FIELD_NAMES: &[&str] = &[$(stringify!($field)),*];
        }
    };
}

#[derive(FieldNames)]
struct Point {
    x: i32,
    y: i32,
}

#[with_field_names]
struct Size {
    width: u32,
    height: u32,
}

fn main() {
    let (p, s) = (Point { x: 1, y: 2 }, Size { width: 3, height: 4 });
    println!("{:?} {}", Point::FIELD_NAMES, p.x + p.y);
    println!("{:?} {}", Size::FIELD_NAMES, s.width * s.height);
}
