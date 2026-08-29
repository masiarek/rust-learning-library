#![allow(deprecated)]

fn main() {
    let s = "  padded  ";
    println!("{:?}", s.trim_right());
    println!("{:?}", s.trim_end());
    println!("identical: {}", s.trim_right() == s.trim_end());
}
