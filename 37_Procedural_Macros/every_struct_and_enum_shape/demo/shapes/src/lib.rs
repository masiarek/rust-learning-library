//! Every shape a struct can have, and an enum with every shape of variant.

use describe_derive::Describe;

#[derive(Describe)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Describe)]
pub struct Meters(pub f64);

#[derive(Describe)]
pub struct Origin;

#[derive(Describe)]
pub enum Shape {
    Circle { radius: f64 },
    Rect(u32, u32),
    Empty,
}
