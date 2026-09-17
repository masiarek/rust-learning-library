use facade::Describe; // `describe_facade`, renamed to `facade` in Cargo.toml

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p.describe());
}
