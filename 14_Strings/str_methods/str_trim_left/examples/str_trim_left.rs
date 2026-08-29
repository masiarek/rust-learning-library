#![allow(deprecated)]

fn main() {
    let s = "  padded  ";
    println!("{:?}", s.trim_left());
    println!("{:?}", s.trim_start());
    println!("identical: {}", s.trim_left() == s.trim_start());
}
