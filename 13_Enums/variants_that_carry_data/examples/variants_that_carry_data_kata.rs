//! Kata solution: a struct multiplies, an enum adds.
//!
//!   rustc --edition 2024 variants_that_carry_data_kata.rs -o /tmp/vcd && /tmp/vcd

use std::mem::size_of;

#[derive(Debug)]
enum Shape {
    Point,
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Label(String),
}

// The same four cases modelled the wrong way round: every field always
// present, and four of the five combinations meaningless.
#[allow(dead_code)]
#[derive(Debug)]
struct ShapeStruct {
    kind: u8,
    radius: f64,
    w: f64,
    h: f64,
    label: String,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Point => 0.0,
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rect { w, h } => w * h,
        Shape::Label(name) => name.len() as f64 * 0.0,
    }
}

fn main() {
    println!("ONE TYPE, FOUR SHAPES OF VALUE");
    for s in [Shape::Point, Shape::Circle { radius: 1.0 },
              Shape::Rect { w: 2.0, h: 3.0 }, Shape::Label(String::from("origin"))] {
        println!("  {:<34} area {:.4}", format!("{s:?}"), area(&s));
    }
    println!();

    println!("  the Label arm can read its own String: {:?} is {} chars",
             "origin", "origin".len());
    println!();
    println!("ADDS, RATHER THAN MULTIPLIES");
    println!("  A struct with fields A and B can be any combination of the two,");
    println!("  so its possibilities MULTIPLY. An enum is exactly one variant at");
    println!("  a time, so they ADD. That is what \"algebraic data type\" means,");
    println!("  and it is a modelling decision before it is a memory one:");
    println!("  a Circle has no width, and with the enum there is no field for");
    println!("  one to be wrong in.");
    println!();

    println!("WHAT IT COSTS");
    println!("  size_of::<Shape>()        {:>3} bytes", size_of::<Shape>());
    println!("  size_of::<ShapeStruct>()  {:>3} bytes", size_of::<ShapeStruct>());
    println!("  size_of::<String>()       {:>3} bytes", size_of::<String>());
    println!("  size_of::<f64>()          {:>3} bytes", size_of::<f64>());
    println!();
    println!("  An enum is as big as its LARGEST variant plus a discriminant,");
    println!("  rounded for alignment -- so it costs what the biggest case");
    println!("  needs, not what all the cases need together. The struct pays");
    println!("  for every field on every value, and most of them are unused on");
    println!("  most values.");
    println!();

    println!("AND THE MATCH IS WHERE IT PAYS AGAIN");
    println!("  Each arm destructures its own variant, so `radius` exists in");
    println!("  the Circle arm and nowhere else. With the struct you would read");
    println!("  `s.radius` in a branch chosen by `s.kind`, and nothing checks");
    println!("  that you picked the right field for the right kind.");

    assert_eq!(area(&Shape::Rect { w: 2.0, h: 3.0 }), 6.0);
    assert!(size_of::<Shape>() < size_of::<ShapeStruct>());
}
