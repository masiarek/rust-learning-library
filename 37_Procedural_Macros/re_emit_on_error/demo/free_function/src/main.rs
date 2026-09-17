use logged::logged_error_only;

#[logged_error_only("warning")] // the level is "warn"
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("{}", area(2, 3));
    println!("{}", area(2, 3) * 2);
    println!("{}", area(2, 3) + 1);
    println!("{}", perimeter(2, 3)); // defined nowhere
}
