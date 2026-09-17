#![allow(dead_code)] // the types are described, never built

use syn_shapes::{Shape, shape_of};

#[derive(Shape)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Shape)]
struct Meters(f64);

#[derive(Shape)]
struct Marker;

#[derive(Shape)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

#[derive(Shape)]
union IntOrFloat {
    i: u32,
    f: f32,
}

fn main() {
    println!("Point:      {}", Point::SHAPE);
    println!("Meters:     {}", Meters::SHAPE);
    println!("Marker:     {}", Marker::SHAPE);
    println!("Message:    {}", Message::SHAPE);
    println!("IntOrFloat: {}", IntOrFloat::SHAPE);
    println!("shape_of!:  {}", shape_of!(struct Pair(i32, i32);));
}
