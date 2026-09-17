use tested_describe::Describe;

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(p.describe(), "Point: x = 1, y = 2");
}
