use describe_facade::Describe; // the trait and the derive, in one `use`

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p.describe());
}
