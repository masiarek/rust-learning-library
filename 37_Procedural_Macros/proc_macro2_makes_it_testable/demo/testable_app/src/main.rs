use testable_field_names::FieldNames;

#[derive(FieldNames)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(FieldNames)]
struct Meters(f64);

fn main() {
    println!("{:?}", Point::FIELD_NAMES); // ["x", "y"]
    println!("{:?}", Meters::FIELD_NAMES); // ["0"]
    let (p, m) = (Point { x: 1, y: 2 }, Meters(0.5));
    println!("{} {} {}", p.x, p.y, m.0); // 1 2 0.5
}
