#![allow(deprecated)]

fn main() {
    let text = "alpha\r\nbeta\ngamma\n";

    println!("{:?}", text.lines().collect::<Vec<&str>>());
    println!("{:?}", text.lines_any().collect::<Vec<&str>>());
    println!("identical: {}", text.lines().eq(text.lines_any()));
}
