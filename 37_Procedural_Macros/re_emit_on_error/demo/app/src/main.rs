use logged::logged;

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    #[logged("info")]
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rect { width: 2, height: 3 };
    println!("{}", rect.area());
    println!("{}", rect.area() * 2);
    println!("{}", rect.area() + 1);
}
